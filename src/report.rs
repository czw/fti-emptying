// SPDX-License-Identifier: MIT

use crate::fti::ContainerDatesMap;
use crate::humanize;
use anyhow::{Context, Result};
use i18n_embed::fluent::FluentLanguageLoader;
use i18n_embed_fl::fl;

/// Look at two status updates from FTI and generate messages if any container
/// has been emptied recently.
pub fn generate_message_strings(
    i18n: &FluentLanguageLoader,
    status: &ContainerDatesMap,
    old_status: Option<ContainerDatesMap>,
) -> Vec<String> {
    // No old status means no messages
    let mut messages = Vec::new();
    let old_status = old_status.unwrap_or(ContainerDatesMap::new());
    for (container, (emptied, _)) in status {
        if let Some((old_emptied, _)) = old_status.get(container) {
            if emptied != old_emptied {
                messages.push(fl!(
                    i18n,
                    "CONTAINER-just-emptied",
                    container = container.clone()
                ));
            }
        }
    }
    messages
}

pub fn notify_console(i18n: FluentLanguageLoader, status: &ContainerDatesMap) -> Result<()> {
    let now = chrono::Local::now();
    for (container, (emptied, scheduled)) in status {
        let msg = fl!(
            i18n,
            "CONTAINER-emptied-at-NOW-next-emptying-at-NEXT",
            container = container.clone(),
            now = humanize::time(&i18n, now, *emptied)?,
            next = humanize::days_in_the_future(&i18n, now, *scheduled)
        );
        println!("{}", msg);
    }
    Ok(())
}

#[cfg(feature = "desktop-notifications")]
pub fn notify_desktop(messages: &Vec<String>) -> Result<()> {
    for message in messages {
        notify_rust::Notification::new()
            .summary("FTI")
            .body(message)
            .show()?;
    }
    Ok(())
}

pub fn notify_ntfy(
    messages: &Vec<String>,
    host: &String,
    topic: &String,
    station_id: u32,
) -> Result<()> {
    let uri = format!("{host}/{topic}-{station_id}");
    for message in messages {
        ureq::post(&uri)
            .send_string(message)
            .context("Couldn't send message to ntfy.sh server")?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate_strings(
        status: &ContainerDatesMap,
        old_status: Option<ContainerDatesMap>,
    ) -> Result<Vec<String>> {
        let i18n = crate::i18n::load_languages()?;
        Ok(generate_message_strings(&i18n, &status, old_status))
    }

    #[test]
    fn generate_messages_identical_statuses() -> Result<()> {
        // With when the old and new status are identical, no messages should
        // be generated.
        let now = chrono::Local::now();
        let mut status = ContainerDatesMap::new();
        status.insert("Plastic".to_string(), (now, now));
        let old_status = Some(status.clone());

        let messages = generate_strings(&status, old_status)?;
        assert_eq!(messages.len(), 0);
        Ok(())
    }

    #[test]
    fn generate_messages_no_old_status() -> Result<()> {
        // With no old status, no messages should be generated.
        let now = chrono::Local::now();
        let mut status = ContainerDatesMap::new();
        status.insert("Plastic".to_string(), (now, now));
        let old_status: Option<ContainerDatesMap> = None;

        let messages = generate_strings(&status, old_status)?;
        assert_eq!(messages.len(), 0);
        Ok(())
    }

    #[test]
    fn generate_messages_one_updated() -> Result<()> {
        // We have one update
        let now = chrono::Local::now();
        let then = now
            .checked_sub_signed(chrono::Duration::seconds(3600))
            .context("Failed to set a time an hour ago")?;
        let mut status = ContainerDatesMap::new();
        status.insert("Plastic".to_string(), (now, now));
        let mut old = ContainerDatesMap::new();
        old.insert("Plastic".to_string(), (then, then));
        let old_status: Option<ContainerDatesMap> = Some(old);

        let messages = generate_strings(&status, old_status)?;
        assert_eq!(messages.len(), 1);
        Ok(())
    }
}
