// === Imports ===
use erebus_io::prelude::*;
use std::fs::File;
use std::io::{Seek, SeekFrom};
use rand::Rng;

// === Tests ===

#[test]
fn test_vectordata_f64_factored_roundtrip() {
    // ---------------------------------------------------------------
    // 1. Input data
    // ---------------------------------------------------------------
    let values = vec![
        0.0,
        1.5,
        -2.25,
        f64::INFINITY,
        f64::NEG_INFINITY,
        1e-310,
        42.0,
    ];

    let mut validity = BitVec::new();
    validity.extend([
        true,   // 0.0
        true,   // 1.5
        false,  // becomes default()
        true,   // +inf
        true,   // -inf
        true,   // subnormal-ish
        false,  // becomes default()
    ]);

    let expected: Vec<f64> = vec![
        0.0,
        1.5,
        f64::default(),
        f64::INFINITY,
        f64::NEG_INFINITY,
        1e-310,
        f64::default(),
    ];

    let path = "tests/vector_data/f64/output/factored_test1.erebus";

    // ---------------------------------------------------------------
    // 2. Write the file
    // ---------------------------------------------------------------
    {
        let file = File::create(path).unwrap();
        let mut writer = ErebusWriter::new(file);
        writer
            .vector_data()
            .f64()
            .factored()
            .write(&values, &validity);
    }

    // ---------------------------------------------------------------
    // 3. Read the file
    // ---------------------------------------------------------------
    let (read_vals, read_validity) = {
        let file = File::open(path).expect("failed to open file");
        let mut reader = ErebusReader::new(file);

        reader
            .vector_data()
            .f64()
            .factored()
            .read()
            .expect("read failed")
    };

    // ---------------------------------------------------------------
    // 4. Debug printing
    // ---------------------------------------------------------------
    println!("Input values:   {:?}", values);
    println!("Input validity: {:?}", validity);
    println!("Read values:    {:?}", read_vals);
    println!("Read validity:  {:?}", read_validity);
    println!("Expected vals:  {:?}", expected);

    // ---------------------------------------------------------------
    // 5. Assertions
    // ---------------------------------------------------------------

    // Validity must match exactly.
    assert_eq!(read_validity, validity, "Validity bitvec mismatch");

    // Value-by-value bitwise comparison.
    for i in 0..expected.len() {
        assert!(
            read_vals[i].to_bits() == expected[i].to_bits(),
            "Value mismatch at index {}: got {:?}, expected {:?}",
            i,
            read_vals[i],
            expected[i]
        );
    }
}

#[test]
fn test_vectordata_f64_factored_high_precision() {
    // -----------------------------------------------------------
    // 1. High-precision, difficult IEEE-754 test values
    // -----------------------------------------------------------
    let values: Vec<f64> = vec![
        1.234567890123456,               // long mantissa
        -9.876543210987654,              // long mantissa, negative
        1.0e308,                         // near max normal
        1.0e-308,                        // near min normal
        2.2250738585072014e-308,         // smallest positive normal
        5e-324,                           // smallest subnormal > 0
        -5e-324,                          // smallest subnormal < 0
        3.141592653589793,               // pi
        -2.718281828459045,              // -e
        0.30000000000000004,             // floating point troublemaker
        4503599627370495.0,              // largest integer with full precision
        -4503599627370495.0,
        9007199254740991.0,              // 2^53 - 1
        -9007199254740991.0,
        1.7976931348623157e308,          // MAX finite f64
        f64::MIN_POSITIVE,               // smallest positive *normal*
    ];

    // Same validity for all (all valid)
    let mut validity = BitVec::new();
    validity.resize(values.len(), true);

    let path = "tests/vector_data/f64/output/factored_test2.erebus";

    // -----------------------------------------------------------
    // 2. Write file
    // -----------------------------------------------------------
    {
        let file = File::create(path).unwrap();
        let mut writer = ErebusWriter::new(file);
        writer
            .vector_data()
            .f64()
            .factored()
            .write(&values, &validity);
    }

    // -----------------------------------------------------------
    // 3. Read file
    // -----------------------------------------------------------
    let (read_vals, read_validity) = {
        let file = File::open(path).expect("failed to open file");
        let mut reader = ErebusReader::new(file);

        reader
            .vector_data()
            .f64()
            .factored()
            .read()
            .expect("read failed")
    };

    // -----------------------------------------------------------
    // 4. Debug Output
    // -----------------------------------------------------------
    println!("--- High Precision F64 Factored Roundtrip ---");
    for i in 0..values.len() {
        println!(
            "idx {:>2}: in={:.17e} out={:.17e} bits_in={:#018x} bits_out={:#018x}",
            i,
            values[i],
            read_vals[i],
            values[i].to_bits(),
            read_vals[i].to_bits()
        );
    }

    // -----------------------------------------------------------
    // 5. Assertions
    // -----------------------------------------------------------

    // validity must match exactly
    assert_eq!(read_validity, validity);

    // bitwise exact equality for all values
    for i in 0..values.len() {
        assert!(
            read_vals[i].to_bits() == values[i].to_bits(),
            "Mismatch at index {}:\n  Input  = {:.17e} ({:#018x})\n  Output = {:.17e} ({:#018x})",
            i,
            values[i],
            values[i].to_bits(),
            read_vals[i],
            read_vals[i].to_bits(),
        );
    }
}

#[test]
fn fuzz_f64_factored_roundtrip() {
    // Where to store fuzz outputs
    let path = "tests/vector_data/f64/output/factored_test3.erebus";

    let mut rng = rand::thread_rng();

    // Perform many fuzz iterations
    for iter in 0..2000 {
        // Random vector length
        let len = rng.gen_range(1..2000);

        // Generate random values
        let mut values = Vec::with_capacity(len);
        let mut validity = BitVec::with_capacity(len);

        for _ in 0..len {
            let is_valid = rng.gen_bool(0.8); // 80% valid
            validity.push(is_valid);

            if is_valid {
                // Generate random f64 including edge cases
                let choice: u8 = rng.gen_range(0..6);
                let v = match choice {
                    0 => rng.gen::<f64>(),            // uniform bits
                    1 => rng.gen_range(-1e300..1e300),
                    2 => rng.gen_range(-1e-300..1e-300),
                    3 => 0.0,
                    4 => -0.0,
                    5 => rng.gen_range(-1e100..1e100),
                    _ => unreachable!(),
                };
                values.push(v);
            } else {
                // invalid rows are ignored and come back as f64::default()
                values.push(12345.678); // arbitrary junk
            }
        }

        // ---------- Write ----------
        {
            let file = File::create(path).unwrap();
            let mut writer = ErebusWriter::new(file);
            writer
                .vector_data()
                .f64()
                .factored()
                .write(&values, &validity);
        }

        // ---------- Read ----------
        let (read_vals, read_validity) = {
            let file = File::open(path).expect("failed to open file");
            let mut reader = ErebusReader::new(file);

            reader
                .vector_data()
                .f64()
                .factored()
                .read()
                .expect("read failed")
        };

        // ---------- Validate ----------
        assert_eq!(
            read_validity, validity,
            "Validity mismatch on iteration {}",
            iter
        );

        for i in 0..len {
            if validity[i] {
                let input  = values[i];
                let output = read_vals[i];

                if input == 0.0 {
                    // Zero case → ignore sign bit
                    assert!(
                        output == 0.0,
                        "Zero mismatch at idx {} on iter {}: input = {:?}, output = {:?}",
                        i, iter, input, output
                    );
                } else {
                    // Exact IEEE-754 bitwise match
                    assert!(
                        output.to_bits() == input.to_bits(),
                        "Mismatch at idx {} on iter {}: input = {:?}, output = {:?}",
                        i, iter, input, output
                    );
                }
            } else {
                // Invalid → must be default (0.0)
                assert!(
                    read_vals[i] == f64::default(),
                    "Invalid row not defaulted at idx {} on iter {}",
                    i, iter
                );
            }
        }
    }
}

#[test]
fn fuzz_f64_factored_roundtrip_zstd() {
    let path = "tests/vector_data/f64/output/factored_test4.erebus";
    let mut rng = rand::thread_rng();

    for iter in 0..1500 {
        let len = rng.gen_range(1..2000);

        let mut values = Vec::with_capacity(len);
        let mut validity = BitVec::with_capacity(len);

        for _ in 0..len {
            let is_valid = rng.gen_bool(0.8);
            validity.push(is_valid);

            if is_valid {
                let choice: u8 = rng.gen_range(0..6);
                let v = match choice {
                    0 => rng.gen::<f64>(),
                    1 => rng.gen_range(-1e300..1e300),
                    2 => rng.gen_range(-1e-300..1e-300),
                    3 => 0.0,
                    4 => -0.0,
                    5 => rng.gen_range(-1e100..1e100),
                    _ => unreachable!(),
                };
                values.push(v);
            } else {
                values.push(12345.678);
            }
        }

        {
            let file = File::create(path).unwrap();
            let mut writer = ErebusWriter::new(file)
                .with_compression(CompressionType::Zstd);

            writer
                .vector_data()
                .f64()
                .factored()
                .write(&values, &validity)
                .expect("write failed");
        }

        // ---- Read ----
        let (read_vals, read_validity, header) = {
            let file = File::open(path).unwrap();
            let mut reader = ErebusReader::new(file);

            // FIRST call .read() (it returns values, validity)
            let (vals, valid) = reader
                .vector_data()
                .f64()
                .factored()
                .read()
                .expect("read failed");

            // THEN read header again (seek back to start)
            reader.seek_abs(4 + 1); // MAGIC + VERSION
            let header = reader.read_global_header().unwrap();

            (vals, valid, header)
        };

        assert_eq!(header.compression, CompressionType::Zstd);

        assert_eq!(read_validity, validity);

        for i in 0..len {
            if validity[i] {
                let input = values[i];
                let output = read_vals[i];

                if input == 0.0 {
                    assert!(
                        output == 0.0,
                        "Zero mismatch at idx {} iter {}",
                        i, iter
                    );
                } else {
                    assert!(
                        output.to_bits() == input.to_bits(),
                        "Mismatch idx {} iter {}: in={:?}, out={:?}",
                        i, iter, input, output
                    );
                }
            } else {
                assert!(
                    read_vals[i] == f64::default(),
                    "Invalid row not defaulted at idx {} iter {}",
                    i, iter
                );
            }
        }
    }
}

#[test]
fn fuzz_f64_factored_roundtrip_lz4() {
    let path = "tests/vector_data/f64/output/factored_test5.erebus";
    let mut rng = rand::thread_rng();

    for iter in 0..1500 {
        let len = rng.gen_range(1..2000);

        let mut values = Vec::with_capacity(len);
        let mut validity = BitVec::with_capacity(len);

        for _ in 0..len {
            let is_valid = rng.gen_bool(0.8);
            validity.push(is_valid);

            if is_valid {
                let choice: u8 = rng.gen_range(0..6);
                let v = match choice {
                    0 => rng.gen::<f64>(),
                    1 => rng.gen_range(-1e300..1e300),
                    2 => rng.gen_range(-1e-300..1e-300),
                    3 => 0.0,
                    4 => -0.0,
                    5 => rng.gen_range(-1e100..1e100),
                    _ => unreachable!(),
                };
                values.push(v);
            } else {
                values.push(12345.678);
            }
        }

        {
            let file = File::create(path).unwrap();
            let mut writer = ErebusWriter::new(file)
                .with_compression(CompressionType::Lz4);

            writer
                .vector_data()
                .f64()
                .factored()
                .write(&values, &validity)
                .expect("write failed");
        }

        let (read_vals, read_validity, header) = {
            let file = File::open(path).unwrap();
            let mut reader = ErebusReader::new(file);

            // FIRST call .read() (it returns values, validity)
            let (vals, valid) = reader
                .vector_data()
                .f64()
                .factored()
                .read()
                .expect("read failed");

            // THEN read header again (seek back to start)
            reader.seek_abs(4 + 1); // MAGIC + VERSION
            let header = reader.read_global_header().unwrap();

            (vals, valid, header)
        };

        assert_eq!(header.compression, CompressionType::Lz4);

        assert_eq!(read_validity, validity);

        for i in 0..len {
            if validity[i] {
                let input = values[i];
                let output = read_vals[i];

                if input == 0.0 {
                    assert!(output == 0.0);
                } else {
                    assert!(
                        output.to_bits() == input.to_bits(),
                        "Mismatch idx {} iter {}: in={:?}, out={:?}",
                        i, iter, input, output
                    );
                }
            } else {
                assert!(
                    read_vals[i] == f64::default(),
                    "Invalid row not defaulted at idx {} iter {}",
                    i, iter
                );
            }
        }
    }
}