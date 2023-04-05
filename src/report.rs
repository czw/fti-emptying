// SPDX-License-Identifier: MIT

use crate::fti::ContainerDatesMap;
use crate::humanize;
use anyhow::Result;

/// Look at two status updates from FTI and generate messages if any container
/// has been emptied recently.
pub fn generate_message_strings(
    status: &ContainerDatesMap,
    old_status: Option<ContainerDatesMap>,
) -> Vec<String> {
    // No old status means no messages
    let mut messages = Vec::new();
    let old_status = old_status.unwrap_or(ContainerDatesMap::new());
    for (container, (emptied, _)) in status {
        if let Some((old_emptied, _)) = old_status.get(container) {
            if emptied != old_emptied {
                messages.push(format!("{container} has just been emptied"));
            }
        }
    }
    messages
}

pub fn notify_console(status: &ContainerDatesMap) -> Result<()> {
    let now = chrono::Local::now();
    for (container, (emptied, scheduled)) in status {
        println!(
            "{}: Emptied {}, next emptying {}",
            container,
            humanize::time(now, *emptied)?,
            humanize::days_in_the_future(now, *scheduled)
        );
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

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Context;

    #[test]
    fn generate_messages_identical_statuses() {
        // With when the old and new status are identical, no messages should
        // be generated.
        let now = chrono::Local::now();
        let mut status = ContainerDatesMap::new();
        status.insert("Plastic".to_string(), (now, now));
        let old_status = Some(status.clone());

        let messages = generate_message_strings(&status, old_status);
        assert_eq!(messages.len(), 0);
    }

    #[test]
    fn generate_messages_no_old_status() {
        // With no old status, no messages should be generated.
        let now = chrono::Local::now();
        let mut status = ContainerDatesMap::new();
        status.insert("Plastic".to_string(), (now, now));
        let old_status: Option<ContainerDatesMap> = None;

        let messages = generate_message_strings(&status, old_status);
        assert_eq!(messages.len(), 0);
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

        let messages = generate_message_strings(&status, old_status);
        assert_eq!(messages.len(), 1);
        Ok(())
    }
}
