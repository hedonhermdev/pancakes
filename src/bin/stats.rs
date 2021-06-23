use pancakes::fannkuch_adaptive;
use fast_tracer::stats;
use std::time;

fn timed<T>(body: impl FnOnce() -> T) -> (T, std::time::Duration) { let start = time::Instant::now();
    let result = body();
    let time_taken = start.elapsed();
    (result, time_taken)
}

fn main() {
    let n = std::env::args().nth(1)
        .and_then(|n| n.parse().ok())
        .unwrap_or(12);

    let mut block_size = 1;

    while block_size < 10_000 {
        println!("block_size: {}", block_size);
        let (_, time) = timed(|| stats(||fannkuch_adaptive(n, block_size)));
        println!("time_taken: {}\n", time.as_nanos());
        block_size *= 2;
    }
}
