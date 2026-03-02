# ke

Half make. Per-project command shortcuts stored in a single global file.

Each entry is tied to a folder — `ke` checks your current directory and only runs commands defined for it.

### Setup

`cd` into your project and add commands:

```sh
ke --add build cargo build
ke --add test cargo test --all
```

This creates `~/.ke/commands.yaml` automatically. Then just run:

```sh
ke build
```

Commands are scoped to the folder they were added from — running `ke` in any other folder will error.

Or open the config directly in your default editor:

```sh
ke --edit
```

For multiline commands, edit `~/.ke/commands.yaml` directly:

```yaml
- folder: ~/dev/myproject
  commands:
    check: |
      cargo clippy
      cargo test
```

### Custom config

```sh
ke --config /path/to/commands.yaml build
```

### Install

```sh
cargo install ke
```
