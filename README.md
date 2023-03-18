# keepass-merge
[![Build Status](https://github.com/louib/keepass-merge/actions/workflows/merge.yml/badge.svg?branch=main)](https://github.com/louib/keepass-merge/actions/workflows/merge.yml)
[![dependency status](https://deps.rs/repo/github/louib/keepass-merge/status.svg)](https://deps.rs/repo/github/louib/keepass-merge)
[![License file](https://img.shields.io/github/license/louib/keepass-merge)](https://github.com/louib/keepass-merge/blob/main/LICENSE)

CLI tool to merge KDBX (keepass) databases

`keepass-merge` is based on the [`keepass-rs` library](https://github.com/sseemayer/keepass-rs).

## Usage
```
CLI tool to merge KDBX (keepass) databases

Usage: keepass-merge [OPTIONS] <DESTINATION_DB> <SOURCE_DB>

Arguments:
  <DESTINATION_DB>  The path of the database file to merge to
  <SOURCE_DB>       The path of the database file to merge from

Options:
  -n, --no-prompt         Disables the password prompt on stdout
  -s, --same-credentials  Use the same credentials for both databases
  -d, --dry-run           Do not save the resulting database
  -h, --help              Print help
  -V, --version           Print version
```

## Installing
### With Nix
Assuming that you have enabled both the `flakes` and `nix-command` experimental features:
```
nix profile install github:louib/keepass-merge
```

### With Cargo
```
cargo install --path .
```
