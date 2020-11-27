#![recursion_limit = "1024"]
mod app;
mod components;
mod pages;
mod slides_data;
mod switch;

#[cfg(not(debug_assertions))]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is the entry point for the web app
pub fn main() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    yew::start_app::<app::AppModel>();
}
