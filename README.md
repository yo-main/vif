# VIF

Blam, say hi to Vif, the language that will take over python.

## Usages

### How to build

```bash
cargo build --release
```

### How to open a vif shell

```bash
./target/release/vif-cli
```

### Open documentation

```bash
mdbook serve --open ./docs
```

### Run test suites

```bash
cargo test --workspace
```

### Benchmarking

```bash
# if not done already
cp ./target/release/vif-cli ./target/release/faster
cargo build --release
hyperfine -w 10 -r 100 './target/release/vif-cli ./snippets/benchmark.zs' './target/release/faster ./snippets/benchmark.zs'
```

