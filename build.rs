// SPDX-FileCopyrightText: 2023 The WAG development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

extern crate embed_resource;
fn main() {
    embed_resource::compile("wag-manifest.rc", embed_resource::NONE);
}
