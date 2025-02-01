Joystick-Module for Linux
---

Garry's Mod joystick module for Linux using SDL2.

Inspired by:
    - https://github.com/MattJeanes/Joystick-Module
    - https://gitlab.h08.us/puff/joystick-module-linux

---

## How to build

### With Just

Requirements: Rust, Just, Podman (docker needs editing the justfile)

- Run to create the build image:
    `just build-dev-image`
- Run to build the module:
    `just build`
- Copy `target/release/libjoystick_module.so` to `(your gmod folder)/garrysmod/lua/bin/gmcl_joystick_linux64.dll`

### With container

- Install docker (or podman)
- Run to build a steam sniper SDK with Rust installed:
    `docker build -f dev.containerfile -t steam-sniper-rust:latest .`
- Run container:
    `docker run --rm -v .:/workspace -w /workspace steam-sniper-rust:latest bash -l`
- Execute the build script in the container
    ```sh
    cargo build --release
    pushd target/release
    clang -shared -lSDL2 -o libjoystick_module.so \$(find build -name '*.o') libjoystick_module.a
    popd
    ```
- Copy `target/release/libjoystick_module.so` to `(your gmod folder)/garrysmod/lua/bin/gmcl_joystick_linux64.dll`

---

## Why

For me this is a learn project to use Rust for Gmod module development. This project would probably be much smaller and simpler with just C++, but that was not the point.  
Also, the libevdev version by Puff Machine does no longer work when Gmod is run via steams soldier runtime, since it does not include libevdev.