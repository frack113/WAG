// SPDX-FileCopyrightText: 2023 The WAG development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::actions::Actions;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version)]
#[clap(arg_required_else_help = true)]
pub struct Arguments {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Actions(Actions),
}
