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
        if let Some(host) = option_env!("WEBCALC_HOST") {
            dioxus::fullstack::set_server_url(host);
        }
        dioxus::launch(crate::ui::app);
    }
    #[cfg(feature = "server")]
    dioxus::serve(crate::backend::router);
}
