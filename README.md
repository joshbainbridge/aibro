# AiBro

AiBro is your own little coding bro, immersed in the world of AI, crypto, and
all other over hyped tech trends. As a CLI tool you can pipe it code and then
ask it to make changes, add documentation, etc.

## Requirements

This calls the [OpenAI API](https://platform.openai.com/docs/api-reference)
HTTP interface and requires the following enviroment variables:

- `OPENAI_API_KEY`: API key for authentication. Create an OpenAI account and add
a payment method [here](https://platform.openai.com/account/api-keys).

## Installation

Either setup a rust toolchain manually, or if you are using macOS and have Nix
installed, use the provided Nix flake to create a development environment:

```bash
nix develop
```

Installation of the CLI command can then be done using cargo:

```bash
cargo install --path .
```

## Usage

Print about information using:

```bash
aibro --help
```

As an example, input can be provided using a string of arguments passed to the
command, stdin over a pipe, or both:

```bash
aibro write me a ray trace function in cpp > ray_trace.cpp
```

```bash
cat ray_trace.cpp | aibro add doxygen style comments to code
```
