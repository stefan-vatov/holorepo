# Omnirepo

>This tool is currently in its early stages of development. You are welcome to use it if it appears beneficial for your needs; however, please be prepared to encounter rough spots.

Omnirepo is a command-line tool for managing multiple Git repositories. It allows you to organize, clone, and run commands in multiple repositories simultaneously. Omnirepo is especially useful for developers who work with multiple repositories, making the workflow more efficient and streamlined.

## Table of Contents

- [Omnirepo](#omnirepo)
  - [Table of Contents](#table-of-contents)
  - [Features](#features)
  - [Installation](#installation)
  - [Usage](#usage)
    - [CLI Help](#cli-help)
    - [Config File](#config-file)
  - [Commands](#commands)
    - [new](#new)
    - [clone](#clone)
    - [run](#run)
    - [sync](#sync)
  - [Contributing](#contributing)
  - [License](#license)

## Features

- Manage multiple Git repositories from a single config file.
- Clone repositories in parallel.
- Run commands in each repository simultaneously.
- Synchronize files across repositories. (TODO)

## Installation

1. Clone this repository.
2. Navigate to the project's root directory and run `cargo build --release`.
3. Add the compiled binary to your `PATH`.

## Usage

### CLI Help

```plaintext
A tool for managing multiple git repositories

Usage: omnirepo [OPTIONS] <COMMAND>

Commands:
  new    Create a new repository
  clone  Clone a group of repositories based on tags
  run    Run a command in each repository
  sync   Sync a file across all repositories
  help   Print this message or the help of the given subcommand(s)

Options:
  -c, --config <CONFIG>    Point to a .omnirepo.yaml or a directory containing config
  -v, --verbose <VERBOSE>  Log to file [possible values: true, false]
  -h, --help               Print help
  -V, --version            Print version
```

### Config File

Create a `.omnirepo.yaml` file in your user's home directory with the following format (example):

```yml
---

repositories:
  - name: Glimmer config
    url: <valid-clone-url>
    dest: glimmer_config
    tags:
      - config
      - ansible
  - name: Private dotfiles
    url: <valid-clone-url>
    dest: dotfiles
    tags:
      - config
      - dotfiles

templates:
  - name: pre-commit
    url: https://raw.githubusercontent.com/stefan-vatov/omni-templates/main/default/.pre-commit-config.yaml
    kind: File
    dest: "."
    tags:
      - default
      - ci
  - name: .gitignore
    url: https://raw.githubusercontent.com/stefan-vatov/omni-templates/main/default/.gitignore
    kind: File
    dest: "."
    tags:
      - default
  - name: GitHub Workflows
    url: https://raw.githubusercontent.com/stefan-vatov/omni-templates/main/github_workflows
    kind: Dir
    included_files:
      - file_name: pre-commit-hooks.yml
        dest: .github/workflows
    tags:
      - ci

```

## Commands

### new

Create a new repository.

Passing tags with `-t` for the new repo is optional.
Any files with the `default` tag _will be_ automatically added.


```plaintext
Create a new repository

Usage: omnirepo new [OPTIONS] --name <NAME>

Options:
  -n, --name <NAME>                The name of the repository
  -t, --tags <TAGS>                The names of the tags to clone
  -d, --destination <DESTINATION>  Destination to create new repository, current folder by default
  -h, --help                       Print help
```

### clone

Clone a group of repositories based on tags.

```plaintext
Clone a group of repositories based on tags

Usage: omnirepo clone [OPTIONS]

Options:
  -t, --tags <TAGS>                The names of the tags to clone
  -d, --destination <DESTINATION>  Destination to clone the repositories, current folder by default
  -h, --help                       Print help
```

### run

Run a command in each repository.

```plaintext
Run a command in each repository

Usage: omnirepo run [OPTIONS] --command <COMMAND>

Options:
  -c, --command <COMMAND>          The command to run
  -d, --destination <DESTINATION>  Destination to folder where the repos were cloned, current folder by default.
  -h, --help                       Print help
```

### sync

Sync a file across all repositories.
If the file does not exist it will be created.

```plaintext
Sync a file across all repositories

Usage: omnirepo sync [OPTIONS] --file <FILE>

Options:
  -f, --file <FILE>                    The file to sync
  -u, --url <URL>                      Source file for syncing from URL
  -s, --source-file <SOURCE_FILE>      Local source file for syncing
  -t, --template-file <TEMPLATE_FILE>  Template file for syncing
  -d, --destination <DESTINATION>      Destination to folder where the repos were cloned, current folder by default.
  -h, --help                           Print help
```

## Contributing

Contributions are welcome! Please submit a pull request or create an issue to propose changes or report bugs.

## License

This project is open source and available under the MIT License.
