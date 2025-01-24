FROM registry.gitlab.steamos.cloud/steamrt/sniper/sdk:latest

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > /tmp/rustup.sh; \
    chmod +x /tmp/rustup.sh; \
    /tmp/rustup.sh -y; \
    bash -l -c "cargo install just";