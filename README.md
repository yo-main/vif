# ZEUS

Blam, welcome to Zeus repo, the language that will take over python.

Is zeus a final name ? I am really not sure, but for now I'll stick with it.
Maybe a more adapted name will show itself as I'm progressing in making that language a real thing.

## Usages

### How to build

```bash
cargo build --release
```

### How to open a zeus shell

```bash
./target/release/zeus-cli
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
cp ./target/release/zeus-cli ./target/release/faster
cargo build --release
hyperfine -w 10 -r 100 './target/release/zeus-cli ./snippets/benchmark.zs' './target/release/faster ./snippets/benchmark.zs'
```

