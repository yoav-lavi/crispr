# CRISPR 🧬

`crispr` is a CLI tool allowing to scaffold a project from a template with a `.crispr.{toml,json}` configuration file.

The template uses tokens that need to be replaced per scaffolded project (e.g. `{{REPO_NAME}}`), which are set in the configuration file as either user replaceable or with predetermined values.

`crispr` reads the configuration, asks the user for any needed values and makes the replacements as needed, showing a diff in the process.

`crispr` respects `.gitignore` files and only changes files that should be committed.

The `.crispr.{toml,json}` configuration file itself is automatically ignored when replacing tokens.

![usage](https://github.com/yoav-lavi/crispr/raw/main/usage.png)

## Name

Named after the [CRISPR-cas9](https://wikipedia.org/wiki/CRISPR_gene_editing) genetic engineering technique used for targeted gene editing

## Install 

### Homebrew

```sh
brew install yoav-lavi/tap/crispr
```

### Binary

Binaries can be downloaded from the [releases page](https://github.com/yoav-lavi/crispr/releases)

## Usage

```sh
crispr [FLAGS] [PATH]
```

### Arguments

- `<PATH>`    The path to run `crispr` (`'.'` by default)

### Flags

- `-c, --config`     The path to an alternative configuration file (`'.crispr.json'` by default)

- `-d, --dry`        Dry run - prints output without making changes

- `-h, --help`       Prints help information

- `-V, --version`    Prints version information

## Configuration File

`crispr` uses a TOML or JSON configuration file detailing the tokens to be replaced.

In case both file types are found, the priority is as follows:
- `.crispr.toml`
- `.crispr.json`

### Fields

- `replacements` - a map (`HashMap<String, String>`) of replacement tokens to values
- `user_replacements` - an array (`Vec<String>`) of replacements for which the user will be asked to supply a value

### Example

- `.crispr.toml`

```toml
userReplacements = [
    "{{REPO_NAME}}"
]

[replacements]
"{{YEAR}}" = "2021"
```

- `.crispr.json`

```json
{
  "replacements": {
    "{{YEAR}}": "2021"
  },
  "userReplacements": ["{{REPO_NAME}}"]
}
```

## Limitations

- `crispr` reads files line-by-line, so a token broken into multiple lines (e.g. by formatting) will not be replaced

## Prior Art

- `crispr` takes some inspiration and ideas from [Ruplacer](https://github.com/TankerHQ/ruplacer) but does not intend to replace (pun may be intended) Ruplacer as the use case and goal are different.

## Acknowledgements

`crispr` uses the following dependencies:

- [`clap`](https://github.com/clap-rs/clap) (Apache 2.0)
- [`difference`](https://github.com/johannhof/difference.rs) (MIT)
- [`colored`](https://github.com/mackwic/colored) (MPL)
- [`ignore`](https://github.com/BurntSushi/ripgrep/tree/master/crates/ignore) (MIT)
- [`serde`](https://github.com/serde-rs/serde) (Apache 2.0)
- [`serde_json`](https://github.com/serde-rs/json) (Apache 2.0)
- [`toml`](https://github.com/alexcrichton/toml-rs) (MIT)