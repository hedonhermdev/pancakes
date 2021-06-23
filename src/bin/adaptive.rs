use pancakes::{fannkuch_adaptive, fannkuch_rayon1, fannkuch_rayon2};
use fast_tracer::stats;

fn main() {
    let n = std::env::args().nth(1)
        .and_then(|n| n.parse().ok())
        .unwrap_or(7);

    let mut block_size = 1;

    for _ in 1..10_000 {
        stats(||fannkuch_adaptive(n, block_size));
        block_size *= 2;
    }
}
