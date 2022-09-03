fn main() {
    // the if let syntax increases verbosity in many cases
    // the tradeoff is that we lose the exhaustive matching - syntax sugar for match (only for one pattern)
    let number = Some(3);
    if let Some(n) = number {
        println!("The number is {}", n)
    }
    // in stead of something like
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
}
