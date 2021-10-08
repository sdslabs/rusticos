# Rustic OS
> Rustic OS written completely in Rust

[![Kernel CI](https://github.com/sdslabs/rusticos/actions/workflows/kernel.yaml/badge.svg)](https://github.com/sdslabs/rusticos/actions/workflows/kernel.yaml)

Rustic OS is our attempt at developing a modular kernel written completely in Rust with an attempt to make it as usable for daily driver needs as possible.

## Contents

* [Dependencies](#dependencies)
* [Development](#development)
* [Contributing](#contributing)
* [Contact](#contact)

## Dependencies

The dependencies required for compiling and running Rustic OS are [Rust](https://www.rust-lang.org/) and [QEMU](https://www.qemu.org/). Their installation guides are linked below:

* [Rust](https://www.rust-lang.org/tools/install)
* [QEMU](https://www.qemu.org/download/)

## Development

Open your favourite terminal and perform the following tasks:-

1. Clone this repository.

    ```bash
    $ git clone https://github.com/sdslabs/rusticos
    ```

1. Go inside the cloned directory and list available *makefile* commands.

    ```bash
    $ cd rusticos && make help

    RusticOS, Lightweight OS implementation in Rust

    install        Install toolchain dependencies
    fmt            Format codebase using cargo fmt
    kernel_build   Build kernel image
    kernel_test    Run kernel tests
    kernel_run     Attach QEMU and run kernel

    Do check out the code at https://github.com/sdslabs/rusticos
    ```

1. Run `make install` to install the necessary toolchain dependencies and change Rust version to nightly-2020-12-07.

    > This is necessary to enable some of the dependency crates

    ```bash
    $ make install
    ```

1. Compile the kernel crate and build the kernel binary and link with the bootloader.

    ```bash
    $ make kernel_build
    ```

1. Attach QEMU as the runner and boot up the kernel.

    ```bash
    $ make kernel_run
    ```

## Contributing

If you'd like to contribute to this project, refer to the [contributing documentation](./CONTRIBUTING.md).

## Contact

If you have a query regarding the product or just want to say hello then feel free to visit
[chat.sdslabs.co](http://chat.sdslabs.co/) or drop a mail at [contact@sdslabs.co.in](mailto:contact@sdslabs.co.in)

Created with :heart: by [SDSLabs](https://github.com/sdslabs)