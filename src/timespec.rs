//! Time and duration arguments for the waveform download.
//!
//! Everything is UTC, the way FDSN and SEED count time.

use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, Utc};
use eyre::{eyre, Result};

/// The date and time layouts accepted for an absolute timestamp.
const ABSOLUTE_FORMATS: [&str; 6] = [
    "%Y-%m-%dT%H:%M:%S%.f",
    "%Y-%m-%dT%H:%M",
    "%Y-%m-%d %H:%M:%S%.f",
    "%Y-%m-%d %H:%M",
    "%Y-%m-%dT%H",
    "%Y-%m-%d %H",
];

/// Parse a duration such as `10m`, `1h30m`, `2d` or a bare number of seconds.
pub(crate) fn parse_duration(text: &str) -> Result<Duration> {
    let text = text.trim();
    if text.is_empty() {
        return Err(eyre!("empty duration"));
    }

    let mut total = Duration::zero();
    let mut digits = String::new();
    let mut saw_unit = false;

    for c in text.chars() {
        if c.is_ascii_digit() {
            digits.push(c);
            continue;
        }
        let count: i64 = digits
            .parse()
            .map_err(|_| eyre!("{:?} is not a duration, expected something like 10m", text))?;
        digits.clear();
        saw_unit = true;
        let unit = match c {
            's' => Duration::seconds(1),
            'm' => Duration::minutes(1),
            'h' => Duration::hours(1),
            'd' => Duration::days(1),
            'w' => Duration::weeks(1),
            _ => return Err(eyre!("unknown duration unit {:?}, use s, m, h, d or w", c)),
        };
        total = total
            .checked_add(&(unit * count as i32))
            .ok_or_else(|| eyre!("duration {:?} is out of range", text))?;
    }

    // A bare number is a count of seconds.
    if !digits.is_empty() {
        let seconds: i64 = digits
            .parse()
            .map_err(|_| eyre!("{:?} is not a duration", text))?;
        total += Duration::seconds(seconds);
    } else if !saw_unit {
        return Err(eyre!(
            "{:?} is not a duration, expected something like 10m",
            text
        ));
    }

    if total <= Duration::zero() {
        return Err(eyre!("duration {:?} must be positive", text));
    }
    Ok(total)
}

/// Parse a point in time: `now`, an offset from now such as `-1h`, or an
/// absolute UTC timestamp such as `2026-09-01T12:00:00`.
pub(crate) fn parse_time(text: &str, now: NaiveDateTime) -> Result<NaiveDateTime> {
    let text = text.trim();
    if text.eq_ignore_ascii_case("now") {
        return Ok(now);
    }
    if let Some(offset) = text.strip_prefix('-') {
        return Ok(now - parse_duration(offset)?);
    }
    if let Some(offset) = text.strip_prefix('+') {
        return Ok(now + parse_duration(offset)?);
    }

    // A timestamp carrying its own offset, e.g. 2026-09-01T12:00:00+02:00.
    if let Ok(fixed) = DateTime::parse_from_rfc3339(text) {
        return Ok(fixed.with_timezone(&Utc).naive_utc());
    }

    // FDSN times are UTC, so a trailing Z is redundant rather than wrong.
    let bare = text.trim_end_matches(['Z', 'z']);
    for format in ABSOLUTE_FORMATS {
        if let Ok(parsed) = NaiveDateTime::parse_from_str(bare, format) {
            return Ok(parsed);
        }
    }
    if let Ok(date) = NaiveDate::parse_from_str(bare, "%Y-%m-%d") {
        return Ok(date
            .and_hms_opt(0, 0, 0)
            .expect("midnight is a valid time of day"));
    }

    Err(eyre!(
        "{:?} is not a time, expected `now`, an offset like `-1h`, or a timestamp like `2026-09-01T12:00:00`",
        text
    ))
}

/// Work out the window to download from whichever of the three arguments the
/// user supplied.
///
/// Any two of them fix the window. A start on its own runs up to now, and a
/// duration on its own is the window ending now.
pub(crate) fn resolve_window(
    start: Option<&str>,
    end: Option<&str>,
    duration: Option<&str>,
    now: NaiveDateTime,
) -> Result<(NaiveDateTime, NaiveDateTime)> {
    let start = start.map(|s| parse_time(s, now)).transpose()?;
    let end = end.map(|s| parse_time(s, now)).transpose()?;
    let duration = duration.map(parse_duration).transpose()?;

    let (start, end) = match (start, end, duration) {
        (Some(start), Some(end), _) => (start, end),
        (Some(start), None, Some(duration)) => (start, start + duration),
        (None, Some(end), Some(duration)) => (end - duration, end),
        (Some(start), None, None) => (start, now),
        (None, None, Some(duration)) => (now - duration, now),
        (None, Some(_), None) => {
            return Err(eyre!(
                "--end alone does not say how much data to fetch, add --start or --duration"
            ))
        }
        (None, None, None) => {
            return Err(eyre!(
                "no time range given, use --start/--end or --duration (e.g. --duration 10m)"
            ))
        }
    };

    if end <= start {
        return Err(eyre!(
            "the time range ends before it starts ({} to {})",
            start,
            end
        ));
    }
    Ok((start, end))
}

/// Split a window into request-sized pieces, cut at UTC midnight.
///
/// Midnight is where SDS starts a new day file, so day-long chunks line up with
/// the archive and keep any single request modest.
pub(crate) fn chunks(
    start: NaiveDateTime,
    end: NaiveDateTime,
    max: Duration,
) -> Vec<(NaiveDateTime, NaiveDateTime)> {
    let mut chunks = Vec::new();
    let mut current = start;
    while current < end {
        let midnight = (current.date() + Duration::days(1))
            .and_hms_opt(0, 0, 0)
            .expect("midnight is a valid time of day");
        let next = end.min(midnight).min(current + max);
        chunks.push((current, next));
        current = next;
    }
    chunks
}

/// The way FDSN wants a timestamp written.
pub(crate) fn fdsn_time(time: NaiveDateTime) -> String {
    time.format("%Y-%m-%dT%H:%M:%S%.3f").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn now() -> NaiveDateTime {
        NaiveDate::from_ymd_opt(2026, 9, 9)
            .unwrap()
            .and_hms_opt(12, 0, 0)
            .unwrap()
    }

    fn at(year: i32, month: u32, day: u32, hour: u32, minute: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(year, month, day)
            .unwrap()
            .and_hms_opt(hour, minute, 0)
            .unwrap()
    }

    #[test]
    fn durations_accept_units_and_combinations() {
        assert_eq!(parse_duration("30s").unwrap(), Duration::seconds(30));
        assert_eq!(parse_duration("10m").unwrap(), Duration::minutes(10));
        assert_eq!(parse_duration("2h").unwrap(), Duration::hours(2));
        assert_eq!(parse_duration("1w").unwrap(), Duration::weeks(1));
        assert_eq!(parse_duration("1h30m").unwrap(), Duration::minutes(90));
        // A bare number is seconds.
        assert_eq!(parse_duration("45").unwrap(), Duration::seconds(45));
    }

    #[test]
    fn nonsense_durations_are_rejected() {
        assert!(parse_duration("").is_err());
        assert!(parse_duration("soon").is_err());
        assert!(parse_duration("10y").is_err());
        assert!(parse_duration("0s").is_err());
    }

    #[test]
    fn times_can_be_absolute_or_relative() {
        assert_eq!(parse_time("now", now()).unwrap(), now());
        assert_eq!(parse_time("-1h", now()).unwrap(), at(2026, 9, 9, 11, 0));
        assert_eq!(
            parse_time("2026-09-01T12:00:00", now()).unwrap(),
            at(2026, 9, 1, 12, 0)
        );
        assert_eq!(
            parse_time("2026-09-01 12:00", now()).unwrap(),
            at(2026, 9, 1, 12, 0)
        );
        assert_eq!(
            parse_time("2026-09-01", now()).unwrap(),
            at(2026, 9, 1, 0, 0)
        );
    }

    #[test]
    fn a_trailing_z_is_accepted_and_an_offset_is_converted() {
        assert_eq!(
            parse_time("2026-09-01T12:00:00Z", now()).unwrap(),
            at(2026, 9, 1, 12, 0)
        );
        // 14:00 in +02:00 is 12:00 UTC.
        assert_eq!(
            parse_time("2026-09-01T14:00:00+02:00", now()).unwrap(),
            at(2026, 9, 1, 12, 0)
        );
    }

    #[test]
    fn any_two_window_arguments_fix_the_range() {
        let (start, end) =
            resolve_window(Some("2026-09-01"), Some("2026-09-02"), None, now()).unwrap();
        assert_eq!((start, end), (at(2026, 9, 1, 0, 0), at(2026, 9, 2, 0, 0)));

        let (start, end) = resolve_window(Some("2026-09-01"), None, Some("2h"), now()).unwrap();
        assert_eq!((start, end), (at(2026, 9, 1, 0, 0), at(2026, 9, 1, 2, 0)));

        let (start, end) = resolve_window(None, Some("2026-09-01"), Some("2h"), now()).unwrap();
        assert_eq!((start, end), (at(2026, 8, 31, 22, 0), at(2026, 9, 1, 0, 0)));
    }

    #[test]
    fn a_lone_start_runs_to_now_and_a_lone_duration_ends_now() {
        let (start, end) = resolve_window(Some("-3h"), None, None, now()).unwrap();
        assert_eq!((start, end), (at(2026, 9, 9, 9, 0), now()));

        let (start, end) = resolve_window(None, None, Some("10m"), now()).unwrap();
        assert_eq!((start, end), (at(2026, 9, 9, 11, 50), now()));
    }

    #[test]
    fn an_unusable_window_is_refused() {
        assert!(resolve_window(None, None, None, now()).is_err());
        assert!(resolve_window(None, Some("2026-09-01"), None, now()).is_err());
        // Backwards.
        assert!(resolve_window(Some("2026-09-02"), Some("2026-09-01"), None, now()).is_err());
    }

    #[test]
    fn chunks_are_cut_at_midnight() {
        let cut = chunks(
            at(2026, 9, 1, 22, 0),
            at(2026, 9, 3, 3, 0),
            Duration::days(1),
        );
        assert_eq!(
            cut,
            vec![
                (at(2026, 9, 1, 22, 0), at(2026, 9, 2, 0, 0)),
                (at(2026, 9, 2, 0, 0), at(2026, 9, 3, 0, 0)),
                (at(2026, 9, 3, 0, 0), at(2026, 9, 3, 3, 0)),
            ]
        );
    }

    #[test]
    fn a_short_window_stays_one_chunk() {
        let cut = chunks(
            at(2026, 9, 1, 10, 0),
            at(2026, 9, 1, 10, 30),
            Duration::days(1),
        );
        assert_eq!(cut, vec![(at(2026, 9, 1, 10, 0), at(2026, 9, 1, 10, 30))]);
    }

    #[test]
    fn a_smaller_chunk_size_splits_further() {
        let cut = chunks(
            at(2026, 9, 1, 10, 0),
            at(2026, 9, 1, 13, 0),
            Duration::hours(1),
        );
        assert_eq!(cut.len(), 3);
    }

    #[test]
    fn fdsn_times_carry_milliseconds() {
        assert_eq!(fdsn_time(at(2026, 9, 1, 12, 0)), "2026-09-01T12:00:00.000");
    }
}
