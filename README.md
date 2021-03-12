# MiniWASM - A minimalist Rust WebAssembly project template

This is a minimal Rust-powered WebAssembly application template. It was
designed to showcase what must be done to get Rust-powered WebAssembly going
with only a small amount of code, while still providing useful tools, such as
integration with `console.log`.

## How to build?

The easiest way is to install Docker on your system and run the
`docker-shell.sh` script. This script builds a Docker image containing Rust,
the wasm32-unknown-unknown target, and a couple of dependencies for optimizing
the generated `.wasm` files.

After building the Docker image the script then launches a container with that
image, giving you a shell with a proper build environment. Alternatively you
can just install Rust and the `wasm32` target on your host machine, and also
install `binaryen` and `wabt`.

Either way, after opening a shell with a build environment, you can just run
the `build.sh` script. This builds the `miniwasm.wasm` file and runs it
through the optimizer to reduce the file size.

Now that you have the compiled WebAssembly file you can run the `serve.sh`
script. This runs a web server in the current working directory (thanks to
Python). You can then go to `http://localhost:8000/` to view the application.
Open the console to see the log messages that the WebAssembly produces.

## Why not wasm-bindgen or wasm-pack?

Wasm-bindgen is awesome, so use it when you can. I myself am working on
projects that have more strict performance requirements, and wasm-bindgen's
translation layer often gets in the way of performance when I need to pass
data back and forth between Rust and JavaScript. Therefore I created MiniWASM
as a template to quickly prototype new experiments for my algorithms.

Another reason why you might want to use MiniWASM as a starting point is that
you want to build something small and don't want to depend on wasm-pack's NPM
packages.

## Compatibility

This application template should be compatible with all modern browsers.
However, it has only been tested with Chrome 88 and Safari 14. It will
probably work fine in Firefox too though.

## Technical details

This project consists of two components:

1. A Rust file serving as the entry point for the WebAssembly application.
2. An HTML file with embedded JavaScript code to bootstrap the WebAssembly
   application.

### The Rust WebAssembly application

The Rust WebAssembly application is a single-file crate that performs a few
functions:

1. It sets up `wee_alloc` as the memory allocator.

2. It has a bridge to send log messages to JavaScript. This is done by
   importing some proxy functions (one for `console.log` and one for
   `console.error`) and calling them with the address and length of a `&str`.

   The JavaScript implementation of these functions then looks into the
   WebAssembly application's `WebAssembly.Memory` instance and extracts the
   characters, converting the raw bytes back into a JavaScript-representation
   before handing them off to the JavaScript console functions.

3. It defines a struct that holds the application's global state and stores
   this in a cell in thread local storage.

4. It defines functions that act as a bridge between the WebAssembly module's
   external interface and the application struct.

5. It has a bootstrapping function that sets up a new panic handler so that we
   can see panics in the JavaScript console.

### JavaScript bootstrapping code

Instantiating a WebAssembly module is easy. We just have to fetch the `.wasm`
file and pass its contents into `WebAssembly.instantiate`.

To get logging to work we need to do a little bit more though. When
instantiating the WebAssembly module we provide an import descriptor that
exposes `console.log` and `console.error`. We can't expose those functions
directly though, because WebAssembly's calling convention only allows us to
use primitive types as arguments and return values. Therefore we wrap the
logging functions with a function that takes the location of a string slice
and extracts the bytes from the WebAssembly application's `Memory` instance.
These bytes are converted into a JavaScript string and then handed off to the
logging function.

## License

Copyright 2021 Emil Loer.

This project is licensed under the MIT license. A copy of the license can be
found in the project repository.
