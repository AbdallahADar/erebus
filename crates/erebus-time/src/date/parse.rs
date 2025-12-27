// === Imports ===
use crate::prelude::*;
use super::convert::ymd_to_days;
use super::validate::{validate_ymd_lenient, validate_ymd_strict};
use crate::calendar::gregorian::{month_from_short, month_from_long};

// === Types ===

#[derive(Debug, Clone)]
pub struct YmdExtractor {
    year: FieldExtractor,
    month: FieldExtractor,
    day: FieldExtractor,
    total_len: usize,
    literals: Vec<LiteralCheck>,
}

#[derive(Debug, Clone)]
struct LiteralCheck {
    idx: usize,
    byte: u8,
}

#[derive(Debug, Clone)]
enum FieldExtractor {
    Fixed {
        offset: usize,
        width: usize,
        kind: FixedKind,
    },
    MonthName {
        offset: usize,
        long: bool,        // %B vs %b
        term: Option<u8>,  // literal terminator if known (e.g. space, '-', ',', etc.)
    },
}

#[derive(Debug, Clone, Copy)]
enum FixedKind {
    Year4,   // %Y
    Year2,   // %y
    Month2,  // %m
    Day2,    // %d
}

// === Impls ===

impl YmdExtractor {
    #[inline]
    fn variable_offset(&self) -> Option<usize> {
        match self.month {
            FieldExtractor::MonthName { offset, .. } => Some(offset),
            _ => None,
        }
    }
}

pub fn compile_ymd_format(fmt: &str) -> ErrorResult<YmdExtractor> {
    let bytes = fmt.as_bytes();
    let mut i = 0usize;
    let mut out_pos = 0usize;

    let mut year = None;
    let mut month = None;
    let mut day = None;

    let mut literals = Vec::new();

    while i < bytes.len() {
        if bytes[i] == b'%' {
            if i + 1 >= bytes.len() {
                return Err(ErebusError::InvalidOperation("Dangling '%' in format".into()));
            }

            match bytes[i + 1] {
                b'Y' => {
                    ensure_unique(&year, "year")?;
                    year = Some(FieldExtractor::Fixed {
                        offset: out_pos,
                        width: 4,
                        kind: FixedKind::Year4,
                    });
                    out_pos += 4;
                }
                b'y' => {
                    ensure_unique(&year, "year")?;
                    year = Some(FieldExtractor::Fixed {
                        offset: out_pos,
                        width: 2,
                        kind: FixedKind::Year2,
                    });
                    out_pos += 2;
                }
                b'm' => {
                    ensure_unique(&month, "month")?;
                    month = Some(FieldExtractor::Fixed {
                        offset: out_pos,
                        width: 2,
                        kind: FixedKind::Month2,
                    });
                    out_pos += 2;
                }
                b'd' => {
                    ensure_unique(&day, "day")?;
                    day = Some(FieldExtractor::Fixed {
                        offset: out_pos,
                        width: 2,
                        kind: FixedKind::Day2,
                    });
                    out_pos += 2;
                }
                b'b' | b'B' => {
                    ensure_unique(&month, "month")?;

                    // If the format has a literal immediately after %b/%B, use it as terminator.
                    // Example: "%B %d, %Y" => terminator is ' ' (space).
                    let term = bytes.get(i + 2).copied().filter(|c| *c != b'%');

                    month = Some(FieldExtractor::MonthName {
                        offset: out_pos,
                        long: bytes[i + 1] == b'B',
                        term,
                    });

                    // Minimum width: 3 (Jan/Feb/Mar). Long months can be longer.
                    out_pos += 3;
                }
                _ => {
                    return Err(ErebusError::InvalidOperation(
                        format!("Unsupported token: %{}", bytes[i + 1] as char),
                    ));
                }
            }

            i += 2;
        } else {
            literals.push(LiteralCheck {
                idx: out_pos,
                byte: bytes[i],
            });
            out_pos += 1;
            i += 1;
        }
    }

    Ok(YmdExtractor {
        year: year.ok_or_else(|| ErebusError::InvalidOperation("Missing year".into()))?,
        month: month.ok_or_else(|| ErebusError::InvalidOperation("Missing month".into()))?,
        day: day.ok_or_else(|| ErebusError::InvalidOperation("Missing day".into()))?,
        total_len: out_pos,
        literals,
    })
}

fn ensure_unique<T>(slot: &Option<T>, name: &str) -> ErrorResult<()> {
    if slot.is_some() {
        Err(ErebusError::InvalidOperation(format!(
            "Duplicate {} field in format",
            name
        )))
    } else {
        Ok(())
    }
}

pub fn parse_ymd_strict(s: &str, ex: &YmdExtractor) -> ErrorResult<i32> {
    let (y, m, d) = extract_ymd(s, ex).ok_or_else(|| {
        ErebusError::InvalidOperation("Failed to parse date string".into())
    })?;

    validate_ymd_strict(y, m, d)?;
    Ok(ymd_to_days(y, m, d))
}

pub fn parse_ymd_lenient(s: &str, ex: &YmdExtractor) -> Option<i32> {
    let (y, m, d) = extract_ymd(s, ex)?;
    if validate_ymd_lenient(y, m, d) {
        Some(ymd_to_days(y, m, d))
    } else {
        None
    }
}

pub fn parse_ymd_str_vector_lenient(values: &[&str], ex: &YmdExtractor) -> DateVector {
    let n = values.len();
    let mut days = vec![0i32; n];
    let mut validity = bitvec![0; n];

    for i in 0..n {
        if let Some(v) = parse_ymd_lenient(values[i], ex) {
            days[i] = v;
            validity.set(i, true);
        }
    }

    DateVector { days, validity }
}

fn extract_year(b: &[u8], f: &FieldExtractor) -> Option<i32> {
    match *f {
        FieldExtractor::Fixed { offset, kind, .. } => match kind {
            FixedKind::Year4 => parse_i32_4(b, offset),
            FixedKind::Year2 => parse_i32_2(b, offset).map(infer_century),
            _ => None,
        },
        _ => None,
    }
}

fn extract_month(b: &[u8], f: &FieldExtractor) -> Option<u8> {
    match *f {
        FieldExtractor::Fixed { offset, kind, .. } => match kind {
            FixedKind::Month2 => parse_u8_2(b, offset),
            _ => None,
        },
        FieldExtractor::MonthName { offset, long, term } => {
            // Determine end of month token
            let end = match term {
                Some(t) => {
                    let mut j = offset;
                    while j < b.len() && b[j] != t {
                        j += 1;
                    }
                    if j == offset { return None; } // empty
                    j
                }
                None => b.len(),
            };

            let s = std::str::from_utf8(&b[offset..end]).ok()?;
            if long {
                month_from_long(s)
            } else {
                month_from_short(s)
            }
        }
    }
}

fn extract_day(b: &[u8], f: &FieldExtractor) -> Option<u8> {
    match *f {
        FieldExtractor::Fixed { offset, kind, .. } => match kind {
            FixedKind::Day2 => parse_u8_2(b, offset),
            _ => None,
        },
        _ => None,
    }
}

fn extract_ymd(s: &str, ex: &YmdExtractor) -> Option<(i32, u8, u8)> {
    let b = s.as_bytes();

    // still keep minimum length check
    if b.len() < ex.total_len {
        return None;
    }

    // If month is variable-length (%b / %B), compute runtime shift
    let (base_end, shift) = match ex.month {
        FieldExtractor::MonthName { offset, term, .. } => {
            let base_end = offset + 3; // compile-time assumption

            // find actual month token end (terminator or end-of-string)
            let end = match term {
                Some(t) => {
                    let mut j = offset;
                    while j < b.len() && b[j] != t {
                        j += 1;
                    }
                    // month cannot be empty
                    if j == offset { return None; }
                    j
                }
                None => b.len(),
            };

            let actual_len = end.checked_sub(offset)?;
            if actual_len < 3 { return None; }

            let shift = actual_len - 3;
            (base_end, shift)
        }
        _ => (usize::MAX, 0), // no variable field, no shift
    };

    // Validate literals, but adjust those after the month token
    for lit in &ex.literals {
        let idx = adjust_idx(lit.idx, base_end, shift);
        if *b.get(idx)? != lit.byte {
            return None;
        }
    }

    // Extract year/day using adjusted offsets if needed
    let year = extract_year_shifted(b, &ex.year, base_end, shift)?;
    let month = extract_month_shifted(b, &ex.month, base_end, shift)?;
    let day = extract_day_shifted(b, &ex.day, base_end, shift)?;

    Some((year, month, day))
}

#[inline]
fn adjust_idx(idx: usize, base_end: usize, shift: usize) -> usize {
    if idx >= base_end { idx + shift } else { idx }
}

fn extract_year_shifted(b: &[u8], f: &FieldExtractor, base_end: usize, shift: usize) -> Option<i32> {
    match *f {
        FieldExtractor::Fixed { offset, kind, .. } => {
            let off = adjust_idx(offset, base_end, shift);
            match kind {
                FixedKind::Year4 => parse_i32_4(b, off),
                FixedKind::Year2 => parse_i32_2(b, off).map(infer_century),
                _ => None,
            }
        }
        _ => None,
    }
}

fn extract_day_shifted(b: &[u8], f: &FieldExtractor, base_end: usize, shift: usize) -> Option<u8> {
    match *f {
        FieldExtractor::Fixed { offset, kind, .. } => {
            let off = adjust_idx(offset, base_end, shift);
            match kind {
                FixedKind::Day2 => parse_u8_2(b, off),
                _ => None,
            }
        }
        _ => None,
    }
}

fn extract_month_shifted(b: &[u8], f: &FieldExtractor, base_end: usize, shift: usize) -> Option<u8> {
    match *f {
        FieldExtractor::Fixed { offset, kind, .. } => {
            let off = adjust_idx(offset, base_end, shift);
            match kind {
                FixedKind::Month2 => parse_u8_2(b, off),
                _ => None,
            }
        }
        FieldExtractor::MonthName { offset, long, term } => {
            // MonthName offset itself is never shifted: it starts where it starts.
            let end = match term {
                Some(t) => {
                    let mut j = offset;
                    while j < b.len() && b[j] != t {
                        j += 1;
                    }
                    if j == offset { return None; }
                    j
                }
                None => b.len(),
            };

            let s = std::str::from_utf8(&b[offset..end]).ok()?;
            if long { month_from_long(s) } else { month_from_short(s) }
        }
    }
}

#[inline]
fn is_digit(x: u8) -> bool {
    x.wrapping_sub(b'0') <= 9
}

fn parse_u8_2(b: &[u8], off: usize) -> Option<u8> {
    let a = *b.get(off)?;
    let c = *b.get(off + 1)?;
    if !is_digit(a) || !is_digit(c) {
        None
    } else {
        Some((a - b'0') * 10 + (c - b'0'))
    }
}

fn parse_i32_4(b: &[u8], off: usize) -> Option<i32> {
    let mut v = 0i32;
    for i in 0..4 {
        let d = *b.get(off + i)?;
        if !is_digit(d) {
            return None;
        }
        v = v * 10 + (d - b'0') as i32;
    }
    Some(v)
}

fn parse_i32_2(b: &[u8], off: usize) -> Option<i32> {
    let a = *b.get(off)?;
    let c = *b.get(off + 1)?;
    if !is_digit(a) || !is_digit(c) {
        None
    } else {
        Some(((a - b'0') * 10 + (c - b'0')) as i32)
    }
}

#[inline]
fn infer_century(y: i32) -> i32 {
    // Same rule as found here (but 2069 instead of 1969)
    // https://www.ni.com/docs/en-US/bundle/labview-api-ref/page/vi-lib/string/format-codes-for-the-time-format-string.html?srsltid=AfmBOoouO41mwGW1ljmOY9_MU1384ms0W5seat0bSzccYhcVz-iznro4
    // 00–69 → 2000–2069
    // 70–99 → 1970–1999
    if y <= 69 { 2000 + y } else { 1900 + y }
}