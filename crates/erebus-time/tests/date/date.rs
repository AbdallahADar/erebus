// === Imports ===
use erebus_time::prelude::*;

// === Tests ===

#[test]
fn test_date_from_ymd_basic() {
    let d = Date::from_ymd(2024, 3, 15).unwrap();
    assert_eq!(d.days(), ymd_to_days(2024, 3, 15));
}

#[test]
fn test_date_from_ymd_leap_day() {
    let d = Date::from_ymd(2024, 2, 29).unwrap();
    assert_eq!(d.days(), ymd_to_days(2024, 2, 29));
}

#[test]
fn test_date_from_ymd_invalid_month() {
    let err = Date::from_ymd(2024, 13, 1).unwrap_err();
    match err {
        ErebusError::InvalidMonth(month) => assert_eq!(month, 13),
        other => panic!("Unexpected error: {:?}", other),
    }
}

#[test]
fn test_date_from_ymd_invalid_day() {
    let err = Date::from_ymd(2023, 2, 29).unwrap_err();

    match err {
        ErebusError::InvalidDay { year, month, day } => {
            assert_eq!(year, 2023);
            assert_eq!(month, 2);
            assert_eq!(day, 29);
        }
        other => panic!("Unexpected error: {:?}", other),
    }
}

#[test]
fn test_date_from_ymd_str_numeric() {
    let d = Date::from_ymd_str("2024-03-15", "%Y-%m-%d").unwrap();
    assert_eq!(d.days(), ymd_to_days(2024, 3, 15));
}

#[test]
fn test_date_from_ymd_str_numeric_short() {
    let d = Date::from_ymd_str("24-03-15", "%y-%m-%d").unwrap();
    assert_eq!(d.days(), ymd_to_days(2024, 3, 15));
}

#[test]
fn test_date_from_ymd_str_numeric_short_90s() {
    let d = Date::from_ymd_str("70-03-15", "%y-%m-%d").unwrap();
    assert_eq!(d.days(), ymd_to_days(1970, 3, 15));
}

#[test]
fn test_date_from_ymd_str_short_month() {
    let d = Date::from_ymd_str("15-Mar-2024", "%d-%b-%Y").unwrap();
    assert_eq!(d.days(), ymd_to_days(2024, 3, 15));
}

#[test]
fn test_date_from_ymd_str_long_month() {
    let d = Date::from_ymd_str("March 15, 2024", "%B %d, %Y").unwrap();
    assert_eq!(d.days(), ymd_to_days(2024, 3, 15));
}

#[test]
fn test_date_from_ymd_str_long_month_hyphen() {
    let d = Date::from_ymd_str("March-15-2024", "%B-%d-%Y").unwrap();
    assert_eq!(d.days(), ymd_to_days(2024, 3, 15));
}

#[test]
fn test_date_from_ymd_str_long_month_slash() {
    let d = Date::from_ymd_str("March/15/2024", "%B/%d/%Y").unwrap();
    assert_eq!(d.days(), ymd_to_days(2024, 3, 15));
}

#[test]
fn test_date_from_ymd_str_long_month_at_start() {
    let d = Date::from_ymd_str("September 07, 1999", "%B %d, %Y").unwrap();
    assert_eq!(d.days(), ymd_to_days(1999, 9, 7));
}

#[test]
fn test_date_from_ymd_str_long_month_middle() {
    let d = Date::from_ymd_str("15 September 1999", "%d %B %Y").unwrap();
    assert_eq!(d.days(), ymd_to_days(1999, 9, 15));
}

#[test]
fn test_date_from_ymd_str_long_month_at_end() {
    let d = Date::from_ymd_str("15 2024 March", "%d %Y %B").unwrap();
    assert_eq!(d.days(), ymd_to_days(2024, 3, 15));
}

#[test]
fn test_date_from_ymd_str_invalid_format() {
    let err = Date::from_ymd_str("2024/03/15", "%Y-%m-%d").unwrap_err();
    match err {
        ErebusError::InvalidOperation(_) => {}
        other => panic!("Unexpected error: {:?}", other),
    }
}

#[test]
fn test_date_from_ymd_str_long_month_invalid_name() {
    let err = Date::from_ymd_str("Marhc 15, 2024", "%B %d, %Y").unwrap_err();
    match err {
        ErebusError::InvalidOperation(_) => {}
        other => panic!("Unexpected error: {:?}", other),
    }
}

#[test]
fn test_date_from_ymd_str_long_month_offset_shift_regression() {
    // Month length = 5, compile-time assumed = 3 → shift = 2
    let d = Date::from_ymd_str("March 01, 2000", "%B %d, %Y").unwrap();
    assert_eq!(d.days(), ymd_to_days(2000, 3, 1));
}

#[test]
fn test_date_from_ymd_str_long_month_invalid_day() {
    let err = Date::from_ymd_str("February 29, 2023", "%B %d, %Y").unwrap_err();
    match err {
        ErebusError::InvalidDay { year, month, day } => {
            assert_eq!(year, 2023);
            assert_eq!(month, 2);
            assert_eq!(day, 29);
        }
        other => panic!("Unexpected error: {:?}", other),
    }
}

#[test]
fn test_date_from_ymd_str_invalid_date() {
    let err = Date::from_ymd_str("2023-02-29", "%Y-%m-%d").unwrap_err();

    match err {
        ErebusError::InvalidDay { year, month, day } => {
            assert_eq!(year, 2023);
            assert_eq!(month, 2);
            assert_eq!(day, 29);
        }
        other => panic!("Unexpected error: {:?}", other),
    }
}

#[test]
fn test_date_from_ymd_str_long_month_at_end_invalid() {
    let err = Date::from_ymd_str("15 2024 Marhc", "%d %Y %B").unwrap_err();
    match err {
        ErebusError::InvalidOperation(_) => {}
        other => panic!("Unexpected error: {:?}", other),
    }
}

#[test]
fn test_date_days_accessor() {
    let d = Date::from_ymd(1999, 12, 31).unwrap();
    let days = d.days();

    let (y, m, day) = days_to_ymd(days);
    assert_eq!((y, m, day), (1999, 12, 31));
}

#[test]
fn test_date_pre_epoch() {
    let d = Date::from_ymd(1900, 1, 1).unwrap();
    let (y, m, day) = days_to_ymd(d.days());
    assert_eq!((y, m, day), (1900, 1, 1));
}

#[test]
fn test_date_vector_len() {
    let v = DateVector::from_ymd_vectors(
        &[2024, 2024],
        &[3, 3],
        &[15, 16],
    ).unwrap();

    assert_eq!(v.len(), 2);
}

#[test]
fn test_date_vector_is_empty_true() {
    let v = DateVector::from_ymd_vectors(&[], &[], &[]).unwrap();
    assert!(v.is_empty());
}

#[test]
fn test_date_vector_is_empty_false() {
    let v = DateVector::from_ymd_vectors(
        &[2024],
        &[3],
        &[15],
    ).unwrap();

    assert!(!v.is_empty());
}

#[test]
fn test_date_vector_from_ymd_vectors_all_valid() {
    let v = DateVector::from_ymd_vectors(
        &[2024, 2023],
        &[3, 12],
        &[15, 31],
    ).unwrap();

    assert_eq!(v.len(), 2);

    assert_eq!(v.get(0).unwrap().days(), ymd_to_days(2024, 3, 15));
    assert_eq!(v.get(1).unwrap().days(), ymd_to_days(2023, 12, 31));
}

#[test]
fn test_date_vector_from_ymd_vectors_mixed_validity() {
    let v = DateVector::from_ymd_vectors(
        &[2024, 2023, 2023],
        &[2, 2, 13],
        &[29, 29, 1],
    ).unwrap();

    // 2024-02-29 → valid
    assert!(v.get(0).is_some());
    assert_eq!(v.get(0).unwrap().days(), ymd_to_days(2024, 2, 29));

    // 2023-02-29 → invalid
    assert!(v.get(1).is_none());

    // invalid month
    assert!(v.get(2).is_none());
}

#[test]
fn test_date_vector_from_ymd_vectors_length_mismatch() {
    let err = DateVector::from_ymd_vectors(
        &[2024, 2023],
        &[3],
        &[15, 16],
    ).unwrap_err();

    match err {
        ErebusError::LengthMismatch { expected, found } => {
            assert_eq!(expected, 2);
            assert_eq!(found, 1);
        }
        other => panic!("Unexpected error: {:?}", other),
    }
}

#[test]
fn test_date_vector_from_ymd_str_vectors_numeric() {
    let v = DateVector::from_ymd_str_vectors(
        &["2024-03-15", "2023-12-31"],
        "%Y-%m-%d",
    ).unwrap();

    assert_eq!(v.len(), 2);
    assert_eq!(v.get(0).unwrap().days(), ymd_to_days(2024, 3, 15));
    assert_eq!(v.get(1).unwrap().days(), ymd_to_days(2023, 12, 31));
}

#[test]
fn test_date_vector_from_ymd_str_vectors_mixed_validity() {
    let v = DateVector::from_ymd_str_vectors(
        &["2024-02-29", "2023-02-29", "bad-date"],
        "%Y-%m-%d",
    ).unwrap();

    assert!(v.get(0).is_some()); // leap day valid
    assert!(v.get(1).is_none()); // invalid date
    assert!(v.get(2).is_none()); // invalid string
}

#[test]
fn test_date_vector_from_ymd_str_vectors_long_month() {
    let v = DateVector::from_ymd_str_vectors(
        &["March 15, 2024", "February 29, 2024", "February 29, 2023"],
        "%B %d, %Y",
    ).unwrap();

    assert_eq!(v.len(), 3);

    assert!(v.get(0).is_some());
    assert!(v.get(1).is_some());
    assert!(v.get(2).is_none());

    assert_eq!(v.get(0).unwrap().days(), ymd_to_days(2024, 3, 15));
    assert_eq!(v.get(1).unwrap().days(), ymd_to_days(2024, 2, 29));
}

#[test]
fn test_date_vector_from_ymd_str_vectors_invalid_format() {
    let err = DateVector::from_ymd_str_vectors(
        &["2024-03-15"],
        "%Y-%Q-%d", // unsupported token
    ).unwrap_err();

    match err {
        ErebusError::InvalidOperation(_) => {}
        other => panic!("Unexpected error: {:?}", other),
    }
}

#[test]
fn test_date_vector_from_ymd_str_vectors_comprehensive() {
    let values = [
        "2024-03-15",        // valid numeric
        "2023-02-29",        // invalid (non-leap)
        "2024-02-29",        // valid leap day
        "bad-date",          // invalid format
        "March 01, 2000",    // valid long month
        "February 29, 2024", // valid long month leap day
        "February 29, 2023", // invalid long month
        "15-Mar-2024",       // valid short month
        "15-Mar-2023",       // valid short month
        "15-Mar-202x",      // invalid trailing char
        "Jan 32, 2024",      // invalid day
        "Apr 31, 2024",      // invalid day
        "December 31, 1999", // valid long month end of year
        "00-00-0000",        // invalid numeric
        "2024-13-01",        // invalid month
    ];

    let formats = [
        "%Y-%m-%d",
        "%Y-%m-%d",
        "%Y-%m-%d",
        "%Y-%m-%d",
        "%B %d, %Y",
        "%B %d, %Y",
        "%B %d, %Y",
        "%d-%b-%Y",
        "%d-%b-%Y",
        "%d-%b-%Y",
        "%b %d, %Y",
        "%b %d, %Y",
        "%B %d, %Y",
        "%m-%d-%Y",
        "%Y-%m-%d",
    ];

    let mut out = Vec::new();

    for i in 0..values.len() {
        out.push((values[i], formats[i]));
    }

    // Build vectors per-format (group by format)
    let mut all_dates = Vec::new();
    let mut all_expected = Vec::new();

    for (s, fmt) in out {
        let v = DateVector::from_ymd_str_vectors(&[s], fmt).unwrap();
        all_dates.push(v.get(0));
        all_expected.push(match s {
            "2024-03-15"        => Some(ymd_to_days(2024, 3, 15)),
            "2024-02-29"        => Some(ymd_to_days(2024, 2, 29)),
            "March 01, 2000"    => Some(ymd_to_days(2000, 3, 1)),
            "February 29, 2024" => Some(ymd_to_days(2024, 2, 29)),
            "15-Mar-2024"       => Some(ymd_to_days(2024, 3, 15)),
            "15-Mar-2023"       => Some(ymd_to_days(2023, 3, 15)),
            "December 31, 1999" => Some(ymd_to_days(1999, 12, 31)),
            _ => None,
        });
    }

    assert_eq!(all_dates.len(), all_expected.len());

    for i in 0..all_dates.len() {
        match (&all_dates[i], &all_expected[i]) {
            (Some(d), Some(expected)) => assert_eq!(d.days(), *expected),
            (None, None) => {}
            other => panic!("Mismatch at index {}: {:?}", i, other),
        }
    }
}