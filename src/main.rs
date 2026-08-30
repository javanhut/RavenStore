mod backend;
mod catalog;
mod config;
mod ui;

fn main() -> glib::ExitCode {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    // `raven-store --updates` opens straight on the Updates page; Settings
    // uses it for its "Install updates" button.
    let mut start_page = "discover";
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--updates" => start_page = "updates",
            "--installed" => start_page = "installed",
            "--page" => {
                if let Some(p) = args.next() {
                    start_page = Box::leak(p.into_boxed_str());
                }
            }
            "--version" | "-V" => {
                println!("raven-store {}", env!("CARGO_PKG_VERSION"));
                return glib::ExitCode::SUCCESS;
            }
            "--help" | "-h" => {
                println!("raven-store [--updates | --installed | --page <id>]");
                return glib::ExitCode::SUCCESS;
            }
            other => tracing::warn!("ignoring unknown argument {other}"),
        }
    }
    ui::run(start_page)
}
