# Nix and NixOS

The repository is a flake (`flake.nix`, and the game's packages in `nix/`). It builds Foundry Client from source with nixpkgs'
`rustPlatform` and holds none of Mojang's files: the game files still come from `foundry-prepare` and your own
copy of Minecraft 26.3, as everywhere else ([hosting.md](hosting.md), step 2).

| Package | What | Systems |
|---|---|---|
| `foundry-server` | `foundry-server`, with `foundry-engine` (the game without sound or controllers) beside it | Linux, macOS (Apple Silicon) |
| `foundry-engine` | The server build of the game alone | Linux, macOS |
| `foundry-prepare` | `foundry-prepare`, with a JDK 25 for the exporters it compiles | Linux, macOS |
| `foundry-client` | The game and `foundry-ctl` | Linux, macOS |
| `foundry-launcher` | The launcher in an FHS environment (it builds native mods with Cargo) | Linux |

```sh
nix run github:foundry-rust/foundry#foundry-server -- --version
nix profile install github:foundry-rust/foundry#foundry-server
```

`overlays.default` adds them to `pkgs`; on Linux, `nix develop` gives the build environment of the workspace.

## Preparing the game files

`foundry-prepare` works in a writable folder (its pinned hashes, the exporters' sources, scratch files). The
Nix wrapper copies one to `$FOUNDRY_PREPARE_ROOT`, by default `~/.local/share/foundry-nix/prepare`, unless you
pass `--root`. Write the files where you want them with `--output`:

```sh
foundry-prepare --prepare --server-jar ~/Downloads/server.jar --output ~/foundry/26.3
```

## A server on NixOS: `services.foundry-server`

```nix
{
  inputs.foundry.url = "github:foundry-rust/foundry";

  outputs = { nixpkgs, foundry, ... }: {
    nixosConfigurations.box = nixpkgs.lib.nixosSystem {
      system = "x86_64-linux";
      modules = [
        foundry.nixosModules.foundry-server
        {
          services.foundry-server = {
            enable = true;
            eula = true;                      # you agree to https://aka.ms/MinecraftEULA
            openFirewall = true;              # port 25565 by default
            resources = "/srv/foundry/26.3";  # the folder foundry-prepare wrote, copied here
            init = { world = "survival"; players = [ "Alex" ]; op = "Alex"; };
            settings = { "world.difficulty" = "hard"; "server.max_players" = 10; };
          };
        }
      ];
    };
  };
}
```

- The server folder is `/var/lib/foundry-server` (`dataDir`), owned by the `foundry-server` user. The first
  start writes `server.toml` from `init`, as `foundry-server init` would. Every start then sets `port`,
  `resources` and `settings` with `foundry-server config set`, which checks them and keeps the rest of the
  file: the settings you don't name stay yours to edit.
- Give paths as strings, not Nix paths: a Nix path would copy the game files into the world-readable store.
- The service runs `foundry-server run` and stops it with SIGTERM, which saves the world first. Manage it as
  that user: `sudo -u foundry-server foundry-server --dir /var/lib/foundry-server status` (or `cmd`, `logs`,
  `backup`...).
- Nix decides the release: the update check is off (`FOUNDRY_NO_UPDATE_CHECK=1`) and `foundry-server update`
  refuses, since this isn't an install.sh install.
- The unit is hardened (read-only system, no home folders, no capabilities, its own `/tmp` and devices). A
  `resources` folder under `/home` isn't visible to it: keep it elsewhere, such as `/srv`.

### Preparing on the server

Instead of `resources`, `prepare` runs `foundry-prepare` before the server starts, from files you put on the
machine yourself: the 26.3 `server.jar` and a copy of the official launcher's game folder with 26.3 played
once (`versions/26.3`, `libraries`, `assets`), readable by the `foundry-server` user.

```nix
services.foundry-server.prepare = {
  enable = true;
  serverJar = "/srv/foundry/server.jar";
  minecraftDir = "/srv/foundry/minecraft";
};
```

It writes `/var/lib/foundry-server/game-files`, and runs again only when the `foundry-prepare` package or
those paths change. The `foundry-server-prepare` unit has no network at all: nothing is ever downloaded.
