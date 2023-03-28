// SPDX-License-Identifier: MIT

use anyhow::Result;
use clap::Parser;
mod fti;
mod humanize;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct CLArgs {
    /// Recycling station ID number
    station_id: u32,
}

fn pretty_print(status: fti::ContainerDatesMap) -> Result<()> {
    let now = chrono::Local::now();
    for (container, (emptied, scheduled)) in status {
        println!(
            "{}: Emptied {}, next emptying {}",
            container,
            humanize::time(now, emptied)?,
            humanize::days_in_the_future(now, scheduled)
        );
    }
    Ok(())
}

fn main() -> Result<()> {
    let args = CLArgs::parse();
    let status = fti::fetch_recycling_station_status(args.station_id)?;
    pretty_print(status)?;
    Ok(())
}
