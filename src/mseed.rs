//! Just enough miniSEED to file a record away.
//!
//! The download never looks at samples, so this reads only the fixed section of
//! the data header: which stream the record belongs to, when it starts and how
//! long it is. That is all the SDS layout needs, and it keeps us independent of
//! a decoder for the many miniSEED encodings.

use chrono::{NaiveDate, NaiveDateTime};
use eyre::{eyre, Result};
use log::warn;

/// The fixed section of the data header, present on every record.
const FIXED_HEADER_LEN: usize = 48;
/// Enough bytes to read the fixed header and the blockette that follows it.
const MIN_PARSE_LEN: usize = 64;
/// Used when a record carries no blockette 1000, as pre-2.3 writers did.
const FALLBACK_RECORD_LEN: usize = 512;
/// Blockettes live in the header, so a chain reaching this far is malformed.
const MAX_BLOCKETTE_SCAN: usize = 1024;

/// Where a record belongs and when it starts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RecordHeader {
    pub(crate) network: String,
    pub(crate) station: String,
    pub(crate) location: String,
    pub(crate) channel: String,
    pub(crate) start: NaiveDateTime,
    /// Total record length in bytes, header included.
    pub(crate) length: usize,
}

impl RecordHeader {
    /// The stream this record belongs to, in the usual `NET.STA.LOC.CHAN` form.
    pub(crate) fn stream_id(&self) -> String {
        format!(
            "{}.{}.{}.{}",
            self.network, self.station, self.location, self.channel
        )
    }
}

/// Reads the SEED byte order from the start time, the only header field with a
/// range narrow enough to tell the two apart. Records are meant to be
/// big-endian, but little-endian ones exist in the wild.
fn plausible_start(year: u16, day_of_year: u16) -> bool {
    (1900..=2100).contains(&year) && (1..=366).contains(&day_of_year)
}

/// A SEED code, trimmed of the spaces it is padded with.
///
/// Codes end up in file names, so anything but the alphanumerics SEED allows is
/// rejected rather than sanitised: a record we cannot name is a record we do not
/// understand.
fn seed_code(bytes: &[u8], field: &str) -> Result<String> {
    let code = std::str::from_utf8(bytes)
        .map_err(|_| eyre!("record has a non-UTF-8 {} code", field))?
        .trim()
        .to_string();
    if code
        .chars()
        .any(|c| !c.is_ascii_alphanumeric() && c != '-' && c != '_')
    {
        return Err(eyre!("record has an unusable {} code {:?}", field, code));
    }
    Ok(code)
}

/// Rebuild the record start time from the SEED `BTIME` fields.
fn start_time(
    year: u16,
    day_of_year: u16,
    hour: u8,
    minute: u8,
    second: u8,
    tenths_of_milli: u16,
) -> Result<NaiveDateTime> {
    let date = NaiveDate::from_yo_opt(year as i32, day_of_year as u32).ok_or_else(|| {
        eyre!(
            "record starts on an impossible date {}-{}",
            year,
            day_of_year
        )
    })?;
    // A leap second is carried in the nanoseconds, the way chrono represents it.
    let (second, leap) = if second == 60 {
        (59u32, 1_000_000_000u32)
    } else {
        (second as u32, 0)
    };
    let nanos = leap + tenths_of_milli as u32 * 100_000;
    date.and_hms_nano_opt(hour as u32, minute as u32, second, nanos)
        .ok_or_else(|| eyre!("record starts at an impossible time of day"))
}

/// Walk the blockette chain for blockette 1000, which states the record length
/// as a power of two.
///
/// Returns `None` while the chain runs past the bytes we have, so the caller can
/// wait for more of the stream.
fn record_length(buf: &[u8], first_blockette: usize, big_endian: bool) -> Option<Option<usize>> {
    let read_u16 = |at: usize| {
        let raw = [buf[at], buf[at + 1]];
        if big_endian {
            u16::from_be_bytes(raw)
        } else {
            u16::from_le_bytes(raw)
        }
    };

    let mut offset = first_blockette;
    while offset != 0 && offset < MAX_BLOCKETTE_SCAN {
        if offset + 8 > buf.len() {
            // The chain continues past what we have been handed so far.
            return None;
        }
        if read_u16(offset) == 1000 {
            let exponent = buf[offset + 6];
            // 2^8 = 256 bytes is the smallest record SEED allows, 2^20 a
            // generous ceiling; anything else means we are misreading.
            if !(8..=20).contains(&exponent) {
                return Some(None);
            }
            return Some(Some(1usize << exponent));
        }
        let next = read_u16(offset + 2) as usize;
        // The chain must move forward, or we would spin on a malformed record.
        if next <= offset {
            break;
        }
        offset = next;
    }
    Some(None)
}

/// Parse the header of the record at the start of `buf`.
///
/// `Ok(None)` means the buffer is too short to decide yet.
pub(crate) fn parse_header(buf: &[u8]) -> Result<Option<RecordHeader>> {
    if buf.len() < MIN_PARSE_LEN {
        return Ok(None);
    }

    // The quality indicator is the cheapest check that we are still aligned on a
    // record boundary rather than reading the middle of one.
    let quality = buf[6];
    if !matches!(quality, b'D' | b'R' | b'Q' | b'M') {
        return Err(eyre!(
            "expected a miniSEED record, found quality byte {:?}",
            quality as char
        ));
    }

    let be = |at: usize| u16::from_be_bytes([buf[at], buf[at + 1]]);
    let le = |at: usize| u16::from_le_bytes([buf[at], buf[at + 1]]);
    let big_endian = if plausible_start(be(20), be(22)) {
        true
    } else if plausible_start(le(20), le(22)) {
        false
    } else {
        return Err(eyre!("record carries an unreadable start time"));
    };
    let read_u16 = |at: usize| if big_endian { be(at) } else { le(at) };

    let start = start_time(
        read_u16(20),
        read_u16(22),
        buf[24],
        buf[25],
        buf[26],
        // buf[27] is unused padding.
        read_u16(28),
    )?;

    let length = match record_length(buf, read_u16(46) as usize, big_endian) {
        None => return Ok(None),
        Some(Some(length)) => length,
        Some(None) => {
            warn!(
                "record without a readable blockette 1000, assuming {} byte records",
                FALLBACK_RECORD_LEN
            );
            FALLBACK_RECORD_LEN
        }
    };
    if length < FIXED_HEADER_LEN {
        return Err(eyre!("record claims to be only {} bytes long", length));
    }

    Ok(Some(RecordHeader {
        station: seed_code(&buf[8..13], "station")?,
        location: seed_code(&buf[13..15], "location")?,
        channel: seed_code(&buf[15..18], "channel")?,
        network: seed_code(&buf[18..20], "network")?,
        start,
        length,
    }))
}

/// Cuts a stream of bytes into whole miniSEED records.
///
/// Records arrive split across HTTP chunks, so bytes are buffered until a
/// complete record is available.
#[derive(Default)]
pub(crate) struct RecordStream {
    buf: Vec<u8>,
    /// How much of `buf` has already been handed out.
    consumed: usize,
}

impl RecordStream {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn push(&mut self, chunk: &[u8]) {
        // Drop what has been consumed before growing, so a long download does
        // not keep the whole response in memory.
        if self.consumed > 0 && self.consumed == self.buf.len() {
            self.buf.clear();
            self.consumed = 0;
        } else if self.consumed > 1 << 20 {
            self.buf.drain(..self.consumed);
            self.consumed = 0;
        }
        self.buf.extend_from_slice(chunk);
    }

    /// The next complete record, or `None` while more bytes are needed.
    pub(crate) fn next_record(&mut self) -> Result<Option<(RecordHeader, &[u8])>> {
        let pending = &self.buf[self.consumed..];
        let Some(header) = parse_header(pending)? else {
            return Ok(None);
        };
        if pending.len() < header.length {
            return Ok(None);
        }
        let start = self.consumed;
        self.consumed += header.length;
        Ok(Some((header, &self.buf[start..self.consumed])))
    }

    /// Check that the stream ended on a record boundary.
    ///
    /// Some servers pad the response, so trailing filler is tolerated; a partial
    /// record is not, because it means the download was cut short.
    pub(crate) fn finish(&self) -> Result<()> {
        let leftover = &self.buf[self.consumed..];
        if leftover.iter().all(|b| *b == 0 || b.is_ascii_whitespace()) {
            return Ok(());
        }
        Err(eyre!(
            "download ended mid-record with {} bytes left over",
            leftover.len()
        ))
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// A 512 byte record with a blockette 1000, big-endian like the real ones.
    pub(crate) fn record(station: &str, location: &str, channel: &str, network: &str) -> Vec<u8> {
        record_on_day(station, location, channel, network, 42)
    }

    /// The same, placed on a chosen day of 2026, for the day-file layout.
    pub(crate) fn record_on_day(
        station: &str,
        location: &str,
        channel: &str,
        network: &str,
        day_of_year: u16,
    ) -> Vec<u8> {
        let mut r = vec![0u8; 512];
        r[..6].copy_from_slice(b"000001");
        r[6] = b'D';
        r[7] = b' ';
        let pad = |dst: &mut [u8], code: &str| {
            dst.fill(b' ');
            dst[..code.len()].copy_from_slice(code.as_bytes());
        };
        pad(&mut r[8..13], station);
        pad(&mut r[13..15], location);
        pad(&mut r[15..18], channel);
        pad(&mut r[18..20], network);
        r[20..22].copy_from_slice(&2026u16.to_be_bytes()); // year
        r[22..24].copy_from_slice(&day_of_year.to_be_bytes());
        r[24] = 13; // hour
        r[25] = 37; // minute
        r[26] = 5; // second
        r[28..30].copy_from_slice(&1234u16.to_be_bytes()); // 0.1234 s
        r[39] = 1; // one blockette follows
        r[44..46].copy_from_slice(&64u16.to_be_bytes()); // start of data
        r[46..48].copy_from_slice(&48u16.to_be_bytes()); // first blockette
        r[48..50].copy_from_slice(&1000u16.to_be_bytes()); // blockette type
        r[50..52].copy_from_slice(&0u16.to_be_bytes()); // no next blockette
        r[54] = 9; // 2^9 = 512 byte records
        r
    }

    #[test]
    fn a_record_header_yields_its_stream_and_start() {
        let header = parse_header(&record("APE", "00", "BHZ", "GE"))
            .unwrap()
            .expect("header");

        assert_eq!(header.stream_id(), "GE.APE.00.BHZ");
        assert_eq!(header.length, 512);
        assert_eq!(
            header.start,
            NaiveDate::from_ymd_opt(2026, 2, 11)
                .unwrap()
                .and_hms_nano_opt(13, 37, 5, 123_400_000)
                .unwrap()
        );
    }

    #[test]
    fn an_empty_location_stays_empty() {
        let header = parse_header(&record("APE", "", "BHZ", "GE"))
            .unwrap()
            .expect("header");
        assert_eq!(header.location, "");
        assert_eq!(header.stream_id(), "GE.APE..BHZ");
    }

    #[test]
    fn a_short_buffer_asks_for_more_data() {
        let full = record("APE", "00", "BHZ", "GE");
        assert!(parse_header(&full[..32]).unwrap().is_none());
    }

    #[test]
    fn a_little_endian_record_is_read_too() {
        let mut r = record("APE", "00", "BHZ", "GE");
        r[20..22].copy_from_slice(&2026u16.to_le_bytes());
        r[22..24].copy_from_slice(&42u16.to_le_bytes());
        r[28..30].copy_from_slice(&1234u16.to_le_bytes());
        r[44..46].copy_from_slice(&64u16.to_le_bytes());
        r[46..48].copy_from_slice(&48u16.to_le_bytes());
        r[48..50].copy_from_slice(&1000u16.to_le_bytes());
        r[50..52].copy_from_slice(&0u16.to_le_bytes());

        let header = parse_header(&r).unwrap().expect("header");
        assert_eq!(header.stream_id(), "GE.APE.00.BHZ");
        assert_eq!(header.length, 512);
    }

    #[test]
    fn data_that_is_not_miniseed_is_rejected() {
        let mut junk = vec![b'x'; 64];
        junk[6] = b'x';
        assert!(parse_header(&junk).is_err());
    }

    #[test]
    fn a_code_that_would_escape_the_archive_is_rejected() {
        let mut r = record("APE", "00", "BHZ", "GE");
        r[8..13].copy_from_slice(b"../.."); // a station code aimed at the parent
        assert!(parse_header(&r).is_err());
    }

    #[test]
    fn records_are_cut_out_of_a_chunked_stream() {
        let first = record("APE", "00", "BHZ", "GE");
        let second = record("BLA", "", "HHN", "QS");
        let wire: Vec<u8> = first.iter().chain(second.iter()).copied().collect();

        let mut stream = RecordStream::new();
        // Split mid-record, the way an HTTP body arrives.
        stream.push(&wire[..300]);
        assert!(stream.next_record().unwrap().is_none());

        stream.push(&wire[300..700]);
        let (header, bytes) = stream.next_record().unwrap().expect("first record");
        assert_eq!(header.stream_id(), "GE.APE.00.BHZ");
        assert_eq!(bytes.len(), 512);
        assert!(stream.next_record().unwrap().is_none());

        stream.push(&wire[700..]);
        let (header, _) = stream.next_record().unwrap().expect("second record");
        assert_eq!(header.stream_id(), "QS.BLA..HHN");
        assert!(stream.next_record().unwrap().is_none());
        stream.finish().unwrap();
    }

    #[test]
    fn a_truncated_download_is_reported() {
        let full = record("APE", "00", "BHZ", "GE");
        let mut stream = RecordStream::new();
        stream.push(&full[..400]);
        assert!(stream.next_record().unwrap().is_none());
        assert!(stream.finish().is_err());
    }

    #[test]
    fn trailing_padding_is_tolerated() {
        let full = record("APE", "00", "BHZ", "GE");
        let mut stream = RecordStream::new();
        stream.push(&full);
        stream.push(b"\n");
        stream.next_record().unwrap().expect("record");
        stream.finish().unwrap();
    }
}
