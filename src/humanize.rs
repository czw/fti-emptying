// SPDX-License-Identifier: MIT

use anyhow::{bail, Result};
use chrono::{DateTime, Local};

pub fn days_in_the_future(now: DateTime<Local>, then: DateTime<Local>) -> String {
    let days = (then.date_naive() - now.date_naive()).num_days();
    match days {
        0 => "today".to_string(),
        1 => "tomorrow".to_string(),
        2 => "the day after tomorrow".to_string(),
        _ => format!("in {} days", days),
    }
}

pub fn time(now: DateTime<Local>, then: DateTime<Local>) -> Result<String> {
    let delta = then - now;
    match delta.num_seconds() {
        i64::MIN..=-2678400 => Ok("A really long time ago".to_string()),
        -2678399..=-86400 => Ok(format!("{} days ago", delta.num_days().abs())),
        -86399..=-3600 => Ok(format!("{} hours ago", delta.num_hours().abs())),
        -3599..=-60 => Ok(format!("{} minutes ago", delta.num_minutes().abs())),
        -59..=-1 => Ok(format!("{} seconds ago", delta.num_seconds().abs())),
        0 => Ok("now".to_string()),
        _ => bail!("I'm not handling future events"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    fn check_time(duration: Duration) -> Result<String> {
        let now = Local::now();
        let then = now.clone() + duration;
        time(now, then)
    }

    #[test]
    fn time_5_days_ago() -> Result<()> {
        let humanstr = check_time(-Duration::days(5))?;
        assert_eq!(humanstr, "5 days ago".to_string());
        Ok(())
    }

    #[test]
    fn time_5_hours_ago() -> Result<()> {
        let humanstr = check_time(-Duration::hours(5))?;
        assert_eq!(humanstr, "5 hours ago".to_string());
        Ok(())
    }

    #[test]
    fn time_5_minutes_ago() -> Result<()> {
        let humanstr = check_time(-Duration::minutes(5))?;
        assert_eq!(humanstr, "5 minutes ago".to_string());
        Ok(())
    }

    #[test]
    fn time_5_seconds_ago() -> Result<()> {
        let humanstr = check_time(-Duration::seconds(5))?;
        assert_eq!(humanstr, "5 seconds ago".to_string());
        Ok(())
    }
}
