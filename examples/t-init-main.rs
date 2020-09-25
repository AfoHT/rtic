#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use cortex_m_semihosting::debug;
use panic_semihosting as _;

#[rtic::app(device = lm3s6965)]
mod app {
    #[init]
    fn taskmain(_: taskmain::Context) {
        debug::exit(debug::EXIT_SUCCESS);
    }
}
