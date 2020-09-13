# trash-cli (trash)

A command line tool written in Rust to interface with the XDG trash.

**Warning**: trash-cli is currently beta level and no guarantees are made about its integrity of managing your files. Additionally, it currently only interacts with the home trash and does not properly handle trashing files if the files are on a different filesystem/partition as your home folder.

Executable is called `trash` and the following subcommands are provided:

Subcommand | Description
-----------|---------------------------------------------------------------------
`empty`    | Empty the trash
`erase`    | Erase the specified files (i.e. `rm`)
`list`     | Recursively list previously trashed files from a specified directory
`prune`    | Delete files from the trash that match a specified regex
`put`      | Trash the specified files
`restore`  | Restore a previously trashed file to its original location

## Installation

### Package managers

[![Packaging status](https://repology.org/badge/vertical-allrepos/trash-cli.svg)](https://repology.org/project/trash-cli/versions)

### Prebuilt binaries:

Prebuilt binaries are provided in the [releases](https://github.com/cjbassi/trash-cli/releases) tab.

### From source:

```bash
cargo install --git https://github.com/cjbassi/trash-cli
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

## Related projects

- sindresorhus
  - [empty-recycle-bin](https://github.com/sindresorhus/empty-recycle-bin)
  - [empty-trash-cli](https://github.com/sindresorhus/empty-trash-cli)
  - [macos-trash](https://github.com/sindresorhus/macos-trash)
  - [recycle-bin](https://github.com/sindresorhus/recycle-bin)
  - [trash-cli](https://github.com/sindresorhus/trash-cli)
- [garbage](https://git.sr.ht/~iptq/garbage)
- [rip](https://github.com/nivekuil/rip)
- [trash-cli](https://github.com/andreafrancia/trash-cli)
