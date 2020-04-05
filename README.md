# qesp
[![Build Status](https://travis-ci.com/carrascomj/qesp.svg?branch=master)](https://travis-ci.com/carrascomj/qesp)
[![Build Status](https://img.shields.io/crates/v/qesp.svg)](https://crates.io/crates/qesp/)  

Rust binary crate to remove characters of all files in a directory. It supports recursive
walking through the tree.

Check the tiny [APÎ documentation](https://docs.rs/qesp/0.1.0/qesp) for some
details about the implementation.

## Installation
You can build it with [cargo](https://doc.rust-lang.org/cargo/) from
[crates.io](https://crates.io/crates/qesp/).

```shell
cargo install qesp
```

#### Installing from source
Just clone the repository and install it with cargo. For instance:

```shell
git clone https://github.com/carrascomj/qesp.git
cargo install --path qesp
```

#### But I don't want this cargo stuff...
Then download the perl script [qesp](https://github.com/carrascomj/qesp/blob/master/benchmarks/qesp)
and put in under your PATH. For instance:

```shell
wget https://raw.githubusercontent.com/carrascomj/qesp/master/benchmarks/qesp
mv qesp ~/.local/bin
```

## Usage
    qesp 0.2.1
    Remove annoying characters of names in a directory

    USAGE:
        qesp [FLAGS] [dir]

    FLAGS:
        -h, --help         Prints help information
        -r, --recursive    recursively attempts to rename whole directory tree
        -V, --version      Prints version information

    OPTIONS:
        -p, --pattern <pattern>    annoying characters to be removed [default: [ ()]]

    ARGS:
        <dir>    Target directory [default: .]

## "Annoying characters"
Space and parenthesis.
