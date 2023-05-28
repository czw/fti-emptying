// SPDX-License-Identifier: MIT

use anyhow::{bail, Result};
use chrono::{DateTime, Local};
use i18n_embed::fluent::FluentLanguageLoader;
use i18n_embed_fl::fl;

pub fn days_in_the_future(
    i18n: &FluentLanguageLoader,
    now: DateTime<Local>,
    then: DateTime<Local>,
) -> String {
    let days = (then.date_naive() - now.date_naive()).num_days();
    fl!(i18n, "DAYS-IN-THE-FUTURE", days = days)
}

pub fn time(
    i18n: &FluentLanguageLoader,
    now: DateTime<Local>,
    then: DateTime<Local>,
) -> Result<String> {
    let delta = then - now;
    match delta.num_seconds() {
        i64::MIN..=-2_678_400 => Ok(fl!(i18n, "a-really-long-time-ago")),
        -2_678_399..=-86400 => Ok(fl!(i18n, "DAYS-days-ago", days = delta.num_days().abs())),
        -86399..=-3600 => Ok(fl!(
            i18n,
            "HOURS-hours-ago",
            hours = delta.num_hours().abs()
        )),
        -3599..=-60 => Ok(fl!(
            i18n,
            "MINUTES-minutes-ago",
            minutes = delta.num_minutes().abs()
        )),
        -59..=-1 => Ok(fl!(
            i18n,
            "SECONDS-seconds-ago",
            seconds = delta.num_seconds().abs()
        )),
        0 => Ok(fl!(i18n, "now")),
        _ => bail!("I'm not handling future events"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    fn check_time(duration: Duration) -> Result<String> {
        let i18n = crate::i18n::load_languages()?;
        let now = Local::now();
        let then = now + duration;
        let str = time(&i18n, now, then)?;
        Ok(str.replace("\u{2068}", "").replace("\u{2069}", ""))
    }

    #[test]
    fn time_a_day_ago() -> Result<()> {
        let humanstr = check_time(-Duration::days(1))?;
        assert_eq!(humanstr, "a day ago".to_string());
        Ok(())
    }

    #[test]
    fn time_5_days_ago() -> Result<()> {
        let humanstr = check_time(-Duration::days(5))?;
        assert_eq!(humanstr, "5 days ago".to_string());
        Ok(())
    }

    #[test]
    fn time_an_hour_ago() -> Result<()> {
        let humanstr = check_time(-Duration::hours(1))?;
        assert_eq!(humanstr, "an hour ago".to_string());
        Ok(())
    }

    #[test]
    fn time_5_hours_ago() -> Result<()> {
        let humanstr = check_time(-Duration::hours(5))?;
        assert_eq!(humanstr, "5 hours ago".to_string());
        Ok(())
    }

    #[test]
    fn time_a_minute_ago() -> Result<()> {
        let humanstr = check_time(-Duration::minutes(1))?;
        assert_eq!(humanstr, "a minute ago".to_string());
        Ok(())
    }

    #[test]
    fn time_5_minutes_ago() -> Result<()> {
        let humanstr = check_time(-Duration::minutes(5))?;
        assert_eq!(humanstr, "5 minutes ago".to_string());
        Ok(())
    }

    #[test]
    fn time_a_second_ago() -> Result<()> {
        let humanstr = check_time(-Duration::seconds(1))?;
        assert_eq!(humanstr, "a second ago".to_string());
        Ok(())
    }

    #[test]
    fn time_5_seconds_ago() -> Result<()> {
        let humanstr = check_time(-Duration::seconds(5))?;
        assert_eq!(humanstr, "5 seconds ago".to_string());
        Ok(())
    }
}
