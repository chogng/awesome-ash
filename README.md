# Ash

A workspace for code and AI agents.

Ash Code is the terminal interface for working with AI agents. It uses the shared
Rust App Server for sessions, tools, execution and persisted state. The desktop
App is a native Rust window ready for frontend development.

[User guide](https://github.com/chogng/ash-docs) · [Development](docs/README.md)

## Run from source

Prepare Rust, Just and the backend build tools using the [build guide](docs/build.md).

```sh
just build-ash  # Build the native desktop executable
just ash        # Open the native desktop window
just ash-code   # Run Ash Code in the terminal
```

The native window opens on macOS, Windows and Linux with X11. Cargo builds it
from `app/Cargo.toml`. The desktop workbench and Web frontend are scheduled for
a later phase.

With Ash Code installed, run `ash` for interactive sessions, `ash ask` for a
question, or `ash exec` for a task.

## License

Ash's original code and materials are proprietary and all rights reserved.
See [LICENSE](LICENSE). Third-party components are governed by their respective
licenses and [notices](THIRD_PARTY_NOTICES.md).
