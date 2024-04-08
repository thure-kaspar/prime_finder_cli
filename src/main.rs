use std::ops::RangeInclusive;
use clap::Parser;
use rayon::{prelude::*, ThreadPoolBuilder};


#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long)]
    start: Option<u32>,
    #[arg(short, long, default_value_t = 100_000_000)]
    end: u32,
    #[arg(short, long)]
    threads: Option<usize>
}


fn main() {
    let args: Args = Args::parse();

    if let Some(thread_num) = args.threads {
        ThreadPoolBuilder::new().num_threads(thread_num).build_global().expect("Set threads successfully and only once.");
    }

    let mut start: u32 = 0;
    if let Some(arg) = args.start {
        start = arg;
    }
    
    let end: u32 = args.end;

    
    let range: RangeInclusive<u32> = start..=end;
    range.into_par_iter()
    .filter(|i: &u32| is_prime(*i))
    .for_each(|i: u32| println!("{}", i));

    // print_sieve_primes(&sieve_of_eratosthenes(1000));
}


fn is_prime(number: u32) -> bool {
    let mut i: u32 = 2;
    let max: u32 = (number as f64).sqrt() as u32;
    while i <= max {
        if number % i == 0 {
            return number == i;
        }
        i += 1;
    }
    number >= 2
}


fn sieve_of_eratosthenes(number: usize) -> Vec<bool> {
    let mut sieve: Vec<bool> = vec![true; number];
    sieve[0] = false;

    let mut p: usize = 2;

    while p <= number {
        let mut j = p;
        if j > number {
            break;
        }
        while j <= number {
            sieve[j - 1] = false;
            j += p;

        }

        let old_p = p;
        let mut num = p+1;
        while num <= number {
            if sieve[num - 1] {
                p = num;
                break;
            }
            num += 1;
        }
        if p == old_p {
            break;
        }
    }

    sieve
}

fn print_sieve_primes(sieve: &Vec<bool>) {
    let mut i = 1;
    for b in sieve {
        if *b {
            println!("{i}");
        }
        i += 1
    }
}