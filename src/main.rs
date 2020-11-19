#![recursion_limit = "512"]
use log::Level;
use web_logger::Config;

mod app;
mod components;
mod pages;
mod switch;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is the entry point for the web app
pub fn main() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    web_logger::custom_init(Config { level: Level::Info });
    yew::start_app::<app::AppModel>();
}
