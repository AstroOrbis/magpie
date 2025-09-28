[![CI status](https://github.com/LimeEng/magpie/actions/workflows/ci.yaml/badge.svg)](https://github.com/LimeEng/magpie/actions/workflows/ci.yaml)
[![Latest version](https://img.shields.io/crates/v/magpie?color=blue)](https://crates.io/crates/magpie)

# Magpie

<img src="https://cdn.github.emileng.se/repo/magpie/logo.svg" width="200" alt="Magpie logo" align="right">

Magpie is a high-performance library for the classic board game [Othello](https://en.wikipedia.org/wiki/Reversi). It provides both a user-friendly API and a low-level interface suitable for AI engines.

### Key Features

- **Built with bitboards**: Uses bitboards for extremely fast board operations
- **Zero dependencies**: Core functionality has no external dependencies
- **Optional Serde support**: Serialization available through an optional feature flag

Furthermore, the library offers two abstraction levels:

- **Game API**: Ensures rule compliance, tracks turns, and maintains board consistency
- **Board API**: Provides raw board operations without validation, when performance is critical.

## Installation

```sh
cargo add magpie
# If serialization with Serde is desired, activate the serde feature flag.
cargo add magpie -F serde
```

## Examples

Examples are [described here](/examples).

Curious to play? One example features a functional Othello game with a random AI opponent. Run `cargo run --example human_vs_ai` to start a game!

## Benchmarks

Benchmarks are [described here](/benches)

Simply run `cargo bench` to run all benchmarks.

## Tests

Run `cargo test -r -Fserde` to run native/prop tests (or `cargo nextest r -r -Fserde` if you use nextest).

Overall, we do 3 kinds of testing; native, proptest, and Kani.
Kani is somewhat like proptest - except it uses formal verification instead of random testing, letting it detect (given enough time) *every possible edge case*.
Due to the nature of Kani, however, you need to do a few extra things to run those tests!

Install it [here](https://model-checking.github.io/kani/install-guide.html) (hint: on NixOS, [this](https://github.com/AstroOrbis/nur) might be of use :wink:).

<details>
  <summary>The Kani command is pretty large, so click me to see it all!</summary>
This command should:

- Run all Kani tests throughout the entire crate & tests directory

- Print a unit test for values that fail, so you can quickly reproduce the error 

- Not print too much unnecessary output


```sh
cargo kani --tests -Fserde -Zconcrete-playback --output-format=terse
```

Add the -j flag to use multiple threads - although it makes the output a bit harder to read.
</details>

Thankfully, since we're bitboard-based, it doesn't need to deal with proofs of Rust's internal array functions.

Godspeed!
