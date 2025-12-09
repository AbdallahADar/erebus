// === Imports ===
use erebus_io::prelude::*;
use std::fs::File;
use rand::Rng;

// === Tests ===

#[test]
fn test_vectordata_f64_raw_roundtrip() {
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
        true,
        true,
        false,  // becomes default()
        true,
        true,
        true,
        false, // becomes default()
    ]);

    let expected = vec![
        0.0,
        1.5,
        f64::default(),
        f64::INFINITY,
        f64::NEG_INFINITY,
        1e-310,
        f64::default(),
    ];

    let path = "tests/vector_data/f64/output/raw_test1.erebus";

    // ---------------------------------------------------------------
    // 2. Write file
    // ---------------------------------------------------------------
    {
        let file = File::create(path).unwrap();
        let mut writer = ErebusWriter::new(file);
        writer
            .vector_data()
            .f64()
            .raw()
            .write(&values, &validity);
    }

    // ---------------------------------------------------------------
    // 3. Read file
    // ---------------------------------------------------------------
    let (read_vals, read_validity) = {
        let file = File::open(path).expect("failed to open file");
        let mut reader = ErebusReader::new(file);

        reader
            .vector_data()
            .f64()
            .raw()
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

    assert_eq!(read_validity, validity, "Validity mismatch");

    for i in 0..expected.len() {
        assert!(
            read_vals[i].to_bits() == expected[i].to_bits(),
            "Mismatch at idx {}: got {:?}, expected {:?}",
            i, read_vals[i], expected[i]
        );
    }
}

#[test]
fn test_vectordata_f64_raw_high_precision() {
    let values = vec![
        1.234567890123456,
        -9.876543210987654,
        1.0e308,
        1.0e-308,
        2.2250738585072014e-308,
        5e-324,
        -5e-324,
        3.141592653589793,
        -2.718281828459045,
        0.30000000000000004,
        4503599627370495.0,
        -4503599627370495.0,
        9007199254740991.0,
        -9007199254740991.0,
        1.7976931348623157e308,
        f64::MIN_POSITIVE,
    ];

    let mut validity = BitVec::new();
    validity.resize(values.len(), true);

    let path = "tests/vector_data/f64/output/raw_test2.erebus";

    {
        let file = File::create(path).unwrap();
        let mut writer = ErebusWriter::new(file);
        writer
            .vector_data()
            .f64()
            .raw()
            .write(&values, &validity);
    }

    let (read_vals, read_validity) = {
        let file = File::open(path).expect("failed to open file");
        let mut reader = ErebusReader::new(file);

        reader
            .vector_data()
            .f64()
            .raw()
            .read()
            .expect("read failed")
    };

    assert_eq!(validity, read_validity);

    for i in 0..values.len() {
        assert!(
            read_vals[i].to_bits() == values[i].to_bits(),
            "Precision mismatch at index {}:\n\
             in  = {:.17e} ({:#018x})\n\
             out = {:.17e} ({:#018x})",
            i,
            values[i], values[i].to_bits(),
            read_vals[i], read_vals[i].to_bits(),
        );
    }
}

#[test]
fn fuzz_f64_raw_roundtrip() {

    let path = "tests/vector_data/f64/output/raw_test3.erebus";
    let mut rng = rand::thread_rng();

    for iter in 0..2000 {
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
                values.push(999.999); // ignored garbage
            }
        }

        // ---------- Write ----------
        {
            let file = File::create(path).unwrap();
            let mut writer = ErebusWriter::new(file);
            writer
                .vector_data()
                .f64()
                .raw()
                .write(&values, &validity);
        }

        // ---------- Read ----------
        let (read_vals, read_validity) = {
            let file = File::open(path).expect("failed to open file");
            let mut reader = ErebusReader::new(file);

            reader
                .vector_data()
                .f64()
                .raw()
                .read()
                .expect("read failed")
        };

        // ---------- Validate ----------
        assert_eq!(
            read_validity, validity,
            "Validity mismatch at iteration {}",
            iter
        );

        for i in 0..len {
            if validity[i] {
                let input = values[i];
                let output = read_vals[i];

                if input == 0.0 {
                    assert!(
                        output == 0.0,
                        "Zero mismatch at idx {} iter {}: in={:?}, out={:?}",
                        i, iter, input, output
                    );
                } else {
                    assert!(
                        output.to_bits() == input.to_bits(),
                        "Mismatch at idx {} iter {}: in={:?}, out={:?}",
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
fn fuzz_f64_raw_roundtrip_zstd() {
    let path = "tests/vector_data/f64/output/raw_test4.erebus";
    let mut rng = rand::thread_rng();

    for iter in 0..2000 {
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
                values.push(999.999);
            }
        }

        // ---------- Write ----------
        {
            let file = File::create(path).unwrap();
            let mut writer = ErebusWriter::new(file)
                .with_compression(CompressionType::Zstd);

            writer
                .vector_data()
                .f64()
                .raw()
                .write(&values, &validity);
        }

        // ---------- Read ----------
        let (read_vals, read_validity, header) = {
            let file = File::open(path).unwrap();
            let mut reader = ErebusReader::new(file);

            // FIRST call .read() (it returns values, validity)
            let (vals, valid) = reader
                .vector_data()
                .f64()
                .raw()
                .read()
                .expect("read failed");

            // THEN read header again (seek back to start)
            reader.seek_abs(4 + 1); // MAGIC + VERSION
            let header = reader.read_global_header().unwrap();

            (vals, valid, header)
        };

        assert_eq!(header.compression, CompressionType::Zstd);


        // ---------- Validate ----------
        assert_eq!(
            read_validity, validity,
            "Validity mismatch (ZSTD) at iteration {}",
            iter
        );

        for i in 0..len {
            if validity[i] {
                let input = values[i];
                let output = read_vals[i];

                if input == 0.0 {
                    assert!(
                        output == 0.0,
                        "Zero mismatch @ idx {} iter {} ZSTD: in={:?}, out={:?}",
                        i, iter, input, output
                    );
                } else {
                    assert!(
                        output.to_bits() == input.to_bits(),
                        "Bit mismatch @ idx {} iter {} ZSTD: in={:?} ({:#x}), out={:?} ({:#x})",
                        i, iter,
                        input, input.to_bits(),
                        output, output.to_bits()
                    );
                }
            } else {
                assert!(
                    read_vals[i] == f64::default(),
                    "Invalid row not defaulted @ idx {} iter {} ZSTD",
                    i, iter
                );
            }
        }
    }
}

#[test]
fn fuzz_f64_raw_roundtrip_lz4() {
    let path = "tests/vector_data/f64/output/raw_test5.erebus";
    let mut rng = rand::thread_rng();

    for iter in 0..2000 {
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
                values.push(999.999);
            }
        }

        // ---------- Write ----------
        {
            let file = File::create(path).unwrap();
            let mut writer = ErebusWriter::new(file)
                .with_compression(CompressionType::Lz4);

            writer
                .vector_data()
                .f64()
                .raw()
                .write(&values, &validity);
        }

        // ---------- Read ----------
        let (read_vals, read_validity, header) = {
            let file = File::open(path).unwrap();
            let mut reader = ErebusReader::new(file);

            // FIRST call .read() (it returns values, validity)
            let (vals, valid) = reader
                .vector_data()
                .f64()
                .raw()
                .read()
                .expect("read failed");

            // THEN read header again (seek back to start)
            reader.seek_abs(4 + 1); // MAGIC + VERSION
            let header = reader.read_global_header().unwrap();

            (vals, valid, header)
        };

        assert_eq!(header.compression, CompressionType::Lz4);


        // ---------- Validate ----------
        assert_eq!(
            read_validity, validity,
            "Validity mismatch (LZ4) at iteration {}",
            iter
        );

        for i in 0..len {
            if validity[i] {
                let input = values[i];
                let output = read_vals[i];

                if input == 0.0 {
                    assert!(
                        output == 0.0,
                        "Zero mismatch @ idx {} iter {} LZ4: in={:?}, out={:?}",
                        i, iter, input, output
                    );
                } else {
                    assert!(
                        output.to_bits() == input.to_bits(),
                        "Bit mismatch @ idx {} iter {} LZ4: in={:?} ({:#x}), out={:?} ({:#x})",
                        i, iter,
                        input, input.to_bits(),
                        output, output.to_bits()
                    );
                }
            } else {
                assert!(
                    read_vals[i] == f64::default(),
                    "Invalid row not defaulted @ idx {} iter {} LZ4",
                    i, iter
                );
            }
        }
    }
}