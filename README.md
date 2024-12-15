# `but`

`but`(broadcast utilities) is a CLI utility for working with [foundry](https://getfoundry.sh) broadcast files.

## Installation

```sh
$ cargo install --git https://github.com/0xdapper/but
```

## Usage

### Generating markdown summary

```sh
$ but to-md --note "Human readable message about what this broadcast did" ./broadcast/path/to/file.json | tee ./summary.md
```
