// === Imports ===
use erebus_time::prelude::*;

// === Tests ===

#[test]
fn test_date_is_weekend_saturday_sunday() {
    let fri = Date::from_ymd(2024, 3, 15).unwrap();
    let sat = Date::from_ymd(2024, 3, 16).unwrap();
    let sun = Date::from_ymd(2024, 3, 17).unwrap();

    let rule = WeekendRule::SaturdaySunday;

    assert!(!fri.is_weekend(rule));
    assert!(sat.is_weekend(rule));
    assert!(sun.is_weekend(rule));
}

#[test]
fn test_date_is_weekend_friday_saturday() {
    let fri = Date::from_ymd(2024, 3, 15).unwrap();
    let sat = Date::from_ymd(2024, 3, 16).unwrap();
    let sun = Date::from_ymd(2024, 3, 17).unwrap();

    let rule = WeekendRule::FridaySaturday;

    assert!(fri.is_weekend(rule));
    assert!(sat.is_weekend(rule));
    assert!(!sun.is_weekend(rule));
}

#[test]
fn test_date_is_weekend_custom_rule() {
    let fri = Date::from_ymd(2024, 3, 15).unwrap();
    let sat = Date::from_ymd(2024, 3, 16).unwrap();
    let sun = Date::from_ymd(2024, 3, 17).unwrap();

    // Only Sunday is weekend
    let rule = WeekendRule::Custom {
        fri: false,
        sat: false,
        sun: true,
    };

    assert!(!fri.is_weekend(rule));
    assert!(!sat.is_weekend(rule));
    assert!(sun.is_weekend(rule));
}

#[test]
fn test_date_is_weekend_rules() {
    let fri = Date::from_ymd(2024, 3, 15).unwrap(); // Friday
    let sat = Date::from_ymd(2024, 3, 16).unwrap(); // Saturday
    let sun = Date::from_ymd(2024, 3, 17).unwrap(); // Sunday

    // Saturday + Sunday
    let rule = WeekendRule::SaturdaySunday;
    assert!(!fri.is_weekend(rule));
    assert!(sat.is_weekend(rule));
    assert!(sun.is_weekend(rule));

    // Friday + Saturday
    let rule = WeekendRule::FridaySaturday;
    assert!(fri.is_weekend(rule));
    assert!(sat.is_weekend(rule));
    assert!(!sun.is_weekend(rule));

    // Custom: only Sunday
    let rule = WeekendRule::Custom {
        fri: false,
        sat: false,
        sun: true,
    };
    assert!(!fri.is_weekend(rule));
    assert!(!sat.is_weekend(rule));
    assert!(sun.is_weekend(rule));
}

#[test]
fn test_date_is_business_day_custom_market_calendar() {
    let holiday = Date::from_ymd(2024, 7, 4).unwrap();

    let cal = MarketCalendar::new(
        WeekendRule::SaturdaySunday,
        HolidaySet::new(vec![holiday.days()]),
    );

    let weekday = Date::from_ymd(2024, 7, 1).unwrap(); // Monday
    let weekend = Date::from_ymd(2024, 7, 6).unwrap(); // Saturday
    let holiday = Date::from_ymd(2024, 7, 4).unwrap(); // Holiday

    assert!(weekday.is_business_day(&cal));
    assert!(!weekend.is_business_day(&cal));
    assert!(!holiday.is_business_day(&cal));
}

#[test]
fn test_date_is_business_day_nyse_calendar() {
    let nyse = get_nyse_calendar().unwrap();;

    let trading_day = Date::from_ymd(2024, 3, 11).unwrap(); // Monday
    let weekend = Date::from_ymd(2024, 3, 16).unwrap();     // Saturday
    let christmas = Date::from_ymd(2024, 12, 25).unwrap(); // NYSE holiday

    assert!(trading_day.is_business_day(&nyse));
    assert!(!weekend.is_business_day(&nyse));
    assert!(!christmas.is_business_day(&nyse));
}

#[test]
fn test_date_vector_is_weekend_all_rules() {
    // Dates:
    // 2024-03-15 = Friday
    // 2024-03-16 = Saturday
    // 2024-03-17 = Sunday
    let dv = DateVector::from_ymd_vectors(
        &[2024, 2024, 2024],
        &[3, 3, 3],
        &[15, 16, 17],
    ).unwrap();

    // Saturday + Sunday
    let sat_sun = dv.is_weekend(WeekendRule::SaturdaySunday);
    assert_eq!(sat_sun, bitvec![0, 1, 1]);

    // Friday + Saturday
    let fri_sat = dv.is_weekend(WeekendRule::FridaySaturday);
    assert_eq!(fri_sat, bitvec![1, 1, 0]);

    // Custom: Sunday only
    let custom = dv.is_weekend(WeekendRule::Custom {
        fri: false,
        sat: false,
        sun: true,
    });
    assert_eq!(custom, bitvec![0, 0, 1]);
}

#[test]
fn test_date_vector_is_business_day_nyse() {
    let nyse = get_nyse_calendar().unwrap();

    // Dates:
    // 2024-03-11 = Monday (trading day)
    // 2024-03-16 = Saturday (weekend)
    // 2024-12-25 = Christmas (NYSE holiday)
    // 2024-03-12 = Tuesday (trading day)
    let dv = DateVector::from_ymd_vectors(
        &[2024, 2024, 2024, 2024],
        &[3,    3,    12,   3],
        &[11,   16,   25,   12],
    ).unwrap();

    let out = dv.is_business_day(&nyse);

    assert_eq!(out, bitvec![1, 0, 0, 1]);
}

#[test]
fn test_date_vector_is_business_day_with_nulls() {
    let nyse = get_nyse_calendar().unwrap();

    let dv = DateVector::from_ymd_vectors(
        &[2024, 2024, 2024],
        &[2,    2,    3],
        &[29,   30,   11], // Feb 30 invalid
    ).unwrap();

    let out = dv.is_business_day(&nyse);

    // 2024-02-29 valid (Thu)
    // 2024-02-30 invalid → null
    // 2024-03-11 valid (Mon)
    assert_eq!(out, bitvec![1, 0, 1]);
}

#[test]
fn test_date_vector_is_business_day_custom_calendar() {
    // Custom holidays:
    // 2024-03-13 (Wednesday)
    let holidays = HolidaySet::new(vec![
        ymd_to_days(2024, 3, 13),
    ]);

    // Custom calendar:
    // - Weekend = Friday + Saturday
    let cal = MarketCalendar::new(
        WeekendRule::FridaySaturday,
        holidays,
    );

    // Dates:
    // 2024-03-11 = Monday (business)
    // 2024-03-13 = Wednesday (holiday)
    // 2024-03-15 = Friday (weekend under rule)
    // 2024-03-17 = Sunday (business under rule)
    let dv = DateVector::from_ymd_vectors(
        &[2024, 2024, 2024, 2024],
        &[3,    3,    3,    3],
        &[11,   13,   15,   17],
    ).unwrap();

    let out = dv.is_business_day(&cal);

    assert_eq!(out, bitvec![1, 0, 0, 1]);
}

#[test]
fn test_date_shift_business_days_forward() {
    let cal = get_nyse_calendar().unwrap();

    // Monday → Tuesday
    let d = Date::from_ymd(2024, 3, 11).unwrap(); // Mon
    let shifted = d.shift_business_days(&cal, 1);

    assert_eq!(shifted, Date::from_ymd(2024, 3, 12).unwrap());
}

#[test]
fn test_date_shift_business_days_skip_weekend() {
    let cal = get_nyse_calendar().unwrap();

    // Friday → Monday
    let d = Date::from_ymd(2024, 3, 15).unwrap(); // Fri
    let shifted = d.shift_business_days(&cal, 1);

    assert_eq!(shifted, Date::from_ymd(2024, 3, 18).unwrap());
}

#[test]
fn test_date_shift_business_days_skip_holiday() {
    let cal = get_nyse_calendar().unwrap();

    // Christmas 2024 is a holiday
    let d = Date::from_ymd(2024, 12, 24).unwrap(); // Tue
    let shifted = d.shift_business_days(&cal, 1);

    // Skip 25th → 26th
    assert_eq!(shifted, Date::from_ymd(2024, 12, 26).unwrap());
}

#[test]
fn test_date_shift_business_days_backward() {
    let cal = get_nyse_calendar().unwrap();

    // Monday → Friday
    let d = Date::from_ymd(2024, 3, 18).unwrap(); // Mon
    let shifted = d.shift_business_days(&cal, -1);

    assert_eq!(shifted, Date::from_ymd(2024, 3, 15).unwrap());
}

#[test]
fn test_date_shift_business_days_zero() {
    let cal = get_nyse_calendar().unwrap();

    let d = Date::from_ymd(2024, 3, 11).unwrap();
    let shifted = d.shift_business_days(&cal, 0);

    assert_eq!(shifted, d);
}

#[test]
fn test_date_next_business_day() {
    let cal = get_nyse_calendar().unwrap();

    let d = Date::from_ymd(2024, 3, 15).unwrap(); // Fri
    let next = d.next_business_day(&cal);

    assert_eq!(next, Date::from_ymd(2024, 3, 18).unwrap());
}

#[test]
fn test_date_previous_business_day() {
    let cal = get_nyse_calendar().unwrap();

    let d = Date::from_ymd(2024, 3, 18).unwrap(); // Mon
    let prev = d.previous_business_day(&cal);

    assert_eq!(prev, Date::from_ymd(2024, 3, 15).unwrap());
}

#[test]
fn test_date_vector_shift_business_days_forward() {
    let cal = get_nyse_calendar().unwrap();

    // Fri → Mon, Mon → Tue
    let dv = DateVector::from_ymd_vectors(
        &[2024, 2024],
        &[3,    3],
        &[15,   11], // Fri, Mon
    ).unwrap();

    let out = dv.shift_business_days(&cal, 1);

    assert_eq!(
        out.ymd(),
        vec![
            (2024, 3, 18), // Fri → Mon
            (2024, 3, 12), // Mon → Tue
        ]
    );
}

#[test]
fn test_date_vector_shift_business_days_backward() {
    let cal = get_nyse_calendar().unwrap();

    // Mon → Fri, Tue → Mon
    let dv = DateVector::from_ymd_vectors(
        &[2024, 2024],
        &[3,    3],
        &[18,   12], // Mon, Tue
    ).unwrap();

    let out = dv.shift_business_days(&cal, -1);

    assert_eq!(
        out.ymd(),
        vec![
            (2024, 3, 15),
            (2024, 3, 11),
        ]
    );
}

#[test]
fn test_date_vector_next_business_day() {
    let cal = get_nyse_calendar().unwrap();

    let dv = DateVector::from_ymd_vectors(
        &[2024],
        &[3],
        &[15], // Friday
    ).unwrap();

    let out = dv.next_business_day(&cal);

    assert_eq!(out.ymd(), vec![(2024, 3, 18)]);
}

#[test]
fn test_date_vector_previous_business_day() {
    let cal = get_nyse_calendar().unwrap();

    let dv = DateVector::from_ymd_vectors(
        &[2024],
        &[3],
        &[18], // Monday
    ).unwrap();

    let out = dv.previous_business_day(&cal);

    assert_eq!(out.ymd(), vec![(2024, 3, 15)]);
}

#[test]
fn test_date_vector_count_business_days() {
    let cal = get_nyse_calendar().unwrap();

    // Mon, Sat, Christmas, Tue
    let dv = DateVector::from_ymd_vectors(
        &[2024, 2024, 2024, 2024],
        &[3,    3,    12,   3],
        &[11,   16,   25,   12],
    ).unwrap();

    let count = dv.count_business_days(&cal);

    // Only 11th (Mon) and 12th (Tue)
    assert_eq!(count, 2);
}

#[test]
fn test_date_vector_count_weekends() {
    let dv = DateVector::from_ymd_vectors(
        &[2024, 2024, 2024],
        &[3,    3,    3],
        &[15,   16,   17], // Fri, Sat, Sun
    ).unwrap();

    let count = dv.count_weekends(WeekendRule::SaturdaySunday);

    assert_eq!(count, 2);
}

#[test]
fn test_date_vector_count_holidays_nyse() {
    let cal = get_nyse_calendar().unwrap();

    let dv = DateVector::from_ymd_vectors(
        &[2024, 2024, 2024],
        &[12,   12,   3],
        &[25,   26,   11], // Christmas, next day, random weekday
    ).unwrap();

    let count = dv.count_holidays(&cal);

    assert_eq!(count, 1);
}

#[test]
fn test_date_vector_shift_business_days_with_nulls() {
    let cal = get_nyse_calendar().unwrap();

    let dv = DateVector::from_ymd_vectors(
        &[2024, 2024],
        &[2,    2],
        &[29,   30], // Feb 30 invalid
    ).unwrap();

    let out = dv.shift_business_days(&cal, 1);

    assert_eq!(
        out.ymd(),
        vec![
            (2024, 3, 1), // Feb 29 → Mar 1
            (0, 0, 0),    // null preserved
        ]
    );
}