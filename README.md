# 🇨🇭 pocketknife (`pk`)
The `pk` command is an AI powered multitool that leverages OpenAI's [GPT-4](https://openai.com/product/gpt-4).

## Sample Use Cases
Come up with the right CLI command to do something, e.g.:
```bash
pk "unix command to find all files with the word 'bird' in it"
```
Create code or text files from prompts, e.g.: 
```bash
pk "html file skeleton" > index.html
pk "js file to start an express server on port 8080" > server.js
pk "css file that animates buttons fading out on press" > bye-bye-button.css
```
Convert text-based files to other formats, e.g.: 
```bash
pk "json to yaml" test.json > test.yaml
pk "rust to zig" src/main.rs > main.z
pk "csv to json" results.csv > results.json
pk "mocha to jest" mocha-test.js > jest.test.js
```
Refactor code in place, e.g.: 
```bash
pk "collapse to one line" test.json -i
pk "add jsdoc comments" complex-code.js -i
```
Rewrite text to sound smarter, e.g.: 
```bash
pk "use more impressive vocabulary" commencement-speech.txt
```
Query GPT-4 4 directly, e.g.: 
```bash
pk "what does aux do to ps?"
pk "write a song about writing a README"
```
Create scripts to build your own tools (see use case example below).

*Or do something else I haven't thought of yet!*

## A more in-depth example...
Imagine you have a JSON file you want to convert to YAML. You could download a special tool for it. Or you can use `pk`!

Let's say we have a `test.json` file like so:
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
We can use `pk` to convert it to YAML
```bash
$ pk "json to yaml" test.json > test.yaml
$ cat test.yaml
birds: 22
dogs:
  - owls: lies!
```
Or maybe you just want to capitalize all the strings in the JSON file in place for some weird reason?
```bash
$ pk "capitalize everything" test.json -i
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
The point is, `pk` is an amazingly powerful multitool that you can use for almost anything you can think of in regard to processing text.

## Usage Patterns

### 1. Query GPT-4 directly
Simply send a prompt from the command line to get a response sent to stdout.

#### Command:
`pk PROMPT`

#### Example:
```bash
$ pk "unix command for finding files with the word 'bird' in them"
grep -r "bird" .
```
### 2. Create code or text files
Prompt GPT-4 to create the contents of a file.

#### Command:
`pk PROMPT > DEST_FILE`

#### Example:
```bash
$ pk "html file skeleton" > index.html
$ cat index.html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Document</title>
</head>
<body>

</body>
</html>
```

### 3. Process a file and output results to stdout
Have GPT-4 process a file based on a prompt and send the results to stdout.

#### Command:
`pk PROMPT FILE`

#### Example:
```bash
$ pk "yaml to json" test.yaml 
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
Have GPT-4 process a file based on a prompt and send the results to another file.

#### Command:
`pk PROMPT SRC_FILE > DEST_FILE`

#### Example:
```bash
$ pk "yaml to json" test.yaml > test.json
```

### 5. Refactor a file in place
Have GPT-4 process a file based on a prompt and overwrite the file with the result.

**WARNING: Use this flag at your own risk! This is not reversible! Make sure you have another copy of the file or its current state is backed up in version control if you're concerned about losing its contents!**

#### Command:
`pk PROMPT FILE -i`

#### Example:
```bash
$ pk "all periods to dollar signs" test.txt -i
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
pk "yaml to json" "$1" 
```
Then use it and send stdout to another file:
```bash
$ ./yaml2json.sh sample.yaml > sample.json
```

## Installation
`pk` is written in Rust and needs to be compiled locally at this time (downloadable binaries coming soon!). 

1. Install Rust from https://www.rust-lang.org/ if you don't have it already.
2. Clone this repo locally.
3. Grab your OpenAI API key and set the `OPENAI_API_KEY` environment variable with it.
4. Run `cargo install --path .`

## Limitations & Considerations
* Any file you list after the prompt will be sent to the GPT-4 API over HTTPS. Do not send files you are not comfortable with sharing with OpenAI.
* GPT-4 is not good with counting or math in general. Asking it how many bytes are in a file or how many lines, or to run a total in a CSV will likely give inaccurate results.
* GPT-4 is however great with text and code! You can use `pk` to create a script to sum the results of a CSV and give it the CSV, then run the script.
* GPT-4 is not perfect! Always review any code it writes before running it and review any output it generates before trusting it. It's a great tool to speed you up, but you need to be the expert.

## Troubleshooting
If you're getting an error...
* Make sure you've set your OpenAI API key to the `OPENAI_API_KEY` environment variable.
* `pk` uses GPT 4, make sure your account has access to it (or get on the waitlist!).
* Make sure you have an Internet connection! `pk` hits the GPT-4 API directly for each command.
