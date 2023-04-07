// SPDX-License-Identifier: MIT

use anyhow::{Context, Result};
use clap::Parser;
use std::time::Duration;
mod fti;
mod humanize;
mod report;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct CLArgs {
    /// Recycling station ID number
    station_id: u32,

    /// Send emptying notifications to the desktop environment
    #[cfg(feature = "desktop-notifications")]
    #[arg(long)]
    notify_desktop: bool,

    /// Send emptying notifications to a ntfy.sh server
    #[arg(long, value_name = "HOST")]
    notify_ntfy_host: Option<String>,

    /// Base topic name to use for ntfy.sh notifications, where the final topic
    /// will be in the form of <TOPIC>-<STATION_ID>
    #[arg(long, default_value = "fti-emptying", value_name = "TOPIC")]
    notify_ntfy_topic: String,
}

fn calc_sleep_time(status: &fti::ContainerDatesMap) -> Result<Duration> {
    // If there's a container being emptied today, check every 15 minutes
    let now = chrono::Local::now();
    let today = now.date_naive();
    for (_, emptying) in status.values() {
        if emptying.date_naive() == today {
            return Ok(Duration::from_secs(15 * 60));
        }
    }

    // Schedule a check tomorrow at 07:00
    let tomorrow = today
        .checked_add_days(chrono::Days::new(1))
        .context("Can't turn today into tomorrow")?;
    let morning = chrono::NaiveTime::from_hms_opt(7, 0, 0).context("Can't build the time 07:00")?;
    let target = tomorrow.and_time(morning);
    let target = chrono::TimeZone::from_local_datetime(&chrono::Local, &target);
    let target = target
        .single()
        .context("Failed to convert a naive date and time to the local timezone")?;
    Ok((target - now).to_std()?)
}

#[cfg(feature = "desktop-notifications")]
fn is_daemon(args: &CLArgs) -> bool {
    args.notify_desktop || args.notify_ntfy_host.is_some()
}

#[cfg(not(feature = "desktop-notifications"))]
fn is_daemon(args: &CLArgs) -> bool {
    args.notify_ntfy_host.is_some()
}

fn main() -> Result<()> {
    let args = CLArgs::parse();
    let daemon_mode = is_daemon(&args);
    let mut old_status: Option<fti::ContainerDatesMap> = None;
    loop {
        // Check once and report the results if not running in daemon mode.
        let status = fti::fetch_recycling_station_status(args.station_id)?;
        if !daemon_mode {
            report::notify_console(&status)?;
            break;
        }

        // Send notifications if there are any messages
        let messages = report::generate_message_strings(&status, old_status);
        if !messages.is_empty() {
            println!("{} containers were emptied, notifying", messages.len());

            #[cfg(feature = "desktop-notifications")]
            if args.notify_desktop {
                report::notify_desktop(&messages)?;
            }

            if let Some(ref host) = args.notify_ntfy_host {
                report::notify_ntfy(&messages, host, &args.notify_ntfy_topic, args.station_id)?;
            }
        }

        // Wait for a while until the next check
        let duration = calc_sleep_time(&status)?;
        println!(
            "Waiting for {} seconds until next check",
            duration.as_secs()
        );
        std::thread::sleep(duration);
        old_status = Some(status);
    }
    Ok(())
}
