use anyhow::{Context, Result};
use pancakes::*;
use std::{fs::File, io::Write, ops::RangeInclusive, time};

fn timed<T>(body: impl FnOnce() -> T) -> (T, std::time::Duration) { let start = time::Instant::now();
    let result = body();
    let time_taken = start.elapsed();
    (result, time_taken)
}

fn bench(
    outfile: &mut File,
    algname: &str,
    input_sizes: RangeInclusive<usize>,
    repetitions: usize,
    num_threads: usize,
) -> Result<()> {
    let tp = rayon::ThreadPoolBuilder::new().num_threads(num_threads).build()?;
    let num_threads = tp.current_num_threads();
    let mut buf = String::new();
    for n in input_sizes.clone() {
        for _ in 0..repetitions {
            let ((_, result), time_taken) = tp.install(|| match algname {
                "rayon1" => timed(|| fannkuch_rayon1(n)),
                "rayon2" => timed(|| fannkuch_rayon2(n)),
                "adaptive" => timed(|| fannkuch_adaptive(n, 1000)),
                _ => timed(|| fannkuch_sequential(n)),
            });

            let entry = format!(
                "{},{},{},{},{}\n",
                num_threads,
                n,
                algname,
                result,
                time_taken.as_nanos()
            );
            eprintln!("{}", entry);
            buf.push_str(&&entry);
        }
    }
    outfile
        .write_all(buf.as_bytes())
        .context("Failed to write entry to file")?;

    Ok(())
}

fn main() -> Result<()> {
    let mut f = File::create("results.csv")?;

    f.write_all("num_threads,N,algorithm,result,time\n".as_bytes())?;

    let num_cpus = num_cpus::get();

    let algs = ["adaptive", "rayon1", "rayon2", "sequential"];
    
    for alg in &algs {
        for num_threads in (1..=num_cpus).rev() {
            bench(&mut f, alg, 6..=12, 10, num_threads)?;
        }
    }

    Ok(())
}
