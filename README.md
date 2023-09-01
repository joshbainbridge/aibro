# AiBro

This is your own little coding bro, immersed in the world of AI, crypto, and all
other types of over hyped tech trends. You can pipe it code and request changes,
add documentation, etc. This can be useful when combined with editors that
support piping text like [Helix](https://helix-editor.com).

[![asciicast](https://asciinema.org/a/605870.svg)](https://asciinema.org/a/605870)

AiBro uses the [OpenAI API](https://platform.openai.com/docs/api-reference)
HTTP interface to answere queries. You will need to setup an OpenAI account and
acquire an authentication key before getting a responce.

## Environment variables:

- `OPENAI_API_KEY`: API key for authentication. Alternatively this can be set or
overriden using a command argument. See the [usage](#usage) section below.
- `AIBRO_DEFAULT_PROMPT`: Optional default fallback prompt for common requests
when piping in code. For example 'return code with documentation'.

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

````
Usage: aibro [OPTIONS] [PROMPT]...

Arguments:
  [PROMPT]...
          Input prompt [override: $AIBRO_DEFAULT_PROMPT]

Options:
  -b, --bro <BRO>
          Selected aibro persona
          
          [default: coder]

          Possible values:
          - coder:   Helpful coding assistant
          - chad:    Over hyped Chad GPT bro
          - grandma: Old lady grandma bro

  -m, --model <MODEL>
          Selected ML model
          
          [default: gpt3]

          Possible values:
          - gpt3: GPT 3.5 turbo model
          - gpt4: GPT 4.0 model

  -t, --temperature <TEMPERATURE>
          Model temperature
          
          [default: 0.3]

  -a, --auth <AUTH>
          Authentication key [override: $OPENAI_API_KEY]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
````
