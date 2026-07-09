// SPDX-FileCopyrightText: 2023 The MalwareTracesGenerator development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

mod commands;
mod displayer;
mod metadata;
mod windows;

use clap::Parser;
use commands::Arguments;
use std::process::ExitCode;

fn main() -> ExitCode {
    Arguments::parse().run()
}
