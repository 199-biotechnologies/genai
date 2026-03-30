# Contributing to GenAI

Thanks for your interest in contributing. Here is how to get started.

## Setup

```bash
git clone https://github.com/199-biotechnologies/genai
cd genai
cargo build
```

You need Rust 2024 edition (1.85+). Install it from [rustup.rs](https://rustup.rs).

## Adding a Model

1. Add the `ModelInfo` entry in `src/models.rs`
2. If the model needs a new provider, create a file in `src/providers/`
3. Run `cargo check` to verify it compiles
4. Submit a PR with the model name and a brief description of why it belongs

## Adding a Provider

1. Create `src/providers/your_provider.rs`
2. Implement the `generate_image` and/or `generate_video` functions
3. Register it in `src/providers/mod.rs`
4. Add the API key environment variable to the README

## Code Style

- Run `cargo fmt` before committing
- Run `cargo clippy` and fix any warnings
- Keep functions short. If a function does two things, split it

## Pull Requests

- One feature per PR
- Write a clear description of what changed and why
- If you add a dependency, explain why it is needed

## Bug Reports

Open an issue with:
- What you expected
- What happened instead
- The command you ran
- Your OS and Rust version

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
