use pancakes::fannkuch_sequential;

fn main() {
    let n = std::env::args().nth(1)
        .and_then(|n| n.parse().ok())
        .unwrap_or(7);

    let (checksum, maxflips) = fannkuch_sequential(n);
    println!("{}\nPfannkuchen({}) = {}", checksum, n, maxflips);
}
