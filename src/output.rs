//! Where downloaded records land: one file, standard output, or an SDS archive.

use std::collections::BTreeSet;
use std::fs::{create_dir_all, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf, MAIN_SEPARATOR};

use chrono::Datelike;
use eyre::{Context, Result};
use log::debug;

use crate::mseed::RecordHeader;

/// How many day files to keep open at once. A download covering many sensors
/// and many days touches more files than we may hold open, so the least
/// recently used one is closed to make room.
const MAX_OPEN_FILES: usize = 64;

/// SDS stores waveform data, which the layout marks with a `D`.
const SDS_TYPE: char = 'D';

/// Where the caller wants the data.
pub(crate) enum Target {
    Stdout,
    /// Every record appended to a single miniSEED file.
    File(PathBuf),
    /// An SDS archive rooted at this directory.
    Sds(PathBuf),
}

impl Target {
    /// Read the target out of the `--output` argument.
    ///
    /// `-` is standard output. A path naming an existing directory, or written
    /// with a trailing separator, is an SDS archive root; anything else is a
    /// plain file.
    pub(crate) fn from_arg(arg: &str) -> Self {
        if arg == "-" {
            return Target::Stdout;
        }
        let looks_like_a_directory =
            arg.ends_with('/') || arg.ends_with(MAIN_SEPARATOR) || Path::new(arg).is_dir();
        if looks_like_a_directory {
            Target::Sds(PathBuf::from(arg))
        } else {
            Target::File(PathBuf::from(arg))
        }
    }
}

/// The path a record takes inside an SDS archive:
/// `<root>/<year>/<net>/<sta>/<chan>.D/<net>.<sta>.<loc>.<chan>.D.<year>.<doy>`
fn sds_path(root: &Path, header: &RecordHeader) -> PathBuf {
    let year = header.start.year();
    let day_of_year = header.start.ordinal();
    root.join(year.to_string())
        .join(&header.network)
        .join(&header.station)
        .join(format!("{}.{}", header.channel, SDS_TYPE))
        .join(format!(
            "{}.{}.{}.{}.{}.{}.{:03}",
            header.network,
            header.station,
            header.location,
            header.channel,
            SDS_TYPE,
            year,
            day_of_year
        ))
}

/// An SDS archive, with a handful of day files open at a time.
pub(crate) struct SdsArchive {
    root: PathBuf,
    /// Open day files, most recently written first.
    open: Vec<(PathBuf, BufWriter<std::fs::File>)>,
    /// Every file this run appended to, for the closing summary.
    touched: BTreeSet<PathBuf>,
}

impl SdsArchive {
    fn new(root: PathBuf) -> Self {
        SdsArchive {
            root,
            open: Vec::new(),
            touched: BTreeSet::new(),
        }
    }

    /// The writer for `path`, opening the file if it is not open already.
    ///
    /// Files are opened for appending, so a download extends an existing archive
    /// rather than replacing it.
    fn writer(&mut self, path: PathBuf) -> Result<&mut BufWriter<std::fs::File>> {
        if let Some(index) = self.open.iter().position(|(open, _)| *open == path) {
            // Move to the front so the files in use survive eviction.
            let entry = self.open.remove(index);
            self.open.insert(0, entry);
            return Ok(&mut self.open[0].1);
        }

        if self.open.len() >= MAX_OPEN_FILES {
            let (path, mut file) = self.open.pop().expect("a file to evict");
            file.flush()
                .wrap_err_with(|| format!("failed to write {}", path.display()))?;
            debug!(
                "closed {} to stay under the open file limit",
                path.display()
            );
        }

        if let Some(parent) = path.parent() {
            create_dir_all(parent)
                .wrap_err_with(|| format!("failed to create {}", parent.display()))?;
        }
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .wrap_err_with(|| format!("failed to open {}", path.display()))?;
        self.touched.insert(path.clone());
        self.open.insert(0, (path, BufWriter::new(file)));
        Ok(&mut self.open[0].1)
    }
}

/// The open destination for a download.
pub(crate) enum Writer {
    /// Standard output or a single file, both written straight through.
    Stream(Box<dyn Write>),
    Sds(SdsArchive),
}

impl Target {
    /// Whether the data is going to standard output, where a summary table would
    /// land in the middle of the miniSEED.
    pub(crate) fn is_stdout(&self) -> bool {
        matches!(self, Target::Stdout)
    }
}

impl Writer {
    pub(crate) fn open(target: Target) -> Result<Self> {
        match target {
            Target::Stdout => Ok(Writer::Stream(Box::new(BufWriter::new(std::io::stdout())))),
            Target::File(path) => {
                if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
                    create_dir_all(parent)
                        .wrap_err_with(|| format!("failed to create {}", parent.display()))?;
                }
                let file = std::fs::File::create(&path)
                    .wrap_err_with(|| format!("failed to create {}", path.display()))?;
                Ok(Writer::Stream(Box::new(BufWriter::new(file))))
            }
            Target::Sds(root) => Ok(Writer::Sds(SdsArchive::new(root))),
        }
    }

    pub(crate) fn write_record(&mut self, header: &RecordHeader, record: &[u8]) -> Result<()> {
        match self {
            Writer::Stream(out) => out.write_all(record).wrap_err("failed to write output"),
            Writer::Sds(archive) => {
                let path = sds_path(&archive.root, header);
                let file = archive.writer(path)?;
                file.write_all(record).wrap_err("failed to write record")
            }
        }
    }

    /// Flush everything and report how many files were written.
    pub(crate) fn finish(self) -> Result<usize> {
        match self {
            Writer::Stream(mut out) => {
                out.flush().wrap_err("failed to flush output")?;
                Ok(1)
            }
            Writer::Sds(mut archive) => {
                for (path, file) in archive.open.iter_mut() {
                    file.flush()
                        .wrap_err_with(|| format!("failed to write {}", path.display()))?;
                }
                Ok(archive.touched.len())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn header(network: &str, station: &str, location: &str, channel: &str) -> RecordHeader {
        RecordHeader {
            network: network.to_string(),
            station: station.to_string(),
            location: location.to_string(),
            channel: channel.to_string(),
            start: NaiveDate::from_ymd_opt(2026, 2, 11)
                .unwrap()
                .and_hms_opt(13, 37, 5)
                .unwrap(),
            length: 512,
            samples: 100,
            sample_rate: 100.0,
            data_offset: 64,
            encoding: Some(11),
            data_big_endian: true,
        }
    }

    #[test]
    fn a_record_lands_in_the_sds_layout() {
        let path = sds_path(Path::new("/archive"), &header("GE", "APE", "00", "BHZ"));
        assert_eq!(
            path,
            Path::new("/archive/2026/GE/APE/BHZ.D/GE.APE.00.BHZ.D.2026.042")
        );
    }

    #[test]
    fn an_empty_location_keeps_its_place_in_the_name() {
        let path = sds_path(Path::new("archive"), &header("QS", "BLA", "", "HHN"));
        assert_eq!(
            path,
            Path::new("archive/2026/QS/BLA/HHN.D/QS.BLA..HHN.D.2026.042")
        );
    }

    #[test]
    fn the_day_of_year_is_padded_to_three_digits() {
        let mut early = header("GE", "APE", "00", "BHZ");
        early.start = NaiveDate::from_ymd_opt(2026, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        assert!(sds_path(Path::new("a"), &early)
            .to_str()
            .unwrap()
            .ends_with(".2026.001"));
    }

    #[test]
    fn a_trailing_separator_means_an_sds_archive() {
        assert!(matches!(Target::from_arg("out/"), Target::Sds(_)));
        assert!(matches!(Target::from_arg("out.mseed"), Target::File(_)));
        assert!(matches!(Target::from_arg("-"), Target::Stdout));
    }

    #[test]
    fn an_existing_directory_means_an_sds_archive() {
        let dir = std::env::temp_dir().join("sqcli-sds-target-test");
        create_dir_all(&dir).unwrap();
        assert!(matches!(
            Target::from_arg(dir.to_str().unwrap()),
            Target::Sds(_)
        ));
        std::fs::remove_dir_all(&dir).ok();
    }

    /// Records are routed to a day file per stream and per day, and each file
    /// holds its records byte for byte, in the order they arrived.
    #[test]
    fn an_archive_splits_records_by_stream_and_day() {
        use crate::mseed::{parse_header, tests::record_on_day};

        let root = std::env::temp_dir().join("sqcli-sds-write-test");
        std::fs::remove_dir_all(&root).ok();

        let wire = [
            record_on_day("APE", "00", "BHZ", "GE", 41),
            record_on_day("APE", "00", "BHN", "GE", 41),
            record_on_day("APE", "00", "BHZ", "GE", 42),
            // A second record for a file already written, to check appending.
            record_on_day("APE", "00", "BHZ", "GE", 41),
        ];

        let mut writer = Writer::open(Target::Sds(root.clone())).unwrap();
        for record in &wire {
            let header = parse_header(record).unwrap().expect("header");
            writer.write_record(&header, record).unwrap();
        }
        assert_eq!(writer.finish().unwrap(), 3);

        let day_file = |channel: &str, day: &str| {
            root.join(format!(
                "2026/GE/APE/{0}.D/GE.APE.00.{0}.D.2026.{1}",
                channel, day
            ))
        };
        // The two records for this stream and day, in arrival order.
        let mut both = wire[0].clone();
        both.extend_from_slice(&wire[3]);
        assert_eq!(std::fs::read(day_file("BHZ", "041")).unwrap(), both);
        assert_eq!(std::fs::read(day_file("BHN", "041")).unwrap(), wire[1]);
        assert_eq!(std::fs::read(day_file("BHZ", "042")).unwrap(), wire[2]);

        std::fs::remove_dir_all(&root).ok();
    }

    /// More streams than we may hold open at once still each get their file.
    #[test]
    fn an_archive_writes_more_files_than_it_keeps_open() {
        use crate::mseed::{parse_header, tests::record_on_day};

        let root = std::env::temp_dir().join("sqcli-sds-many-test");
        std::fs::remove_dir_all(&root).ok();

        let days: Vec<u16> = (1..=(MAX_OPEN_FILES as u16 + 10)).collect();
        let mut writer = Writer::open(Target::Sds(root.clone())).unwrap();
        for day in &days {
            let record = record_on_day("APE", "00", "BHZ", "GE", *day);
            let header = parse_header(&record).unwrap().expect("header");
            writer.write_record(&header, &record).unwrap();
        }
        assert_eq!(writer.finish().unwrap(), days.len());

        // Every day file survived the eviction of its handle.
        for day in &days {
            let path = root.join(format!("2026/GE/APE/BHZ.D/GE.APE.00.BHZ.D.2026.{:03}", day));
            assert_eq!(std::fs::metadata(&path).unwrap().len(), 512, "{:?}", path);
        }

        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn a_single_file_target_holds_every_record_in_order() {
        use crate::mseed::{parse_header, tests::record_on_day};

        let path = std::env::temp_dir().join("sqcli-file-test/out.mseed");
        std::fs::remove_dir_all(path.parent().unwrap()).ok();

        let wire = [
            record_on_day("APE", "00", "BHZ", "GE", 41),
            record_on_day("BLA", "", "HHN", "QS", 42),
        ];
        let mut writer = Writer::open(Target::File(path.clone())).unwrap();
        for record in &wire {
            let header = parse_header(record).unwrap().expect("header");
            writer.write_record(&header, record).unwrap();
        }
        writer.finish().unwrap();

        assert_eq!(std::fs::read(&path).unwrap(), wire.concat());
        std::fs::remove_dir_all(path.parent().unwrap()).ok();
    }
}
