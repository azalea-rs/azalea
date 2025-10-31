# Contributing to Azalea

Thank you for your interest in contributing to Azalea! We're building a collection of Rust crates for making Minecraft bots, clients, and tools, and we appreciate all contributions that help make Azalea better.

## Getting Started

The easiest way to get started is by joining our community in the [Matrix space](https://matrix.to/#/#azalea:matdoes.dev) (recommended) or [Discord server](https://discord.gg/FaRey6ytmC). They're bridged, so you don't need to join both.

### How to Contribute

There are several ways you can contribute to Azalea:

- **Reporting Bugs**: If you encounter a bug, please search for existing issues first. If you can't find a duplicate, open a new issue with a clear description, steps to reproduce, and any relevant logs or code snippets.
- **Suggesting Features**: Have ideas for new features or improvements? Open an issue describing your proposal, its benefits, and potential implementation considerations.
- **Submitting Pull Requests**: Fork the repository, make your changes, and create a pull request. Ensure your code follows our guidelines below.
- **Improving Documentation**: Help us improve documentation, fix typos, or add examples.
- **Creating Plugins**: Build and share Bevy plugins that extend Azalea's functionality.

## Development Setup

### Code Quality Tools

Before submitting a pull request, please ensure:

- **Formatting**: Run `cargo fmt` to format your code
- **Linting**: Run `cargo clippy --all-targets --fix` and address all warnings
- **Spell Checking**: Run [typos](https://github.com/crate-ci/typos) to catch spelling mistakes
- **Dependency Sorting**: Run `cargo sort` to keep Cargo.toml dependencies organized
- **Tests**: Run `cargo test` to ensure all tests pass

You can install the necessary tools with:
```bash
cargo install cargo-sort typos-cli
```
or
```bash
cargo binstall cargo-sort typos-cli
```

## Coding Guidelines

### Pull Request Requirements

- **Title**: Use a concise, informative title that clearly communicates the PR's purpose
- **Description**: Provide a comprehensive description explaining:
  - What was changed and why
  - The impact of the changes
  - Any known issues or limitations
  - Related issues or discussions
- **Code quality**: All code quality tools must be runned, and errors or warnings what they give must fixed

### Best Practices

- **Writing Tests**: Add unit tests for new features and bug fixes. Refer to the [Rust book on testing](https://doc.rust-lang.org/book/ch11-01-writing-tests.html) for guidance.
- **Benchmarking**: For performance-sensitive changes, consider adding benchmarks using Criterion. See their [quickstart guide](https://github.com/bheisler/criterion.rs#quickstart).
- **Clear Commit Messages**: Write clear, concise commit messages that describe your changes.
- **Documentation**: Update relevant documentation when adding new features or changing behavior.
- **Async Best Practices**: 
  - Use async/await appropriately for I/O operations
  - Avoid blocking the async runtime with CPU-intensive tasks
  - Use appropriate synchronization primitives (`Arc`, `Mutex`, channels)

## Minecraft Version Support

We currently support Minecraft **1.21.10**. When contributing protocol-related changes, ensure they're compatible with this version. We prioritize supporting the latest Minecraft version over maintaining multiple version compatibility.

## Plugin Development

If you're creating plugins for Azalea:

- Follow the Bevy plugin conventions
- Ensure your plugin works with Azalea's ECS architecture
- Consider submitting your plugin to be listed in our README
- Document any breaking changes between versions

## FAQ

### How do I disable console messages?
Set the `RUST_LOG` environment variable:
- Disable all: `RUST_LOG=off`
- Filter specific modules: `RUST_LOG=azalea::pathfinder=off`

See the [env_logger documentation](https://docs.rs/env_logger) for more information.

### My PR is failing CI checks. What should I do?
- Run `cargo fmt` and `cargo clippy` locally first
- Ensure all tests pass with `cargo test`
- Check for typos with `typos`
- Verify dependency sorting with `cargo sort`

## Getting Help

- **Community**: Join our [Matrix](https://matrix.to/#/#azalea:matdoes.dev) or [Discord](https://discord.gg/FaRey6ytmC) for real-time help
- **Documentation**: Check [docs.rs/azalea](https://docs.rs/azalea) (stable) or [azalea.matdoes.dev](https://azalea.matdoes.dev) (unstable)
- **Issues**: Use GitHub issues for bug reports and feature requests

## Before Submitting Large Changes

For significant contributions:
- Open an issue first to discuss your approach
- Consider breaking large changes into smaller, focused PRs
- Ensure your changes align with Azalea's [goals](#goals) and don't conflict with our [non-goals](#non-goals)

## Recognition

We maintain a list of [real-world bots using Azalea](https://github.com/azalea-rs/azalea#real-world-bots-using-azalea) in our README. If you've built something with Azalea, consider submitting a PR to add your project!

Thank you for helping make Azalea better! 🚀