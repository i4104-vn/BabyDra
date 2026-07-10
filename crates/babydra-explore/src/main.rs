mod app;
mod ui;

use app::BabyExploreApp;

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let _guard = rt.enter();

    let app = BabyExploreApp::new();
    let exit_code = app.run();
    std::process::exit(exit_code);
}
