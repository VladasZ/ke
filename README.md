# ke

Half make. Per-project command shortcuts stored in a single global file.

### Setup

`cd` into your project and add commands:

```sh
ke --add build cargo build
ke --add test cargo test --all
```

This creates `~/.ke/commands.yaml` automatically. Run with:

```sh
ke build
```

Commands are scoped to the folder they were added from — running `ke` in any other folder will error.

### Global commands

Global commands work from any folder:

```sh
ke --add-global hi echo Hello
```

```yaml
- global:
    hi: echo Hello
```

Folder commands take priority over global ones if the same name exists in both.

### Multiline commands

Edit `~/.ke/commands.yaml` directly:

```yaml
- folder: ~/dev/myproject
  commands:
    check: |
      cargo clippy
      cargo test
```

### Edit config

```sh
ke --edit
```

### Custom config

```sh
ke --config /path/to/commands.yaml build
```

### Install

```sh
cargo install ke
```
