#[cfg(feature = "server")]
mod backend;
#[cfg(feature = "server")]
mod db;
mod eval;
mod ui;
mod user;

fn main() {
    #[cfg(not(feature = "server"))]
    {
        dioxus::fullstack::set_server_url("http://178.250.240.28");
        dioxus::launch(crate::ui::app);
    }
    #[cfg(feature = "server")]
    dioxus::serve(crate::backend::router);
}
