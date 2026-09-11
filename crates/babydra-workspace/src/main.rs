fn main() {
    babydra_core::services::logger::init_logger("babydra-workspace", "babydra-workspace.log");
    babydra_core::services::workspace::cli::run_cli();
}
