// === Imports ===
use erebus_time::prelude::*;

// === Tests ===

#[test]
fn test_date_year() {
    let d = Date::from_ymd(2024, 3, 15).unwrap();
    assert_eq!(d.year(), 2024);
}

#[test]
fn test_date_month() {
    let d = Date::from_ymd(2024, 12, 31).unwrap();
    assert_eq!(d.month(), 12);
}

#[test]
fn test_date_day() {
    let d = Date::from_ymd(2024, 2, 29).unwrap();
    assert_eq!(d.day(), 29);
}

#[test]
fn test_date_ymd() {
    let d = Date::from_ymd(1999, 12, 31).unwrap();
    let (y, m, d2) = d.ymd();
    assert_eq!((y, m, d2), (1999, 12, 31));
}

#[test]
fn test_date_vector_years() {
    let v = DateVector::from_ymd_vectors(
        &[2024, 2023, 2024],
        &[3,    2,    2],
        &[15,   29,   29], // middle is invalid
    ).unwrap();

    let years = v.years();

    assert_eq!(years, vec![2024, 0, 2024]);
}

#[test]
fn test_date_vector_months() {
    let v = DateVector::from_ymd_vectors(
        &[2024, 2023, 2024],
        &[3,    13,   2], // invalid month in middle
        &[15,   10,   29],
    ).unwrap();

    let months = v.months();

    assert_eq!(months, vec![3, 0, 2]);
}

#[test]
fn test_date_vector_days() {
    let v = DateVector::from_ymd_vectors(
        &[2024, 2023, 2024],
        &[2,    2,    2],
        &[29,   29,   28], // middle invalid
    ).unwrap();

    let days = v.days();

    assert_eq!(days, vec![29, 0, 28]);
}

#[test]
fn test_date_vector_ymd() {
    let v = DateVector::from_ymd_vectors(
        &[2024, 2023, 1999],
        &[3,    2,    12],
        &[15,   29,   31], // middle invalid
    ).unwrap();

    let ymd = v.ymd();

    assert_eq!(
        ymd,
        vec![
            (2024, 3, 15),
            (0,    0, 0),
            (1999, 12, 31),
        ]
    );
}