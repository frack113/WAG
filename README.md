<!--
SPDX-FileCopyrightText: 2023 The WAG development team

SPDX-License-Identifier: GPL-3.0-or-later
-->

<div align="center">
  <a href="https://github.com/frack113/WAG/">
    <img src="./media/wag.ico" alt="Logo" />
  </a>

  <h3 align="center">Windows Artifacts Generator</h3>

  <p align="center">
    Generate malware's artifacts for detection testing
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
    <a href="https://spdx.org/licenses/AGPL-3.0-or-later.html">
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

## :clipboard: Table of content

- [:clipboard: Table of content](#clipboard-table-of-content)
- [:eyes: About the project](#eyes-about-the-project)
  - [:question: Why](#question-why)
- [:rocket: Getting started](#rocket-getting-started)
  - [:gear: Prerequisites](#gear-prerequisites)
  - [:hammer_and_wrench: Build](#hammer_and_wrench-build)
- [:construction_worker: Contributing](#construction_worker-contributing)
- [:raised_hands: Acknowledgments](#raised_hands-acknowledgments)
- [:books: Licenses](#books-licenses)

## :eyes: About the project

[Windows Artifacts Generator][WAG] is a tool for creating malware's artifacts for detection testing.

### :question: Why

It's useful for testing configurations, rules or your Endpoint Detection and Response. \
It's not intended to fully simulate the comportment of malware but reproduce steps that led to artifacts creation. \
By avoiding full and complex simulations, [Windows Artifacts Generator][WAG] seek to be simple but nonetheless powerful.

## :rocket: Getting started

This is an example of how you can install or build the project yourself.

### :gear: Prerequisites

Depending on what you want to achieve, you might need different tools. \
For now, you only need [Cargo][Cargo] to build or install the project.

### :hammer_and_wrench: Build

1.  Clone the repository

    ```sh
    git clone https://github.com/frack113/WAG/
    ```

2.  Build and run the project!

    ```sh
    cargo run --release
    ```

After these steps, the application will be in the target directory.

### :ninja: Quick examples

Now that [WAG][WAG] is installed, you can start generating some artifacts! \
For example you can create a file like this:

```sh
wag actions files create --name "example.txt"
```

Or you can generate artifacts from a configuration file:

```sh
wag generate --file "$YOUR_CONFIGURATIONS_FILE_PATH"
```

To see more information about what you can do, see the [documentations][Documentations].

## :construction_worker: Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. \
Any contributions you make are **greatly appreciated**.

If you want, you can help me with any kind of work, for example:

- Correct my English errors
- Develop features
- Code refactoring
- Licensing stuff

## :raised_hands: Acknowledgments

Thanks to all people who made the logo possible:

- "bug-ant" icon from [Heroicons][Heroicons] [MIT][MIT]
- "finger-print" icon from [Heroicons][Heroicons] [MIT][MIT]
- "Beep Sans" font by [Agbama Ulimhuka][Agbama Ulimhuka] [SIL OFL][SIL OFL]

## :books: Licenses

Distributed under the [GPL 3.0 or later][GPL 3.0 or later] license.

[WAG]: https://github.com/frack113/WAG/
[Cargo]: https://doc.rust-lang.org/stable/cargo/
[Documentations]: https://frack113.github.io/WAG/
[Heroicons]: https://heroicons.com/
[MIT]: ./LICENSES/MIT.txt
[Agbama Ulimhuka]: https://github.com/ulims/
[SIL OFL]: ./LICENSES/OFL-1.1.txt
[GPL 3.0 or later]: ./LICENSES/GPL-3.0-or-later.txt
