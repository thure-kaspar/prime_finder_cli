use std::ops::RangeInclusive;
use clap::Parser;
use rayon::prelude::*;


#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long)]
    start: Option<u32>,
    #[arg(short, long, default_value_t = 100_000_000)]
    end: u32,
    #[arg(short, long)]
    threads: Option<u8>
}


fn main() {
    let args: Args = Args::parse();

    let mut start: u32 = 0;
    if let Some(arg) = args.start {
        start = arg;
    }
    
    let end: u32 = args.end;

    
    let range: RangeInclusive<u32> = start..=end;
    range.into_par_iter()
    .filter(|i: &u32| is_prime(*i))
    .for_each(|i: u32| println!("{}", i));
}


fn is_prime(number: u32) -> bool {
    let mut i: u32 = 2;
    let max: u32 = (number as f64).sqrt() as u32;
    while i <= max {
        if number % i == 0 {
            if number == i {
                return true;
            }
            return false;
        }
        i += 1;
    }
    number >= 2
}