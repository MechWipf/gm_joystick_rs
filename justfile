set dotenv-load
set dotenv-required

podman := "podman run --rm -v .:/workspace -w /workspace steam-sniper-rust:latest bash -l -c"

_default:
    @just --list

all: build deploy

build-dev-image:
    podman build -f dev.containerfile -t steam-sniper-rust:latest .

build:
    {{podman}} "cargo build --release; \
    clang joystick_module/src/lib.cc \
        -Ijoystick_module/include \
        -Itarget/cxxbridge \
        -fPIC \
        -shared \
        -o target/release/gmcl_joystick_linux64.dll \
        target/release/libjoystick_module.a \
        /usr/lib/x86_64-linux-gnu/libSDL2.so"

deploy:
    cp -v \
        ~/src/gm_joystick_rs/target/release/gmcl_joystick_linux64.dll \
        $DEPLOY_FOLDER