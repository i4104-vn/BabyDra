mod app;
mod ui;

use app::BabyExploreApp;

fn main() {
    let app = BabyExploreApp::new();
    let exit_code = app.run();
    std::process::exit(exit_code);
}
