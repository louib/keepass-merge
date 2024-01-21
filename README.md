# keepass-merge
[![Build Status](https://github.com/louib/keepass-merge/actions/workflows/merge.yml/badge.svg?branch=main)](https://github.com/louib/keepass-merge/actions/workflows/merge.yml)
[![dependency status](https://deps.rs/repo/github/louib/keepass-merge/status.svg)](https://deps.rs/repo/github/louib/keepass-merge)
[![License file](https://img.shields.io/github/license/louib/keepass-merge)](https://github.com/louib/keepass-merge/blob/main/LICENSE)

> **Warning**   
>
> This repo is a work-in-progress and is not ready for general use.
> It relies on a [unmerged PR](https://github.com/sseemayer/keepass-rs/pull/201) in `keepass-rs`,
> and will not reach version 1.0.0 before that PR is merged.

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
  -n, --no-password
          Do not use a password to decrypt the destination database
  -s, --same-credentials
          Use the same credentials for both databases
  -d, --dry-run
          Do not save the resulting database
      --slot <SLOT>
          The slot number of the yubikey to decrypt the destination database
      --serial-number <SERIAL_NUMBER>
          The serial number of the yubikey to decrypt the destination database
      --slot-from <SLOT_FROM>
          The slot number of the yubikey to decrypt the source database
      --serial-number-from <SERIAL_NUMBER_FROM>
          The serial number of the yubikey to decrypt the source database
      --no-password-from
          Do not use a password to decrypt the source database
  -f, --force
          Force saving the database even if warnings were generated
  -h, --help
          Print help
  -V, --version
          Print version
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
