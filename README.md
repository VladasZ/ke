# ke

Half make. Per-project command shortcuts stored in a single global file.

Each entry is tied to a folder — `ke` checks your current directory and only runs commands defined for it.

### Setup

Create `~/.ke/commands.yaml`:

```yaml
- folder: ~/dev/myproject
  commands:
    build: cargo build
    test: |
      cargo test
      cargo clippy
```

`cd` into that folder and run:

```sh
ke build
```

Running `ke` outside a folder that has an entry will error — commands are scoped to the folder they're defined for.

### Custom config

```sh
ke --config /path/to/commands.yaml build
```

### Install

```sh
cargo install ke
```
