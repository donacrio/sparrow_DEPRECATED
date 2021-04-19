# Sparrow

[![Tests](https://github.com/DonaCrio/sparrow/actions/workflows/tests.yaml/badge.svg)](https://github.com/DonaCrio/sparrow/actions/workflows/tests.yaml)
[![Releases](https://github.com/DonaCrio/sparrow/actions/workflows/releases.yaml/badge.svg)](https://github.com/DonaCrio/sparrow/actions/workflows/releases.yaml)

Sparrow is a lightweight in-memory database built in Rust.

## Motivation

This is a long run personal project that I use to learn various aspects of Software Development:
- Low-level programing concepts
- Networking programing concepts
- Rust programming language
- Managing a repository
- Handling Continuous Integration & release process
- Creating good documentation

If you take a deep look at the code you might notice that I sometimes use bare-metal Rust and sometimes higher level crates. This mostly due to the learning process I follow:
1. Read documentation on a new programming concept. *e.g.: How to program a TCP socket interface based on an event system and using Unix `epoll`?*
2. Implement a working solution from scratch, which is obviously not optimized . *e.g.: Use Rust low-level `mio` crate to implement a custom socket event polling system.*
3. Refactor the previous system using an optimize and efficient crate. *e.g: Use Rust `hyper` crate to easily create a low-level http server and abstract socket and asynchronous operations*.
4. Repeat `1.` and learn new stuff :D

## Usage

### Creating a `.env` file

Before building any module of Sparrow, you must know that a `.env` file containing the needed environment variables is required at the root of the project. A default file `default.env` is provided as an example configuration. You can copy it in a `.env` file before building.
### Building binaries for your local machine

In order to build Sparrow's binaries locally you'll need to install Rust and Cargo. You can follow this [instruction](https://doc.rust-lang.org/cargo/getting-started/index.html).


After getting a clean install on your machine, clone the repository and use Cargo to compile Sparrow:
```bash
git clone git@github.com:DonaCrio/sparrow.git
cargo build --release
```

You will find executable binaries under the `./target/release/` folder.

> NB: Note that the release flag will compile optimize binaries. If you want to build non-optimized binaries, just remove this flag. Binaries will then be located under `./target/debug`.

### Building Docker images

A generic dockerfile is provided to create Docker images for every Sparrow module. To build an image you can run the following:
```bash
docker build -t <image_name> --build-arg MODULE_NAME=<module_name> .
```
For instance if you want to build the CLI:
```bash
docker build -t sparrow-cli --build-arg MODULE_NAME=sparrow-cli .
```

The built image is Docker `SCRATCH` image containing the optimized binary built with cargo.

### Running Sparrow

You can run Sparrow modules by running the binaries or docker images. It's possible to override the configuration from the `.env` file by passing options. Please refer to the help display with the `-h` flag:
```bash
<binary_name> -h
```
or with docker:
```bash
docker run -it --rm <module_name> -h
```
