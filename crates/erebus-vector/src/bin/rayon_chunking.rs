// Checking if our `AdaptiveChunker` would deteriorate rayon's native performance
// For now the goal is not to do better than rayon just not worse
// The end goal is that we can make our `AdaptiveChunker` smart and outperform rayon
// This shows that there is no deterioration

use rayon::prelude::*;
use std::time::Instant;
use rand::Rng;

#[inline(always)]
fn heavy_op(x: f64) -> f64 {
    (x * 1.000001).sqrt().sin().cos().tan()
}

/// Helper for repeated runs with warm-up and averages
fn timed_run<F: Fn() -> f64>(label: &str, runs: usize, f: F) -> f64 {
    let mut times = Vec::new();
    for i in 0..runs {
        let start = Instant::now();
        let sum = f();
        let dur = start.elapsed().as_secs_f64();
        if i > 0 { times.push(dur); } // skip warm-up
        println!("{label} | run {i:02}: sum={sum:.4e}, time={dur:.3}s");
    }
    let avg = times.iter().sum::<f64>() / times.len() as f64;
    println!("→ {label}: avg={avg:.3}s\n");
    avg
}

fn main() {
    let runs = 20;
    let chunk_size = 200_000; // pretend adaptive chunk suggestion
    println!("Chunk size used: {chunk_size}\n");

    // === Scenario 1: Uniform heavy workload ===
    {
        let n = 100_000_000;
        let data: Vec<f64> = (0..n).map(|x| x as f64).collect();
        println!("=== Scenario 1: Uniform Workload ===");

        let t1 = timed_run("par_iter()", runs, || {
            data.par_iter().map(|x| heavy_op(*x)).sum::<f64>()
        });
        let t2 = timed_run("par_chunks()", runs, || {
            data.par_chunks(chunk_size)
                .map(|chunk| chunk.iter().map(|x| heavy_op(*x)).sum::<f64>())
                .sum::<f64>()
        });
        println!("Speedup: {:.2}%\n\n", ((t1 - t2) / t1) * 100.0);
    }

    // === Scenario 2: Skewed null density (10% valid) ===
    {
        let n = 50_000_000;
        let data: Vec<Option<f64>> = (0..n)
            .map(|x| if x % 10 == 0 { Some(x as f64) } else { None })
            .collect();
        println!("=== Scenario 2: Sparse / Null-Skewed Workload ===");

        let t1 = timed_run("par_iter()", runs, || {
            data.par_iter()
                .filter_map(|x| x.map(|v| heavy_op(v)))
                .sum::<f64>()
        });
        let t2 = timed_run("par_chunks()", runs, || {
            data.par_chunks(chunk_size)
                .map(|chunk| {
                    chunk.iter()
                        .filter_map(|x| x.map(|v| heavy_op(v)))
                        .sum::<f64>()
                })
                .sum::<f64>()
        });
        println!("Speedup: {:.2}%\n\n", ((t1 - t2) / t1) * 100.0);
    }

    // === Scenario 3: Mixed workload (different costs per element) ===
    {
        let n = 100_000_000;
        let data: Vec<f64> = (0..n).map(|x| x as f64).collect();
        println!("=== Scenario 3: Mixed-Cost Workload ===");

        let t1 = timed_run("par_iter()", runs, || {
            data.par_iter()
                .map(|x| if x % 5.0 == 0.0 { heavy_op(*x) } else { x.sin() })
                .sum::<f64>()
        });
        let t2 = timed_run("par_chunks()", runs, || {
            data.par_chunks(chunk_size)
                .map(|chunk| {
                    chunk.iter()
                        .map(|x| if x % 5.0 == 0.0 { heavy_op(*x) } else { x.sin() })
                        .sum::<f64>()
                })
                .sum::<f64>()
        });
        println!("Speedup: {:.2}%\n\n", ((t1 - t2) / t1) * 100.0);
    }

    // === Scenario 4: Random memory access ===
    {
        let n = 10_000_000;
        let data: Vec<f64> = (0..n).map(|x| x as f64).collect();
        let mut rng = rand::thread_rng();
        let idx: Vec<usize> = (0..n).map(|_| rng.gen_range(0..n)).collect();
        println!("=== Scenario 4: Random Memory Access (I/O Bound) ===");

        let t1 = timed_run("par_iter()", runs, || {
            idx.par_iter()
                .map(|&i| data[i].sqrt())
                .sum::<f64>()
        });
        let t2 = timed_run("par_chunks()", runs, || {
            idx.par_chunks(chunk_size)
                .map(|chunk| chunk.iter().map(|&i| data[i].sqrt()).sum::<f64>())
                .sum::<f64>()
        });
        println!("Speedup: {:.2}%\n\n", ((t1 - t2) / t1) * 100.0);
    }
}