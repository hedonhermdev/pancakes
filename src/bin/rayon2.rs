use pancakes::fannkuch_rayon2;

fn main() {
    let n = std::env::args().nth(1)
        .and_then(|n| n.parse().ok())
        .unwrap_or(7);

    let (checksum, maxflips) = fannkuch_rayon2(n);
    println!("{}\nPfannkuchen({}) = {}", checksum, n, maxflips);
}
