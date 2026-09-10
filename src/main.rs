#[cfg(feature = "server")]
mod backend;
#[allow(clippy::all, warnings)]
mod components;
#[cfg(feature = "server")]
mod db;
mod eval;
mod ui;
mod user;

fn main() {
    #[cfg(not(feature = "server"))]
    dioxus::launch(crate::ui::app);
    #[cfg(feature = "server")]
    dioxus::serve(crate::backend::router);
}
