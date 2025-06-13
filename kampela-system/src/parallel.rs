//! Asynchronous operation generic code
use core::array;

use efm32pg23_fix::SYST;

/// Default (non-blocking) delay for operation start
pub const DELAY: u32 = 1; //1ms

pub struct Timer {
    count: u32,
    last_clock_value: u32,
}

impl Timer {
    /// New timer starts counting from initiation, timer set in ms
    pub fn new(count: u32) -> Self {
        Self {
            count: (count * SYST::get_ticks_per_10ms()).div_ceil(10),
            last_clock_value: SYST::get_current(),
        }
    }
    /// Returns true if timer is on
    pub fn tick(&mut self) -> bool {
        if self.count == 0 {
            return false
        };

        let current = SYST::get_current();
        let diff = if current < self.last_clock_value {
            self.last_clock_value - current
        } else {
            SYST::get_ticks_per_10ms() + self.last_clock_value - current //manage reload, should work if tick < 10ms
        };
        self.last_clock_value = current;

        if self.count <= diff {
            self.count = 0;
            false
        } else {
            self.count -= diff;
            true
        }
    }
}
/// Do things with delay before or after the task
pub enum WithDelay<A> {
    Do(A),
    Wait(Timer),
}

/// Pool of threads that take turn when advance_states() called
/// 
/// StateEnum must implement default state that returns error or if expected value to cancel calls of advance() in function above 
pub struct Threads<StateEnum, const CAPACITY: usize> where
    StateEnum: Default,
{
    threads_pool: [StateEnum; CAPACITY],
    active: usize,
    index: usize,
    repeat: bool,
}

impl<StateEnum, const CAPACITY: usize> Threads<StateEnum, CAPACITY> where
    StateEnum: Default {
    pub fn new(initial: StateEnum) -> Self {
        let mut threads_pool: [StateEnum; CAPACITY] = array::from_fn(|_| StateEnum::default());
        threads_pool[0] = initial;
        Self {
            threads_pool,
            active: 1,
            index: 0,
            repeat: true
        }
    }

    pub fn from<const N: usize>(states: [StateEnum; N]) -> Self {
        let mut threads_pool: [StateEnum; CAPACITY] = array::from_fn(|_| StateEnum::default());
        for (i, state) in states.into_iter().enumerate() {
            threads_pool[i] = state
        }

        Self {
            threads_pool,
            active: !usize::MAX.overflowing_shl(N as u32).0,
            index: 0,
            repeat: true,
        }
    }

    pub fn turn(&mut self) -> &mut StateEnum {
        if self.active == 0 {  // return default
            self.index = 0;
            self.repeat = false;
            self.threads_pool.get_mut(0).expect("threads pool shouldn't be zero sized")
        } else {
            if self.repeat {
                self.repeat = false;
            } else {
                if self.index >= self.active_len() - 1 {
                    self.index = 0;
                } else {
                    self.index += 1;
                };
            }
            self.threads_pool.get_mut(self.index).expect("index overflow should be checked")
        }
    }

    /// Starting new thread for state
    pub fn wind(&mut self, state: StateEnum) {
        let l = self.active_len();

        if l >= self.threads_pool.len() {
            panic!("there is not enough thread slots in the pool {} requested but {} available", l + 1, self.threads_pool.len());
        } else {
            self.threads_pool[l] = state;
            self.active |= 1 << l;
        }
    }

    /// Change current thread state
    /// Be aware that thread won't turn right after the change
    /// If change in default acts like wind()
    pub fn change(&mut self, state: StateEnum) {
        self.hold();
        self.switch(state);
    }

    /// Schedule to switch current thread state
    /// If switch in default acts like wind()
    pub fn switch(&mut self, state: StateEnum) {
        if self.active & (1 << self.index) != 0 {
            self.threads_pool[self.index] = state;
        } else {
            self.wind(state);
        }
    }

    /// Mark end state of current thread
    fn sync_any(&mut self, index: usize) {
        if self.active & (1 << index) != 0 {
            self.threads_pool[index] = StateEnum::default();

            for i in index..self.active_len() - 1 {
                self.threads_pool.swap(i, i +1);
            }
            let masked = self.active & !usize::MAX.overflowing_shl(index as u32).0;
            self.active = masked | self.active.overflowing_shr(1).0 & usize::MAX.overflowing_shl(index as u32).0;
            
            // to preserve order
            if index == self.index {
                self.repeat = true;
            }
            if self.index >= self.active_len() {
                self.index = 0;
            }
        }
    }
    pub fn sync(&mut self) {
        self.sync_any(self.index);
    }

    pub fn hold(&mut self) {
        self.repeat = true;
    }
    /// Ends mathced thread, make sure thread is safe to terminate, true if success, false if not found
    pub fn try_terminate_any<F: Fn (&StateEnum) -> bool>(&mut self, condition: F) -> bool {
        let mut index_to_terminate = None;
        for (index, thread) in self.threads_pool.iter().enumerate() {
            if condition(thread) {
                index_to_terminate = Some(index);
            }
        }
        if let Some(i) = index_to_terminate {
            self.sync_any(i);
        }
        true
    }

    /// Uses closure on every running thread, make sure the thread is safe to change from outside, match thread first
    pub fn try_change_any<F: FnMut(&mut StateEnum)>(&mut self, mut closure: F) -> bool {
        for thread in self.threads_pool.iter_mut() {
            closure(thread)
        }
        true
    }

    /// Returns true until all matched treads are running
    /// Possible to stuck if two or more threads are waiting each other
    pub fn is_all_running(&self, conditions: &[fn(&StateEnum) -> bool]) -> bool {
        'awaiting: for condition in conditions {
            for thread in self.threads_pool.iter() {
                if condition(thread) {
                    continue 'awaiting
                }
            }
            return false
        }
        true
    }

    /// Returns true until any other thread is running
    /// Possible to stuck if two or more threads are waiting each other
    pub fn is_other_running(&self) -> bool {
        self.active & !(1 << self.index) != 0 
    }
    /// Returns true until any thread is running
    pub fn is_any_running(&self) -> bool {
        self.active != 0
    }

    fn active_len(&self) -> usize {
        if self.active == 0 {
            0
        } else {
            self.active.ilog2() as usize + 1
        }
    }
}

/// Asynchronous procedures should implement this.
///
/// To call, iterate over advance()
pub trait AsyncOperation {
    type Init;
    type Input<'a>;
    type Output;

    fn new(data: Self::Init) -> Self;

    /// Call this repeatedly to progress through operation
    fn advance<'a>(&mut self, data: Self::Input<'a>) -> Self::Output;
}

