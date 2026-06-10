# envparser

[![Crates.io](https://img.shields.io/crates/v/envparser.svg)](https://crates.io/crates/envparser)

A dotenv implementation for Rust. Loads environment variables from a `.env` file and merges them with the actual environment variables provided by the OS.

> Storing [configuration in the environment](http://www.12factor.net/config)
> is one of the tenets of a [twelve-factor app](http://www.12factor.net/).
> Anything that is likely to change between deployment environments–such as
> resource handles for databases or credentials for external services–should
> be extracted from the code into environment variables.

This library is meant to be used in development or testing environments where setting environment variables manually is not practical.

## Usage

Call `envparser::dotenv` when your application starts. It will load environment variables from a `.env` file in the current directory or any of its parents.

For finer control over the file name or location, use `from_filename` or `from_path`.

## Installation

```toml
[dependencies]
envparser = "0.1.1"
```

## Example

A `.env` file looks like this:

```sh
# a comment, will be ignored
REDIS_ADDRESS=localhost:6379
MEANING_OF_LIFE=42
```

```rust
use envparser::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }
}
```

## Variable Substitution

Variables can be reused inside the `.env` file using `$VARIABLE` syntax:

```sh
VAR=one
VAR_2=two

RESULT=$NOPE        # '' (empty string, non-existing)
RESULT=$VAR         # 'one'
RESULT="$VAR"       # 'one' (double quotes don't affect substitution)
RESULT=${VAR}       # 'one'
RESULT=$VAR_2       # 'one_2' (stops at first non-alphanumeric symbol)
RESULT=${VAR_2}     # 'two'
RESULT='$VAR'       # '$VAR' (single quotes escape substitution)
RESULT=\$VAR        # '$VAR' (backslash escapes substitution)
RESULT=$PATH        # contents of the $PATH env var (OS vars always take priority)
```

## CLI Usage

envparser also ships with a CLI to run commands using the environment from a `.env` file:

```bash
envparser <COMMAND> [ARGS]...
envparser -f .env.production cargo run
```

## License

MIT — [github.com/harwarl/envy](https://github.com/harwarl/envy)
