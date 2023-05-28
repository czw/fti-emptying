// SPDX-License-Identifier: MIT

use anyhow::Result;
use i18n_embed::{
    fluent::{fluent_language_loader, FluentLanguageLoader},
    DesktopLanguageRequester, LanguageLoader,
};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "i18n"]
struct Localizations;

pub fn load_languages() -> Result<FluentLanguageLoader> {
    let loader = fluent_language_loader!();
    loader.load_available_languages(&Localizations)?;
    Ok(loader)
}

pub fn activate_correct_language(loader: &FluentLanguageLoader) -> Result<()> {
    let requested_languages = DesktopLanguageRequester::requested_languages();
    i18n_embed::select(loader, &Localizations, &requested_languages)?;
    Ok(())
}
