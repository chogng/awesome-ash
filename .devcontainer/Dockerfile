FROM node:24.21.0-trixie

RUN apt-get update && apt-get install --yes --no-install-recommends \
    build-essential \
    bubblewrap \
    ca-certificates \
    cmake \
    curl \
    gcc-x86-64-linux-gnu \
    git \
    gnupg \
    iproute2 \
    iptables \
    libasound2-dev \
    libcap-dev \
    libc6-dev-amd64-cross \
    libdbus-1-dev \
    libgtk-3-dev \
    libssl-dev \
    libwayland-dev \
    libx11-dev \
    libxkbcommon-dev \
    libxkbfile-dev \
    nasm \
    pkg-config \
    python3 \
    python3-venv \
    slirp4netns \
    sudo \
    && rm -rf /var/lib/apt/lists/*

RUN curl --fail --silent --show-error --location https://apt.llvm.org/llvm-snapshot.gpg.key --output /tmp/ash-llvm-key.asc \
    && test "$(gpg --show-keys --with-colons /tmp/ash-llvm-key.asc | awk -F: '$1 == "fpr" { print $10; exit }')" = 6084F3CF814B57C1CF12EFD515CF4D18AF4F7421 \
    && install -m 0644 /tmp/ash-llvm-key.asc /etc/apt/keyrings/llvm.asc \
    && printf '%s\n' 'deb [signed-by=/etc/apt/keyrings/llvm.asc] https://apt.llvm.org/trixie/ llvm-toolchain-trixie-21 main' > /etc/apt/sources.list.d/llvm-21.list \
    && apt-get update \
    && apt-get install --yes --no-install-recommends clang-21 libclang-21-dev llvm-21 \
    && rm -f /tmp/ash-llvm-key.asc \
    && rm -rf /var/lib/apt/lists/*

COPY rust-toolchain.toml /tmp/ash-rust-toolchain.toml
COPY .just-version /tmp/ash-just-version

RUN printf '%s\n' 'node ALL=(root) NOPASSWD: /usr/bin/chown' > /etc/sudoers.d/ash-node \
    && chmod 0440 /etc/sudoers.d/ash-node \
    && mkdir -p /workspaces/ash \
    && chown node:node /workspaces/ash

ENV CARGO_HOME=/home/node/.cargo \
    RUSTUP_HOME=/home/node/.rustup \
    PATH=/home/node/.cargo/bin:${PATH}

USER node

RUN curl --fail --silent --show-error --location https://sh.rustup.rs --output /tmp/ash-rustup-init.sh \
    && sh /tmp/ash-rustup-init.sh -y --profile minimal --default-toolchain none \
    && rm /tmp/ash-rustup-init.sh \
    && rustup toolchain install "$(python3 -c 'import tomllib; print(tomllib.load(open("/tmp/ash-rust-toolchain.toml", "rb"))["toolchain"]["channel"])')" --profile minimal --component clippy --component rustfmt

RUN cargo install --locked --jobs 2 --version "$(cat /tmp/ash-just-version)" just

ENV CXX=clang++-21 \
    LIBCLANG_PATH=/usr/lib/llvm-21/lib \
    ASH_LLVM_BIN=/usr/lib/llvm-21/bin \
    SHELL=/bin/bash \
    LANG=C.UTF-8 \
    LC_ALL=C.UTF-8 \
    PATH=${PATH}:/workspaces/ash/scripts/.venv/bin

WORKDIR /workspaces/ash

# The desktop entrypoint prepares the system socket as root; VS Code terminals use remoteUser node.
USER root
RUN apt-get update && apt-get install --yes --no-install-recommends \
    at-spi2-core \
    dbus-x11 \
    fluxbox \
    novnc \
    ripgrep \
    tigervnc-standalone-server \
    tigervnc-tools \
    websockify \
    x11-utils \
    xauth \
    zenity \
    && rm -rf /var/lib/apt/lists/*

COPY --chmod=0755 .devcontainer/desktop-init.sh /usr/local/bin/ash-desktop-init
COPY --from=ghcr.io/astral-sh/uv:0.11.3 /uv /usr/local/bin/uv
COPY scripts/pyproject.toml /tmp/ash-python-project.toml
RUN test "$(uv --version | cut -d ' ' -f 2)" = "$(python3 -c 'import tomllib; print(tomllib.load(open("/tmp/ash-python-project.toml", "rb"))["tool"]["uv"]["required-version"].removeprefix("=="))')"
ENV DISPLAY=:1 \
    XAUTHORITY=/home/node/.Xauthority
ENTRYPOINT ["/usr/local/bin/ash-desktop-init"]
CMD ["sleep", "infinity"]
