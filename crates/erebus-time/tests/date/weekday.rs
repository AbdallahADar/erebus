// === Imports ===
use erebus_time::prelude::*;

// === Tests ===

fn dv_from_ymd(values: &[(i32, u8, u8)]) -> DateVector {
    let years: Vec<i32> = values.iter().map(|x| x.0).collect();
    let months: Vec<u8> = values.iter().map(|x| x.1).collect();
    let days: Vec<u8> = values.iter().map(|x| x.2).collect();
    DateVector::from_ymd_vectors(&years, &months, &days).unwrap()
}

#[test]
fn test_date_weekday_iso_basic() {
    // 1970-01-01 = Thursday
    let d = Date::from_ymd(1970, 1, 1).unwrap();
    assert_eq!(d.weekday(), 4);

    // 1970-01-05 = Monday
    let d = Date::from_ymd(1970, 1, 5).unwrap();
    assert_eq!(d.weekday(), 1);

    // 1970-01-06 = Tuesday
    let d = Date::from_ymd(1970, 1, 6).unwrap();
    assert_eq!(d.weekday(), 2);

    // 1970-01-11 = Sunday
    let d = Date::from_ymd(1970, 1, 11).unwrap();
    assert_eq!(d.weekday(), 7);
}

#[test]
fn test_date_weekday_with_iso() {
    let d = Date::from_ymd(2024, 3, 18).unwrap(); // Monday
    assert_eq!(d.weekday(), 1);
    assert_eq!(d.weekday_with(WeekConvention::ISO), 1);

    let d = Date::from_ymd(2024, 3, 17).unwrap(); // Sunday
    assert_eq!(d.weekday(), 7);
    assert_eq!(d.weekday_with(WeekConvention::ISO), 7);
}

#[test]
fn test_date_weekday_with_usa() {
    let mon = Date::from_ymd(2024, 3, 18).unwrap(); // Monday
    let sun = Date::from_ymd(2024, 3, 17).unwrap(); // Sunday
    let sat = Date::from_ymd(2024, 3, 16).unwrap(); // Saturday

    // ISO weekdays
    assert_eq!(mon.weekday(), 1);
    assert_eq!(sun.weekday(), 7);
    assert_eq!(sat.weekday(), 6);

    // USA mapping
    assert_eq!(mon.weekday_with(WeekConvention::US), 2);
    assert_eq!(sun.weekday_with(WeekConvention::US), 1);
    assert_eq!(sat.weekday_with(WeekConvention::US), 7);
}

#[test]
fn test_date_weekday_with_custom() {
    // Week starts on Wednesday (ISO=3)
    let conv = WeekConvention::Custom { first_day: 3 };

    let wed = Date::from_ymd(2024, 3, 20).unwrap(); // Wednesday
    let thu = Date::from_ymd(2024, 3, 21).unwrap(); // Thursday
    let tue = Date::from_ymd(2024, 3, 19).unwrap(); // Tuesday

    // ISO
    assert_eq!(wed.weekday(), 3);
    assert_eq!(thu.weekday(), 4);
    assert_eq!(tue.weekday(), 2);

    // Custom mapping
    assert_eq!(wed.weekday_with(conv), 1);
    assert_eq!(thu.weekday_with(conv), 2);
    assert_eq!(tue.weekday_with(conv), 7);
}

#[test]
fn test_datevector_weekdays_iso() {
    // 2024-03-11..17 is Mon..Sun
    let dv = dv_from_ymd(&[
        (2024, 3, 11),
        (2024, 3, 12),
        (2024, 3, 13),
        (2024, 3, 14),
        (2024, 3, 15),
        (2024, 3, 16),
        (2024, 3, 17),
    ]);

    let w = dv.weekdays();
    assert_eq!(w, vec![1, 2, 3, 4, 5, 6, 7]);
}

#[test]
fn test_datevector_weekdays_with_iso_is_identity() {
    let dv = dv_from_ymd(&[
        (2024, 3, 11), // Mon
        (2024, 3, 17), // Sun
    ]);

    let iso = dv.weekdays();
    let mapped = dv.weekdays_with(WeekConvention::ISO);
    assert_eq!(mapped, iso);
}

#[test]
fn test_datevector_weekdays_with_us() {
    let dv = dv_from_ymd(&[
        (2024, 3, 11), // Mon (ISO=1)
        (2024, 3, 16), // Sat (ISO=6)
        (2024, 3, 17), // Sun (ISO=7)
    ]);

    let mapped = dv.weekdays_with(WeekConvention::US);

    // US: Sun=1, Mon=2, ... Sat=7
    assert_eq!(mapped, vec![2, 7, 1]);
}

#[test]
fn test_datevector_weekdays_with_custom_first_day_wed() {
    // Custom: week starts on Wednesday (ISO=3)
    let conv = WeekConvention::Custom { first_day: 3 };

    let dv = dv_from_ymd(&[
        (2024, 3, 20), // Wed (ISO=3)
        (2024, 3, 21), // Thu (ISO=4)
        (2024, 3, 19), // Tue (ISO=2)
    ]);

    let iso = dv.weekdays();
    assert_eq!(iso, vec![3, 4, 2]);

    let mapped = dv.weekdays_with(conv);

    // With first_day=3:
    // Wed(3)->1, Thu(4)->2, Tue(2)->7
    assert_eq!(mapped, vec![1, 2, 7]);
}

#[test]
fn test_datevector_weekdays_with_custom_first_day_sun_matches_us() {
    // If your WeekConvention::US is implemented as first_day=7 mapping,
    // then Custom{first_day:7} should match US weekday mapping.
    // (This is a strong consistency check.)
    let dv = dv_from_ymd(&[
        (2024, 3, 11), // Mon
        (2024, 3, 16), // Sat
        (2024, 3, 17), // Sun
    ]);

    let us = dv.weekdays_with(WeekConvention::US);
    let custom = dv.weekdays_with(WeekConvention::Custom { first_day: 7 });

    assert_eq!(custom, us);
}

#[test]
fn test_datevector_weekdays_with_nulls() {
    // middle element invalid: 2024-02-30
    let dv = DateVector::from_ymd_vectors(
        &[2024, 2024, 2024],
        &[3, 2, 3],
        &[11, 30, 12],
    )
    .unwrap();

    let iso = dv.weekdays();
    let us = dv.weekdays_with(WeekConvention::US);
    let custom = dv.weekdays_with(WeekConvention::Custom { first_day: 3 });

    // Null entries should remain 0
    assert_eq!(iso[1], 0);
    assert_eq!(us[1], 0);
    assert_eq!(custom[1], 0);

    // The valid ones should be correct:
    // 2024-03-11 is Monday -> ISO=1, US=2, custom(first_day=Wed)=? Monday should map to 6
    assert_eq!(iso[0], 1);
    assert_eq!(us[0], 2);
    assert_eq!(custom[0], 6);

    // 2024-03-12 is Tuesday -> ISO=2, US=3, custom(first_day=Wed)=7
    assert_eq!(iso[2], 2);
    assert_eq!(us[2], 3);
    assert_eq!(custom[2], 7);
}

#[test]
fn test_date_vector_count_weekdays() {
    // Dates:
    // 2024-03-11 = Monday
    // 2024-03-12 = Tuesday
    // 2024-03-16 = Saturday
    // 2024-03-17 = Sunday
    let dv = DateVector::from_ymd_vectors(
        &[2024, 2024, 2024, 2024],
        &[3,    3,    3,    3],
        &[11,   12,   16,   17],
    ).unwrap();

    let count = dv.count_weekdays(WeekendRule::SaturdaySunday);

    // Only Monday + Tuesday
    assert_eq!(count, 2);
}

#[test]
fn test_date_vector_count_weekdays_with_nulls() {
    let dv = DateVector::from_ymd_vectors(
        &[2024, 2024, 2024],
        &[2,    2,    3],
        &[29,   30,   11], // Feb 30 invalid
    ).unwrap();

    let count = dv.count_weekdays(WeekendRule::SaturdaySunday);

    // 2024-02-29 (Thu) + 2024-03-11 (Mon)
    assert_eq!(count, 2);
}