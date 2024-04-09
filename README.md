# Rust Prime Finder CLI
## Installation

The rust toolchain needs to be installed (https://rustup.rs/)

```bash
git clone https://github.com/thure-kaspar/prime_finder_cli
cd prime_finder_cli
cargo build -r
```

## Running

After building the project you should see a `/target/release` folder in your project folder. This folder contains an executable called `prime_finder_cli(.exe)`. If you open your Terminal at this location you can run the program by typing "`./prime_finder_cli`" in your terminal. *On Windows this might work differently.*

You can list all available command line parameters by executing the program with the `-h` flag *(e.g. `./prime_finder_cli -h`)*. 

By default it will print out all primes smaller than 100 Million and it uses as many threads as possible. You can change this behaviour with the `-e` flag for the end number and the `-t` flag for the threads. The *Sieve of Eratosthenes* algorithm is also implemented, but not multithreaded. To use it run the cli with the `-m` flag. 
