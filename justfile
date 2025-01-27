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
    cd target/release; \
    clang -shared -lSDL2 -o libjoystick_module.so \$(find build -name '*.o') libjoystick_module.a"

deploy:
    cp -v \
        ~/src/gm_joystick_rs/target/release/libjoystick_module.so \
        $DEPLOY_FOLDER/gmcl_joystick_linux64.dll