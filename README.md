# qesp
[![Build Status](https://travis-ci.com/carrascomj/ripkmer.svg?branch=master)](https://travis-ci.com/carrascomj/ripkmer)  
Rust binary crate to remove characters of all files in a directory. It supports recursive
walking through the tree.


## Installation
You can build it with cargo.
Just clone the repository and install it with cargo. For instance:

```shell
git clone https://github.com/carrascomj/qesp.git
cargo install --path qesp
```

### But I don't want this cargo stuff...
Then download the perl script [qesp](https://github.com/carrascomj/qesp/blob/master/benchmarks/qesp)
and put in under your PATH. For instance:

```shell
wget https://raw.githubusercontent.com/carrascomj/qesp/master/benchmarks/qesp
mv qesp ~/.local/bin
```

## Usage
    CLI tool to remove annoying characters of names in a directory.
    qesp [target_dir] [-r | --recursive]
        - target_dir: default '.';
        - -r, --recursive: recursively attempts to rename whole directory tree;
        - -h, --help: prints this usage text and exit.\n

## "Annoying characters"
Space and parenthesis.
