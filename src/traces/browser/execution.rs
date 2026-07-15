// SPDX-FileCopyrightText: 2023 The MalwareTracesGenerator development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::traces::{
    TraceError,
    browser::parameters::{Browser, BrowserParameters},
};
use std::{fs, io};

pub(super) fn execute(parameters: BrowserParameters) -> Result<(), TraceError> {
    drop(fs::read_dir(&parameters.profile)?);

    for components in target_paths(parameters.browser) {
        let path = components
            .iter()
            .fold(parameters.profile.clone(), |path, component| {
                path.join(component)
            });

        if path.try_exists()? {
            let mut target_file = fs::File::open(path)?;
            io::copy(&mut target_file, &mut io::sink())?;
        }
    }

    Ok(())
}

fn target_paths(browser: Browser) -> &'static [&'static [&'static str]] {
    match browser {
        Browser::Firefox => &[
            &["places.sqlite"],
            &["cookies.sqlite"],
            &["logins.json"],
            &["key3.db"],
            &["key4.db"],
            &["formhistory.sqlite"],
            &["favicons.sqlite"],
        ],
        Browser::Chrome => &[
            &["History"],
            &["Cookies"],
            &["Network", "Cookies"],
            &["Login Data"],
            &["Bookmarks"],
            &["Web Data"],
            &["Preferences"],
            &["Top Sites"],
            &["Favicons"],
        ],
        Browser::Edge => &[
            &["History"],
            &["Cookies"],
            &["Network", "Cookies"],
            &["Login Data"],
            &["Bookmarks"],
            &["Web Data"],
            &["Preferences"],
        ],
        Browser::Opera => &[
            &["History"],
            &["Cookies"],
            &["Network", "Cookies"],
            &["Login Data"],
            &["Bookmarks"],
            &["Wand.dat"],
        ],
        Browser::Brave => &[
            &["History"],
            &["Cookies"],
            &["Network", "Cookies"],
            &["Login Data"],
            &["Bookmarks"],
            &["Web Data"],
        ],
    }
}
