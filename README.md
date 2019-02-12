# trash-man (trash)

A command line tool to interface with the XDG trash and a Rust port of [trash-cli](https://github.com/andreafrancia/trash-cli).

Executable is called `trash` and the following subcommands are provided:

- `empty`: Empty the trash
- `erase`: Erase given files (i.e. `rm`)
- `list`: Recursively list files trashed from the current directory
- `prune`: Erase files from the trash that match a given regex
- `put`: Trash given files
- `restore`: Restore a previously trashed file to its original location

## Installation

### Prebuilt binaries:

Run the following to download the correct binary for your system from the releases tab into `$CARGO_HOME/bin`: (currently only Linux-x86_64 is available)

```
bash <(curl https://raw.githubusercontent.com/japaric/trust/c268696ab9f054e1092f195dddeead2420c04261/install.sh) -f --git cjbassi/trash-man
```

### From source:

```
cargo install --git https://github.com/cjbassi/trash-man
```
