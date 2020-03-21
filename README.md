# qesp
Perl script to remove characters of all files in a directory. It supports recursive
walking through the tree.


## Installation
Just download the script [qesp](https://github.com/carrascomj/qesp/blob/master/qesp)
put in under your `PATH`. For instance:

```shell
wget https://raw.githubusercontent.com/carrascomj/qesp/master/qesp
mv qesp ~/.local/bin
```

## Usage
    Script to remove annoying characters of names in a directory.
    qesp [target_dir] [-r | --recursive]
        - target_dir: default '.';
        - -r, --recursive: recursively attempts to rename whole directory tree;
        - -h, --help: prints this usage text and exit.\n

## "Annoying characters"
Space and parenthesis.
