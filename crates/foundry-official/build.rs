//! Gives the Windows executable Foundry Client's icon and its stack, as foundry-client's
//! (`packaging/windows/exe.rs`).
#[path = "../../packaging/windows/exe.rs"]
mod windows_exe;

fn main() {
    windows_exe::resources("foundry-official");
}
