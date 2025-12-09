use erebus_io::prelude::*;
use rand::Rng;
use std::fs::File;
use std::path::Path;

const N_ROWS: usize = 20_000_000;
const N_RUNS: usize = 10;          // how many times to overwrite & re-measure
const OUTPUT_DIR: &str = "crates/erebus-io/src/bin/output/";

fn generate_random_f64<R: Rng>(rng: &mut R) -> f64 {
    let choice: u8 = rng.gen_range(0..6);
    match choice {
        0 => rng.gen::<f64>(),
        1 => rng.gen_range(-1e300..1e300),
        2 => rng.gen_range(-1e-300..1e-300),
        3 => 0.0,
        4 => -0.0,
        5 => rng.gen_range(-1e100..1e100),
        _ => unreachable!(),
    }
}

fn main() {
    println!("--- Comparing Encodings & Compression for f64 VectorData (Averaged over {N_RUNS} runs) ---");

    let mut rng = rand::thread_rng();

    // --------------------------------------------
    // Generate the SAME random dataset ONCE
    // --------------------------------------------
    let mut values = Vec::with_capacity(N_ROWS);
    let mut validity = BitVec::with_capacity(N_ROWS);

    for _ in 0..N_ROWS {
        let is_valid = rng.gen_bool(0.95);
        validity.push(is_valid);

        if is_valid {
            values.push(generate_random_f64(&mut rng));
        } else {
            values.push(0.0);
        }
    }

    // --------------------------------------------
    // Test cases (one file each, overwritten per run)
    // --------------------------------------------
    let cases = vec![
        ("raw_none.erebus",      EncodingType::F64Raw,     CompressionType::None),
        ("raw_zstd.erebus",      EncodingType::F64Raw,     CompressionType::Zstd),
        ("raw_lz4.erebus",       EncodingType::F64Raw,     CompressionType::Lz4),
        ("factored_none.erebus", EncodingType::F64Factored,CompressionType::None),
        ("factored_zstd.erebus", EncodingType::F64Factored,CompressionType::Zstd),
        ("factored_lz4.erebus",  EncodingType::F64Factored,CompressionType::Lz4),
    ];

    std::fs::create_dir_all(OUTPUT_DIR).unwrap();

    // --------------------------------------------
    // Print table header
    // --------------------------------------------
    println!("{:<18} | {:<11} | {:<10} | {:>12}",
        "File", "Encoding", "Compression", "Avg Size KB");
    println!("{}", "-".repeat(65));

    // --------------------------------------------
    // Run tests
    // --------------------------------------------
    for (fname, encoding, comp) in cases {
        let path = Path::new(OUTPUT_DIR).join(fname);

        let mut total_size: f64 = 0.0;

        for _ in 0..N_RUNS {
            // ---------- Write (overwrite file) ----------
            {
                let f = File::create(&path).unwrap();
                let mut writer = ErebusWriter::new(f);
                writer.set_compression(comp);

                match encoding {
                    EncodingType::F64Raw => {
                        writer.vector_data().f64().raw().write(&values, &validity).unwrap();
                    }
                    EncodingType::F64Factored => {
                        writer.vector_data().f64().factored().write(&values, &validity).unwrap();
                    }
                    _ => unreachable!(),
                };
            }

            // ---------- Measure size ----------
            let size_kb = std::fs::metadata(&path).unwrap().len() as f64 / 1024.0;
            total_size += size_kb;
        }

        let avg_size = total_size / (N_RUNS as f64);

        // ---------- Print row ----------
        println!(
            "{:<18} | {:<11} | {:<10} | {:>12.2}",
            fname,
            match encoding {
                EncodingType::F64Raw => "raw",
                EncodingType::F64Factored => "factored",
                _ => "?",
            },
            match comp {
                CompressionType::None => "none",
                CompressionType::Zstd => "zstd",
                CompressionType::Lz4 => "lz4",
            },
            avg_size
        );
    }
}