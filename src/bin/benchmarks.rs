use anyhow::{Context, Result};
use pancakes::*;
use std::{fs::File, io::Write, time};

fn timed<T>(body: impl FnOnce() -> T) -> (T, std::time::Duration) {
    let start = time::Instant::now();
    let result = body();
    let time_taken = start.elapsed();
    (result, time_taken)
}

fn main() -> Result<()> {
    let algnames = vec!["adaptive", "rayon", "sequential"];
    let input_sizes = 6..=10;
    let num_threads = 1..=4;
    let repetitions = 1..=100;

    let mut f = File::create("results.csv").context("Failed to open file")?;
    f.write("index,num_threads,N,algorithm,result,time\n".as_bytes())
        .context("Failed to write to file.")?;

    let mut index = 0;
    for num_thread in num_threads {
        for algname in algnames.clone() {
            for n in input_sizes.clone() {
                for _ in repetitions.clone() {
                    let tp = rayon::ThreadPoolBuilder::new()
                        .num_threads(num_thread)
                        .build()
                        .expect("Could not build thread pool");
                    let ((_, result), time_taken) = tp.install(|| {
                        match algname {
                            "rayon" => timed(|| fannkuch_rayon(n)),
                            "adaptive" => timed(|| fannkuch_adaptive(n)),
                            _ => timed(|| fannkuch_sequential(n)),
                        }
                    });

                    let entry = format!(
                        "{},{},{},{},{},{}\n",
                        index,
                        num_thread,
                        n,
                        algname,
                        result,
                        time_taken.as_nanos()
                    );
                    eprintln!("{}", entry);
                    f.write(entry.as_bytes())
                        .context("Failed to write entry to file")?;
                    index += 1;
                }
            }
        }
    }

    Ok(())
}
