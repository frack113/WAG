// SPDX-FileCopyrightText: 2023 The WAG team
//
// SPDX-License-Identifier: GPL-3.0-or-later

// build.rs

use winres::{self, WindowsResource};

fn main() {
    if cfg!(target_os = "windows") {
        let mut res: WindowsResource = winres::WindowsResource::new();

        //res.set_icon("rust.ico");
        res.set("FileVersion", "1.0.0");
        res.set("InternalName", "🔓 your detection");
        res.set("CompanyName", "Frack113");
        res.set(
            "FileDescription",
            "Generate Some Windows Artefact to 👊 your EDR",
        );
        res.set("LegalCopyright", "Frack113");
        res.set("LegalTrademark", "Frack113");
        res.set("OriginalFilename", "wag.exe");
        res.set("ProductName", "WindowsArtefactGenerator");
        res.set("ProductVersion", "0.1.0");
        res.set("Comments", "Catch me if you can😁");

        const LANG_NEUTRAL: u16 = 0x00;
        const SUBLANG_NEUTRAL: u16 = 0x00;

        res.set_language((LANG_NEUTRAL << 10) | SUBLANG_NEUTRAL);

        res.set_icon("./media/wag.ico");

        res.compile().unwrap();
    }
}
