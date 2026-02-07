#![no_std]
#![deny(unused_crate_dependencies)]

pub mod uistate;
pub mod platform;
pub mod eth_transaction;
mod eth_registry_data;
pub mod eth_transaction_screen;
pub mod eth_transaction_viewer;
pub mod error_dialog;
pub mod error;
pub mod welcome_screen;

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

#[macro_use]
extern crate lazy_static;

#[cfg(not(feature="std"))]
extern crate alloc;
#[cfg(not(feature="std"))]
extern crate core;
#[cfg(feature="std")]
extern crate std;

pub mod data_state;
