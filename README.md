# 🤏 tweak
Process local files using ChatGPT 4.

ChatGPT 4 is an incredibly powerful tool for processing data. `tweak` is a CLI tool to use ChatGPT 4 to execute a prompt against the contents of a local file to do things like generate results or refactor a file in place (using the `-i` flag).

## How it works
Imagine you have a `test.json` file...
```json
{
  "birds": 22,
  "dogs": [
    {
      "owl": "lies!"
    }
  ]
}
```
...and you want to convert it to YAML...
```bash
$ tweak test.json "Convert from JSON to YAML" > test.yaml
$ cat test.yaml
birds: 22
dogs:
  - owls: lie
```
Or you want to capitalize all the keys and modify the file in place!
```bash
$ tweak -i test.json "Capitalize everything!"
$ cat test.json
{
  "BIRDS": 22,
  "DOGS": [
    {
      "OWL": "LIES!"
    }
  ]
}
```
`tweak` lets you quickly process local files with ChatGPT!

## Installation
`tweak` is written in Rust. 

1. Install Rust from https://www.rust-lang.org/ if you don't have it already
2. Clone this repo locally
3. Grab your OpenAI API key and set the `OPENAI_API_KEY` environment variable with it
4. Run `cargo build --release` to compile the release binary to `target/release/tweak`
5. Move the binary to wherever you like and add it to your `PATH`
5. Run the binary using the instructions below

## Usage
### 1. Output results to stdout
Have ChatGPT process a file based on a prompt and send the results to stdout.

#### Command:
`tweak FILE PROMPT`

#### Example:
```bash
tweak test.yaml "Convert YAML to JSON"
```

### 2. Rewrite a file in place
Have ChatGPT process a file based on a prompt and overwrite the file with the result.

**WARNING: Use this flag at your own risk! This is not reversible! Make sure you have another copy of the file or its current state is backed up in version control if you're concerned about losing its contents!**

#### Command:
`tweak -i FILE PROMPT`

#### Example:
```bash
tweak -i test.txt "Convert all periods to dollar signs"
```

## Troubleshooting
If you're getting an error...
* Make sure you set the `OPENAI_API_KEY` in your `PATH`
* `tweak` uses GPT-4, make sure you have access to it or get on the waitlist