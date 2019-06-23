# trash-man (trash)

A command line tool to interface with the XDG trash and a Rust port of [trash-cli](https://github.com/andreafrancia/trash-cli).

**Warning**: trash-man is currently beta level. More tests need to be created and it currently only properly handles trashing files if the files are on the same filesystem/partition as your trash can. It has been working for me but no guarantees are made about the integrity of managing your files.

Executable is called `trash` and the following subcommands are provided:

- `empty`: Empty the trash
- `erase`: Erase given files (i.e. `rm`)
- `list`: Recursively list files trashed from the current directory
- `prune`: Erase files from the trash that match a given regex
- `put`: Trash given files
- `recover`: Recover a previously trashed file to its original location

## Installation

### Prebuilt binaries:

Note: (currently only a binary for Linux-x86_64 is available)

Run the following to download the correct binary for your system from the releases tab into `$CARGO_HOME/bin`, courtesy of [japaric/trust](https://github.com/japaric/trust):

```bash
bash <(curl -LSfs https://japaric.github.io/trust/install.sh) \
  -f --git cjbassi/trash-man
```

### From source:

```bash
cargo install --git https://github.com/cjbassi/trash-man
```

## Periodically emptying the trash

Various timers can be setup to periodically empty the trash of files that are older than a given age. This helps reduce the amount of files in the trash can and reduces the amount of disk space it requires.

### systemd

For systemd based Linux distros, a systemd timer file is located [here](./systemd/trash-empty.timer) along with the service file [here](./systemd/trash-empty.service).

To setup the systemd timer:

1. Copy both files to `~/.config/systemd/user/`
2. Customize how often the timer runs (defaults to daily) and how old the files need to be to be deleted (defaults to 30 days)
3. Run `systemctl --user daemon-reload`
4. Run `systemctl --user enable --now trash-empty.timer`
