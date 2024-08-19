<!--
SPDX-FileCopyrightText: 2023 The WAG development team

SPDX-License-Identifier: GPL-3.0-or-later
-->

<div align="center">
  <a href="https://github.com/frack113/WAG/">
    <img src="https://raw.githubusercontent.com/frack113/WAG/main/media/logo.svg" alt="Logo" />
  </a>

  <h3 align="center">Windows Artifacts Generator</h3>

  <p align="center">
    Generate malware artifacts for detection tests
    <br />
    <a href="https://github.com/frack113/WAG/issues/">
      Report Bug
    </a>
    ·
    <a href="https://github.com/frack113/WAG/issues/">
      Request Feature
    </a>
    <br />
    <br />
    <a href="https://github.com/">
      <img src="https://img.shields.io/badge/GitHub-181717?logo=github&logoColor=fff&style=for-the-badge" alt="Github badge" />
    </a>
    <a href="./LICENSES/GPL-3.0-or-later.txt">
      <img src="https://img.shields.io/badge/License-GPL%203.0%20or%20later-green.svg?style=for-the-badge" alt="GPL 3.0 or later badge" />
    </a>
    <a href="https://www.microsoft.com/en-us/windows/">
      <img src="https://img.shields.io/badge/Windows-0078D4?logo=windows&logoColor=fff&style=for-the-badge" alt="Windows badge" />
    </a>
    <a href="https://www.rust-lang.org/">
      <img src="https://img.shields.io/badge/Rust-000?logo=rust&logoColor=fff&style=for-the-badge" alt="Rust badge" />
    </a>
    <a href="https://reuse.software/">
      <img src="https://img.shields.io/reuse/compliance/github.com%2Ffrack113%2FWAG?style=for-the-badge" alt="Reuse badge" />
    </a>
  </p>
</div>

## 📋 Table of content

- [📋 Table of content](#-table-of-content)
- [👀 About the project](#-about-the-project)
  - [❓ Why](#-why)
- [🚀 Getting started](#-getting-started)
  - [⚙️ Prerequisites](#%EF%B8%8F-prerequisites)
  - [📦 Installation](#-installation)
  - [🛠️ Build](#%EF%B8%8F-build)
  - [🥷 Quick examples](#-quick-examples)
- [👷 Contributing](#-contributing)
- [🙌 Acknowledgments](#-acknowledgments)
- [📚 Licenses](#-licenses)

## 👀 About the project

[Windows Artifacts Generator][WAG] is a tool for creating malware artifacts for detection tests.

### ❓ Why

It's useful for testing configurations, rules, or your Endpoint Detection and Response. \
It's not intended to fully simulate the behavior of malware but to reproduce the steps that led to artifact creation. \
By avoiding full and complex simulations, [Windows Artifacts Generator][WAG] seeks to be simple but nonetheless powerful.

## 🚀 Getting started

This is an example of how you can install or build the project yourself.

### ⚙️ Prerequisites

Depending on what you want to achieve, you might need different tools. \
For now, you only need [Cargo][Cargo] to build or install the project.

### 📦 Installation

Currently, this project is only available on [crates.io][crates.io]. \
In order to install it, just enter this command in your favorite terminal:

```sh
cargo install windows-artifacts-generator

```

### 🛠️ Build

1.  Clone the repository

    ```sh
    git clone https://github.com/frack113/WAG/
    ```

2.  Build and run the project!

    ```sh
    cargo run --release
    ```

After these steps, the application will be in the target directory.

### 🥷 Quick examples

Now that [WAG][WAG] is installed, you can start generating some artifacts! \
For example, you can create a file like this:

```sh
wag actions files create --name "example.txt"
```

Or you can generate artifacts from a configuration file:

```sh
wag generate --file "$YOUR_CONFIGURATIONS_FILE_PATH"
```

To see more information about what you can do, see the [documentation][Documentation].

## 👷 Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. \
Any contributions you make are **greatly appreciated**.

If you want, you can help me with any kind of work, for example:

- Correct my English errors
- Develop features
- Code refactoring
- Licensing stuff

## 🙌 Acknowledgments

Thanks to all the people who made the logo possible:

- "bug-ant" icon from [Heroicons][Heroicons] [MIT][MIT]
- "finger-print" icon from [Heroicons][Heroicons] [MIT][MIT]
- "Beep Sans" font by [Agbama Ulimhuka][Agbama Ulimhuka] [SIL OFL][SIL OFL]

## 📚 Licenses

Distributed under the [GPL 3.0 or later][GPL 3.0 or later] license.

[WAG]: https://github.com/frack113/WAG/
[Cargo]: https://doc.rust-lang.org/stable/cargo/
[crates.io]: https://crates.io/
[Documentation]: https://frack113.github.io/WAG/
[Heroicons]: https://heroicons.com/
[MIT]: ./LICENSES/MIT.txt
[Agbama Ulimhuka]: https://github.com/ulims/
[SIL OFL]: ./LICENSES/OFL-1.1.txt
[GPL 3.0 or later]: ./LICENSES/GPL-3.0-or-later.txt
