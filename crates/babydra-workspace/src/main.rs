mod cli;

fn main() {
    babydra_core::services::logger::init_logger("babydra-workspace", "babydra-workspace.log");
    cli::run_cli();
}
