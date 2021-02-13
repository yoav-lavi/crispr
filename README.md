# CRISPR 🧬

`crispr` is a CLI tool allowing to scaffold a project from a template with a `.crispr.json` configuration file.

The template uses tokens that need to be replaced per scaffolded project (e.g. `{{REPO_NAME}}`), which are set in the configuration file as either user replaceable or with predetermined values.

`crispr` reads the configuration, asks the user for any needed values and makes the replacements as needed, showing a diff in the process.

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
`crispr` uses a JSON configuration file detailing the tokens to be replaced.

### Fields
- `replacements` - a map (`HashMap<String, String>`) of replacement tokens to values
- `user_replacements` - an array (`Vec<String>`) of replacements for which the user will be asked to supply a value

### Example

```json
{
  "replacements": {
    "{{YEAR}}": "2021"
  },
  "user_replacements": ["{{REPO_NAME}}"]
}
```

### Empty Configuration

```json
{
  "replacements": {},
  "user_replacements": []
}
```

## Prior Art
- `crispr` takes some inspiration and ideas from [Ruplacer](https://github.com/TankerHQ/ruplacer) but does not intend to replace (pun may be intended) Ruplacer as the use case and goal are different.


## Acknowledgements

`crispr` uses the following dependencies:
- (`clap`)[https://github.com/clap-rs/clap] (Apache 2.0)
- (`difference`)[https://github.com/johannhof/difference.rs] (MIT)
- (`colored`)[https://github.com/mackwic/colored] (MPL)
- (`ignore`)[https://github.com/BurntSushi/ripgrep/tree/master/crates/ignore] (Unlicenced)
- (`serde`)[https://github.com/serde-rs/serde] (Apache 2.0)
- (`serde_json`)[https://github.com/serde-rs/json] (Apache 2.0)