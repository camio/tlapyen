# TLAPYEN — The Last Program You'll Ever Need

A Rust proc macro that calls [Claude Code](https://claude.ai/claude-code) at
compile time to generate Rust code from natural language descriptions.

Your builds now require WiFi, determinism is a distant memory, and you've
officially outsourced programming to an AI running inside `rustc`.

## Installation

```toml
[dependencies]
tlapyen = { path = "../tlapyen" }  # or from crates.io if we ever get that desperate
```

You'll also need the Claude CLI:

```sh
npm install -g @anthropic-ai/claude-code
```

Yes, your Rust project now depends on npm. We're sorry. Actually, no we're not.

## Usage

```rust
use tlapyen::tlapyen;

tlapyen!("Make a dog type with a name and age. Give it a method called \
          'something_interesting' that returns a string with a fun, \
          creative fact about the dog based on its name and age.");

fn main() {
    let dog = Dog {
        name: "Rover".to_string(),
        age: 5,
    };
    println!("{}", dog.something_interesting());
    // Prints: ...something? Every `cargo clean && cargo build` is a surprise.
}
```

## How It Works

1. You write a natural language description in `tlapyen!("...")`
2. At compile time, the proc macro shells out to the `claude` CLI
3. Claude generates Rust code
4. The macro parses it and splices it into your program
5. You pray

Results are cached in `target/tlapyen-cache/` so subsequent builds don't
re-invoke Claude. Run `cargo clean` to wipe the cache and live dangerously.

## FAQ

**Is this a good idea?**

Absolutely not.

**Is this production-ready?**

Define "production."

**What about determinism?**

lol

**What if Claude generates bad code?**

You'll get a compile error with Claude's raw output so you can refine your
prompt. It's like pair programming, except your partner is a stochastic parrot
running inside your build system.

**What if I don't have internet?**

Then you can't compile. Welcome to the future.

**Does this work with `cargo check`?**

Yes! It will dutifully call Claude just to check your types. Every. Single.
Time. (Unless cached.)

## Known Limitations

- **Determinism**: Each build might produce different code. Or not. Who knows.
- **Build times**: Now depend on Claude's API latency and mood.
- **Reproducibility**: "It compiled on my machine" has a whole new meaning.
- **Offline builds**: A contradiction in terms.

## License

MIT — because even chaotic software deserves a permissive license.
