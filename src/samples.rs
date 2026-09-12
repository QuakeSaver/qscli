//! Decoding the samples a miniSEED record carries.
//!
//! The download files records away untouched, but the live view has to draw
//! them, so the encodings the sensors actually write are decoded here: Steim1
//! and Steim2 for the integer counts, and the plain integer and floating point
//! layouts for everything else.
//!
//! Steim packs first differences into 64 byte frames of sixteen 32 bit words.
//! The first word of every frame holds sixteen 2 bit codes saying how to read
//! each word of that frame, and the first frame spends two more words on the
//! first and last sample, so a reader can start and check itself. This follows
//! libmseed's `msr_decode_steim1`/`steim2`, including its habit of dropping the
//! very first difference in favour of the stated first sample.

use eyre::{eyre, Result};

use crate::mseed::RecordHeader;

/// A Steim frame: sixteen 32 bit words.
const FRAME_LEN: usize = 64;
const WORDS_PER_FRAME: usize = 16;

// The encodings blockette 1000 names, as far as we decode them.
const ASCII: u8 = 0;
const INT16: u8 = 1;
const INT32: u8 = 3;
const FLOAT32: u8 = 4;
const FLOAT64: u8 = 5;
const STEIM1: u8 = 10;
const STEIM2: u8 = 11;

/// Which Steim generation a record is packed with. They share a frame layout
/// and differ only in how a word's 2 bit code maps to differences.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Steim {
    One,
    Two,
}

/// Read `bits` bits starting at `at`, sign-extended to a full integer.
fn signed_field(word: u32, at: u32, bits: u32) -> i32 {
    let value = (word >> at) & ((1u32 << bits) - 1);
    let sign_bit = 1u32 << (bits - 1);
    (value ^ sign_bit).wrapping_sub(sign_bit) as i32
}

/// The differences one Steim word holds, decided by its 2 bit code and, in
/// Steim2, by the two leading bits of the word itself.
///
/// An empty list is a word carrying no samples, which is how both generations
/// spell padding.
fn word_differences(steim: Steim, code: u32, word: u32, bytes: &[u8]) -> Result<Vec<i32>> {
    // The one-byte differences are read straight out of the record, since a
    // single byte has no byte order to get wrong.
    let byte_diffs = || bytes.iter().map(|b| *b as i8 as i32).collect::<Vec<i32>>();
    // Steim leans on the top two bits of the word for the denser packings.
    let dnib = word >> 30;

    let diffs = match (steim, code) {
        // 00 marks a word that carries no differences at all.
        (_, 0) => Vec::new(),
        // 01 is four one-byte differences in both generations.
        (_, 1) => byte_diffs(),
        (Steim::One, 2) => vec![signed_field(word, 16, 16), signed_field(word, 0, 16)],
        (Steim::One, 3) => vec![word as i32],
        (Steim::Two, 2) => match dnib {
            1 => vec![signed_field(word, 0, 30)],
            2 => (0..2)
                .map(|i| signed_field(word, 15 - i * 15, 15))
                .collect(),
            3 => (0..3)
                .map(|i| signed_field(word, 20 - i * 10, 10))
                .collect(),
            _ => return Err(eyre!("record uses an undefined Steim2 packing (10, 00)")),
        },
        (Steim::Two, 3) => match dnib {
            0 => (0..5).map(|i| signed_field(word, 24 - i * 6, 6)).collect(),
            1 => (0..6).map(|i| signed_field(word, 25 - i * 5, 5)).collect(),
            2 => (0..7).map(|i| signed_field(word, 24 - i * 4, 4)).collect(),
            _ => return Err(eyre!("record uses an undefined Steim2 packing (11, 11)")),
        },
        // The code is two bits wide, so there is no fifth case.
        _ => unreachable!("a 2 bit code is 0..=3"),
    };
    Ok(diffs)
}

/// Rebuild the samples of a Steim-compressed record.
///
/// Stops once `samples` of them are out, the way the header promises, even if
/// the frames hold room for more.
fn decode_steim(
    steim: Steim,
    payload: &[u8],
    samples: usize,
    big_endian: bool,
) -> Result<Vec<i32>> {
    let word_at = |frame: &[u8], index: usize| {
        let raw: [u8; 4] = frame[index * 4..index * 4 + 4]
            .try_into()
            .expect("a frame word is four bytes");
        if big_endian {
            u32::from_be_bytes(raw)
        } else {
            u32::from_le_bytes(raw)
        }
    };

    let mut out: Vec<i32> = Vec::with_capacity(samples);
    // The opening frame spends two words on the first and last sample, so a
    // reader knows where to start and can check itself at the end.
    let mut first_sample = 0i32;
    let mut last_sample = 0i32;

    for (number, frame) in payload.as_chunks::<FRAME_LEN>().0.iter().enumerate() {
        if out.len() >= samples {
            break;
        }
        let codes = word_at(frame, 0);
        let first_word = if number == 0 {
            first_sample = word_at(frame, 1) as i32;
            last_sample = word_at(frame, 2) as i32;
            3
        } else {
            1
        };

        for index in first_word..WORDS_PER_FRAME {
            if out.len() >= samples {
                break;
            }
            let code = (codes >> (30 - 2 * index)) & 0b11;
            let bytes = &frame[index * 4..index * 4 + 4];
            for difference in word_differences(steim, code, word_at(frame, index), bytes)? {
                if out.len() >= samples {
                    break;
                }
                match out.last() {
                    // The first difference is the one reaching back before the
                    // record, so the stated first sample stands in for it.
                    None => out.push(first_sample),
                    Some(previous) => out.push(previous.wrapping_add(difference)),
                }
            }
        }
    }

    if out.len() < samples {
        return Err(eyre!(
            "record promised {} samples but its frames held {}",
            samples,
            out.len()
        ));
    }
    // Walking the differences back out has to land on the sample the record
    // itself states, which catches a misread frame before it reaches a plot.
    if out.last().is_some_and(|last| *last != last_sample) {
        return Err(eyre!(
            "record fails its own integrity check, last sample {} against the stated {}",
            out.last().expect("checked just above"),
            last_sample
        ));
    }
    Ok(out)
}

/// Read fixed-width samples, whatever their width and byte order.
fn decode_fixed(
    payload: &[u8],
    samples: usize,
    big_endian: bool,
    encoding: u8,
) -> Result<Vec<f64>> {
    let width = match encoding {
        INT16 => 2,
        INT32 | FLOAT32 => 4,
        FLOAT64 => 8,
        _ => unreachable!("only the fixed-width encodings reach here"),
    };
    if payload.len() < samples * width {
        return Err(eyre!(
            "record promised {} samples but holds room for {}",
            samples,
            payload.len() / width
        ));
    }

    Ok(payload
        .chunks_exact(width)
        .take(samples)
        .map(|raw| {
            // Every width is read the same way, so the byte order is applied
            // once here rather than per encoding.
            let mut bytes = [0u8; 8];
            bytes[..width].copy_from_slice(raw);
            if big_endian {
                bytes[..width].reverse();
            }
            let word = u64::from_le_bytes(bytes);
            match encoding {
                INT16 => word as u16 as i16 as f64,
                INT32 => word as u32 as i32 as f64,
                FLOAT32 => f32::from_bits(word as u32) as f64,
                FLOAT64 => f64::from_bits(word),
                _ => unreachable!("only the fixed-width encodings reach here"),
            }
        })
        .collect())
}

/// Decode the samples of one record.
///
/// `record` is the whole record, header included, as [`crate::mseed`] cut it
/// out of the stream.
pub(crate) fn decode(header: &RecordHeader, record: &[u8]) -> Result<Vec<f64>> {
    if header.samples == 0 {
        return Ok(Vec::new());
    }
    let encoding = header
        .encoding
        .ok_or_else(|| eyre!("record does not say how its samples are encoded"))?;

    // A record whose data offset is past its end carries nothing we can read.
    let payload = record
        .get(header.data_offset..)
        .ok_or_else(|| eyre!("record's samples start past its end"))?;

    match encoding {
        STEIM1 | STEIM2 => {
            let steim = if encoding == STEIM1 {
                Steim::One
            } else {
                Steim::Two
            };
            let counts = decode_steim(steim, payload, header.samples, header.data_big_endian)?;
            Ok(counts.into_iter().map(|count| count as f64).collect())
        }
        INT16 | INT32 | FLOAT32 | FLOAT64 => {
            decode_fixed(payload, header.samples, header.data_big_endian, encoding)
        }
        // Log and opaque records ride the same streams as waveforms; they are
        // simply not something to draw.
        ASCII => Ok(Vec::new()),
        other => Err(eyre!(
            "record uses encoding {}, which sqcli cannot read",
            other
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mseed::parse_header;
    use crate::mseed::tests::record;

    /// A 512 byte record carrying `payload` as `count` samples in `encoding`.
    fn record_with(encoding: u8, count: u16, payload: &[u8]) -> Vec<u8> {
        let mut r = record("BLA", "", "HHZ", "QS");
        r[30..32].copy_from_slice(&count.to_be_bytes());
        r[52] = encoding;
        r[53] = 1; // big-endian samples, as the sensors write them
        r[64..64 + payload.len()].copy_from_slice(payload);
        r
    }

    /// One 64 byte Steim frame: the word of 2 bit codes, the two integration
    /// constants, and `words` as (code, word) pairs filling W3 onwards.
    fn frame(first: i32, last: i32, words: &[(u32, u32)]) -> Vec<u8> {
        let mut frame = vec![0u8; 64];
        let mut codes = 0u32;
        for (index, (code, word)) in words.iter().enumerate() {
            // Data words start at W3, and every word's 2 bit code sits that
            // many places down from the top of the first word.
            let at = 3 + index;
            codes |= code << (30 - 2 * at);
            frame[at * 4..at * 4 + 4].copy_from_slice(&word.to_be_bytes());
        }
        frame[0..4].copy_from_slice(&codes.to_be_bytes());
        frame[4..8].copy_from_slice(&first.to_be_bytes());
        frame[8..12].copy_from_slice(&last.to_be_bytes());
        frame
    }

    fn decoded(record: &[u8]) -> Result<Vec<f64>> {
        let header = parse_header(record)?.expect("header");
        decode(&header, record)
    }

    #[test]
    fn steim_stands_the_first_sample_in_for_the_first_difference() {
        // Four one-byte differences. The first is the step into the record from
        // whatever came before it, so it is dropped for the stated first sample.
        let data = u32::from_be_bytes([0x7F, 3, (-2i8) as u8, 1]);
        let record = record_with(STEIM2, 4, &frame(1000, 1002, &[(1, data)]));

        assert_eq!(
            decoded(&record).unwrap(),
            vec![1000.0, 1003.0, 1001.0, 1002.0]
        );
    }

    #[test]
    fn steim2_unpacks_the_denser_codes() {
        // Code 11 with dnib 10: seven 4 bit differences, the tightest packing
        // Steim2 has and the one a quiet sensor spends most of its time in.
        let diffs = [0x0, 0x1, 0x2, 0xF, 0x3, 0xE, 0x1];
        let mut data = 0b10u32 << 30;
        for (index, diff) in diffs.iter().enumerate() {
            data |= diff << (24 - index * 4);
        }
        let record = record_with(STEIM2, 7, &frame(100, 104, &[(3, data)]));

        assert_eq!(
            decoded(&record).unwrap(),
            vec![100.0, 101.0, 103.0, 102.0, 105.0, 103.0, 104.0]
        );
    }

    #[test]
    fn steim2_reads_a_single_wide_difference() {
        // Code 10 with dnib 01: one 30 bit difference, which is how a step too
        // large for the packed forms is carried. A word holding a single
        // difference needs two of them, since the first stands in for X0.
        let wide = |diff: i32| (0b01u32 << 30) | ((diff as u32) & 0x3FFF_FFFF);
        let record = record_with(
            STEIM2,
            2,
            &frame(1_000_000, 500_000, &[(2, wide(0)), (2, wide(-500_000))]),
        );

        assert_eq!(decoded(&record).unwrap(), vec![1_000_000.0, 500_000.0]);
    }

    #[test]
    fn steim1_reads_its_own_widths() {
        // Steim1 spells code 10 as two 16 bit differences.
        let data = u32::from_be_bytes([0x00, 0x00, 0xFF, 0x9C]); // 0, then -100
        let record = record_with(STEIM1, 2, &frame(5000, 4900, &[(2, data)]));
        assert_eq!(decoded(&record).unwrap(), vec![5000.0, 4900.0]);

        // And code 11 as one 32 bit difference, so two words make two samples.
        let record = record_with(
            STEIM1,
            2,
            &frame(7000, 6000, &[(3, 0), (3, (-1_000i32) as u32)]),
        );
        assert_eq!(decoded(&record).unwrap(), vec![7000.0, 6000.0]);
    }

    #[test]
    fn a_misread_record_is_caught_by_its_own_last_sample() {
        // The frame says the record ends on 999, but walking the differences
        // lands on 1002. A record that fails this is one we misread.
        let data = u32::from_be_bytes([0, 3, (-2i8) as u8, 1]);
        let record = record_with(STEIM2, 4, &frame(1000, 999, &[(1, data)]));

        let error = decoded(&record).expect_err("the check should fail");
        assert!(
            error.to_string().contains("integrity"),
            "unexpected error: {}",
            error
        );
    }

    #[test]
    fn a_record_promising_more_than_it_holds_is_rejected() {
        let data = u32::from_be_bytes([0, 3, 0, 0]);
        // One frame can hold nowhere near 5000 samples.
        let record = record_with(STEIM2, 5000, &frame(1000, 1003, &[(1, data)]));

        assert!(decoded(&record).is_err());
    }

    #[test]
    fn the_plain_encodings_are_read_in_the_stated_byte_order() {
        let payload: Vec<u8> = [1i32, -2, 3].iter().flat_map(|v| v.to_be_bytes()).collect();
        let record = record_with(INT32, 3, &payload);
        assert_eq!(decoded(&record).unwrap(), vec![1.0, -2.0, 3.0]);

        let payload: Vec<u8> = [1.5f32, -0.25]
            .iter()
            .flat_map(|v| v.to_be_bytes())
            .collect();
        let record = record_with(FLOAT32, 2, &payload);
        assert_eq!(decoded(&record).unwrap(), vec![1.5, -0.25]);

        let payload: Vec<u8> = [7i16, -9].iter().flat_map(|v| v.to_be_bytes()).collect();
        let record = record_with(INT16, 2, &payload);
        assert_eq!(decoded(&record).unwrap(), vec![7.0, -9.0]);
    }

    #[test]
    fn little_endian_samples_are_read_the_other_way_round() {
        let payload: Vec<u8> = [1i32, -2].iter().flat_map(|v| v.to_le_bytes()).collect();
        let mut record = record_with(INT32, 2, &payload);
        record[53] = 0; // the word order byte of blockette 1000

        assert_eq!(decoded(&record).unwrap(), vec![1.0, -2.0]);
    }

    #[test]
    fn records_carrying_no_waveform_decode_to_nothing() {
        // A log record rides the same stream as the waveforms.
        let record = record_with(ASCII, 12, b"a log line");
        assert!(decoded(&record).unwrap().is_empty());

        // And a record stating no samples at all has nothing to give.
        let record = record_with(STEIM2, 0, &[]);
        assert!(decoded(&record).unwrap().is_empty());
    }

    #[test]
    fn an_encoding_we_cannot_read_is_named_in_the_error() {
        let record = record_with(19, 4, &[0; 16]);
        let error = decoded(&record).expect_err("unsupported");
        assert!(error.to_string().contains("19"), "unexpected: {}", error);
    }
}
