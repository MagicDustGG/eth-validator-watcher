<div align="center">
    <h1 align="center">Ethereum Validator Watcher</h1>
  <p align="center">
    <a href="https://magicdust.gg">
        <img src="https://img.shields.io/badge/Website-https%3A%2F%2Fmagicdust.gg-blueviolet">
    </a>  
    <a href="https://etherscan.io/address/magicdust.eth">
        <img src="https://img.shields.io/static/v1?label=Ethereum&message=magicdust.eth&color=ff69b4">
    </a>
    <a href="https://github.com/MagicDustGG/template-rust-project/graphs/contributors">
      <img alt="GitHub contributors" src="https://img.shields.io/github/contributors/MagicDustGG/eth-validator-watcher">
    </a>
    <a href="http://makeapullrequest.com">
      <img alt="pull requests welcome badge" src="https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat">
    </a>
    <a href="https://twitter.com/intent/follow?screen_name=Magicdust_gg">
        <img src="https://img.shields.io/twitter/follow/Magicdust_gg?style=social&logo=twitter"
            alt="follow on Twitter"></a>
    <a href="https://opensource.org/licenses/Apache-2.0"><img src="https://img.shields.io/badge/License-Apache%202.0-blue.svg"
            alt="License"></a>
    <a href=""><img src="https://img.shields.io/badge/semver-0.1.0-blue"
            alt="License"></a>            
  </p>
  
  <h3 align="center">A tool to survey Validators</h3>
</div>


## Pre Requisites

Install Rust: https://www.rust-lang.org/tools/install

Install the crates used for formating, linging and coverage:

```
$ sh scripts/setup.sh
```

## Git hooks

We provide git hooks that match the Github action that will take place when you push your code.

You can find those under `.git_hooks/` and install them by running:

```
$ sh scripts/install_git_hooks.sh
```

## Lint TOML

We use Taplo: https://taplo.tamasfe.dev/

Lint all TOML files in the project:

```sh
$ taplo format 
```

## Test coverage

We use Mozilla Grcov: https://github.com/mozilla/grcov

Run the test coverage script:

```sh
$ sh scripts/test_coverage.sh
```

It will output profraw files under `./target/debug/profraw/`, create a html report under `./target/debug/coverage/` and open it.
