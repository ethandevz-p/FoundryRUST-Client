"""The one Cargo selection the harnesses build their fast-profile programs with.

Cargo settles the features of shared dependencies over every package a command selects, so foundry-client built
under two selections is compiled twice, and the parity prebuild (tools/parity.py, prebuild_commands) makes one Cargo
run per selection. Every harness that builds foundry-client, foundry-ctl, foundry-server, foundry-official or
foundry-relay in the fast profile goes through `command`, naming only the programs it runs. Examples stay in Cargo
runs of their own: building one turns on foundry-client-lib's dev-dependencies, which compile test-only code into the
game.
"""

PACKAGES = ('foundry-game', 'foundry-client-lib', 'foundry-server', 'foundry-official', 'foundry-relay')


def command(*programs):
    """`cargo build` of these programs in the fast profile, with the shared selection."""
    return ['cargo', 'build', '--locked', '--profile', 'fast', *(part for package in PACKAGES for part in ('-p', package)),
            *(part for program in programs for part in ('--bin', program))]
