//! Thin executable for the shared server CLI and supervisor library.
fn main() -> std::process::ExitCode {
    foundry_platform::run_main(foundry_server::main)
}
