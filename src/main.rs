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
    threads: Option<usize>,
    #[arg(short, long)]
    memory: bool
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

    if args.memory {
        if args.threads == Some(1) || args.threads == None {
            print_sieve_primes(&sieve_of_eratosthenes(end as usize), &start);
        } else {
            eprintln!("Multithreading is not implemented for memory mode. Remove --threads or --memory.")
        }
    } else {
        let range: RangeInclusive<u32> = start..=end;
        range.into_par_iter()
        .filter(|i: &u32| is_prime(*i))
        .for_each(|i: u32| println!("{}", i));
    }
    
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


fn print_sieve_primes(sieve: &Vec<bool>, start: &u32) {
    let actual_start: usize;
    if start <= &2 {
        actual_start = 2;
    } else {
        actual_start = *start as usize;
    }
    let mut i = actual_start;
    for b in sieve.as_slice()[actual_start-2..].into_iter() {
        if *b {
            println!("{i}");
        }
        i += 1
    }
}


fn sieve_of_eratosthenes(n: usize) -> Vec<bool> {
    let mut sieve = vec![true; n-1];
    
    for i in 2..=((n as f32).sqrt() as usize){
        if sieve[i-2] {
            let mut j = i * i;
            while j <= n {
                sieve[j-2] = false;
                j += i;
            }
        }
    }

    sieve
}