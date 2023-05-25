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

Usage: omnirepo <COMMAND>

Commands:
  new    Create a new repository
  clone  Clone a group of repositories based on tags
  run    Run a command in each repository
  sync   Sync a file across all repositories
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
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

### clone

Clone a group of repositories based on tags.

### run

Run a command in each repository.

### sync

(TODO)
Sync a file across all repositories.

## Contributing

Contributions are welcome! Please submit a pull request or create an issue to propose changes or report bugs.

## License

This project is open source and available under the MIT License.
