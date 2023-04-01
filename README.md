# 🇨🇭 pocketknife (`pk`)
The AI powered CLI multitool.

Use ChatGPT 4 on the command line to create, refactor or process text-based files.

## Use cases
- Create code or text files from prompts
- Convert text-based files to other formats (e.g. JSON to YAML)
- Refactor code in place
- Create scripts to build your own tools!
- Rewrite text to sound smarter
- Or just query ChatGPT 4 directly

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
$ pk "Convert from JSON to YAML" test.json > test.yaml
$ cat test.yaml
birds: 22
dogs:
  - owls: lie
```
Or you want to capitalize all the keys and modify the file in place!
```bash
$ pk "Capitalize everything!" test.json -i
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
`pk` is an amazingly powerful multitool at your fingertips.

## Installation
`pk` is written in Rust. 

1. Install Rust from https://www.rust-lang.org/ if you don't have it already
2. Clone this repo locally
3. Grab your OpenAI API key and set the `OPENAI_API_KEY` environment variable with it
4. Run `cargo build --release` to compile the release binary to `target/release/pk`
5. Move the binary to wherever you like and add it to your `PATH`
5. Run the binary using the instructions below

## Usage

### 1. Query ChatGPT 4 directly
Simply send a prompt from the command line to get a response sent to stdout.

#### Command:
`pk PROMPT`

#### Example:
```bash
$ pk "UNIX command for listing files"
ls
```
### 2. Create code or text files
Prompt ChatGPT to create the contents of a file.

#### Command:
`pk PROMPT > DEST_FILE`

#### Example:
```bash
$ pk "basic HTML file skeleton" > index.html
$ cat index.html
<!DOCTYPE html>
<html>
  <head>
  </head>
  <body>
  </body>
</html>
```

### 3. Process a file and output results to stdout
Have ChatGPT process a file based on a prompt and send the results to stdout.

#### Command:
`pk PROMPT FILE`

#### Example:
```bash
$ pk "Convert YAML to JSON" test.yaml 
{
  "birds": 22,
  "dogs": [
    {
      "owl": "lies"
    },
    {
      "owl": "ninja"
    }
  ]
}
```

### 4. Process a file and output results to a file
Have ChatGPT process a file based on a prompt and send the results to another file.

#### Command:
`pk PROMPT SRC_FILE > DEST_FILE`

#### Example:
```bash
$ pk "Convert YAML to JSON" test.yaml > test.json
```

### 5. Refactor a file in place
Have ChatGPT process a file based on a prompt and overwrite the file with the result.

**WARNING: Use this flag at your own risk! This is not reversible! Make sure you have another copy of the file or its current state is backed up in version control if you're concerned about losing its contents!**

#### Command:
`pk PROMPT FILE -i`

#### Example:
```bash
$ pk "Convert all periods to dollar signs" test.txt -i
```

### 6. Create a script for a common prompt
If you have a prompt you find useful but hate retyping, build a script!

#### Example script `yaml2json.sh`:
```bash
#!/bin/bash

# Check if FILE parameter is provided
if [ -z "$1" ]; then
  echo "Please provide a FILE parameter"
  exit 1
fi

# Use the FILE parameter with the pk command
pk "Convert YAML to JSON" "$1" 
```
Then use it and send stdout to another file:
```bash
$ ./yaml2json.sh sample.yaml > sample.json
```

## Troubleshooting
If you're getting an error...
* Make sure you set your OpenAI API key to the `OPENAI_API_KEY` environment variable
* `pk` uses GPT 4, make sure your account has access to it (or get on the waitlist!)