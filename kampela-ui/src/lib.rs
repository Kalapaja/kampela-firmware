#![no_std]
#![deny(unused_crate_dependencies)]

pub mod uistate;
pub mod platform;
pub mod eth_transaction;
mod eth_registry_data;
pub mod eth_transaction_screen;
pub mod eth_transaction_viewer;
pub mod text_screen;
pub mod error;
pub mod welcome_screen;
pub mod test_message_screen;

pub mod widget{
    pub mod view;
    pub mod nav_bar{
        pub mod nav_bar;
        pub mod nav_button;
    }
}

pub mod display_def;
pub mod pin{
    pub mod pin;
    pub mod pindots;
    pub mod pinpad;
    pub mod pinbutton;
}

pub mod qr;
pub mod message;

#[cfg(not(feature="std"))]
extern crate alloc;
#[cfg(not(feature="std"))]
extern crate core;
#[cfg(feature="std")]
extern crate std;

pub mod data_state;
