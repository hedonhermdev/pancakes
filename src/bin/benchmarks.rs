use anyhow::{Context, Result};
use pancakes::*;
use std::{fs::{File, OpenOptions}, io::Write, ops::Range, time};

fn timed<T>(body: impl FnOnce() -> T) -> (T, std::time::Duration) { let start = time::Instant::now();
    let result = body();
    let time_taken = start.elapsed();
    (result, time_taken)
}

fn bench(
    outfile: &mut File,
    algname: &str,
    input_sizes: Range<usize>,
    repetitions: usize,
) -> Result<()> {
    let num_threads = rayon::current_num_threads();
    let mut buf = String::new();
    for n in input_sizes.clone() {
        for _ in 0..repetitions {
            let ((_, result), time_taken) = match algname {
                "rayon1" => timed(|| fannkuch_rayon1(n)),
                "rayon2" => timed(|| fannkuch_rayon2(n)),
                "adaptive" => timed(|| fannkuch_adaptive(n)),
                _ => timed(|| fannkuch_sequential(n)),
            };

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
        .write(buf.as_bytes())
        .context("Failed to write entry to file")?;

    Ok(())
}

fn main() -> Result<()> {
    let mut f = File::create("results.csv")?;

    f.write("num_threads,N,algorithm,result,time\n".as_bytes())?;

    bench(&mut f, "adaptive", 12..13, 30)?;
    bench(&mut f, "rayon1", 12..13, 30)?;
    bench(&mut f, "rayon2", 12..13, 30)?;
    bench(&mut f, "sequential", 12..13, 30)?;

    Ok(())
}
