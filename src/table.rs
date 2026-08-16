//! Left-aligned plain-text tables, shared by the `detect` and `sensors` output.

/// The number of terminal columns a string occupies.
///
/// Only ASCII text and the fixed set of sensor icons ever reach this, so a
/// simple rule is enough: emoji from U+1F300 upwards render two columns wide,
/// everything else one. `String::len` would count bytes and misalign the icons.
fn display_width(text: &str) -> usize {
    text.chars()
        .map(|c| if c >= '\u{1F300}' { 2 } else { 1 })
        .sum()
}

fn print_row(cells: &[String], widths: &[usize]) {
    let padded: Vec<String> = cells
        .iter()
        .zip(widths)
        .map(|(cell, width)| {
            let padding = width.saturating_sub(display_width(cell));
            format!("{}{}", cell, " ".repeat(padding))
        })
        .collect();
    println!("{}", padded.join("  ").trim_end());
}

/// Print `headers` followed by `rows`, every column padded to its widest cell.
///
/// Each row must hold exactly one cell per header.
pub(crate) fn print_table(headers: &[&str], rows: &[Vec<String>]) {
    let widths: Vec<usize> = headers
        .iter()
        .enumerate()
        .map(|(column, header)| {
            rows.iter()
                .map(|row| display_width(&row[column]))
                .max()
                .unwrap_or(0)
                .max(display_width(header))
        })
        .collect();

    let header_cells: Vec<String> = headers.iter().map(|h| h.to_string()).collect();
    print_row(&header_cells, &widths);
    for row in rows {
        print_row(row, &widths);
    }
}

#[cfg(test)]
mod tests {
    use super::display_width;

    #[test]
    fn ascii_width_is_the_character_count() {
        assert_eq!(display_width("MEMS"), 4);
        assert_eq!(display_width(""), 0);
    }

    #[test]
    fn emoji_are_two_columns_wide() {
        assert_eq!(display_width("🌀"), 2);
        assert_eq!(display_width("🌋"), 2);
    }

    #[test]
    fn non_emoji_symbols_stay_single_width() {
        // U+25A3, the MEMS icon: three bytes, but a single terminal column.
        assert_eq!("▣".len(), 3);
        assert_eq!(display_width("▣"), 1);
    }
}
