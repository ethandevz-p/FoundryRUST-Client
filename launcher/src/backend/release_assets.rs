/// The release assets of each system Foundry Client ships for: (OS, architecture, the game, the launcher).
/// game-build.yml builds the game's, nightly-build.yml signs and publishes them, and launcher.yml
/// builds the launcher's (`platform_assets_are_published`).
pub const PLATFORMS: [(&str, &str, &str, &str); 5] = [
    (
        "linux",
        "x86_64",
        "foundry-linux-x86_64.tar.gz",
        "foundry-launcher-linux-x86_64.tar.gz",
    ),
    (
        "macos",
        "aarch64",
        "foundry-macos-arm64.zip",
        "foundry-launcher-macos-arm64.zip",
    ),
    // Intel Macs (arismoko/foundry#1161).
    (
        "macos",
        "x86_64",
        "foundry-macos-x64.zip",
        "foundry-launcher-macos-x64.zip",
    ),
    (
        "windows",
        "x86_64",
        "foundry-windows-x64.zip",
        "foundry-launcher-windows-x64.zip",
    ),
    (
        "windows",
        "aarch64",
        "foundry-windows-arm64.zip",
        "foundry-launcher-windows-arm64.zip",
    ),
];
