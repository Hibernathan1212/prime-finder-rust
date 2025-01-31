use std::fs::File;
use std::io::Write;
use std::io;

fn prime_finder(limit: usize, file_path: &str) {

    let mut prime = vec![true; limit / 2];

    let mut file = File::create(file_path).expect("Unable to create file");

    writeln!(file, "2").unwrap();
    writeln!(file, "3").unwrap();

    let mut p = 0;

    while p < limit / 3 {
        if prime[p] {
            let val = (p + 1) * 3 - ((p % 2) as isize - 2) as usize;

            writeln!(file, "{}", val).unwrap();

            let mut i = val;
            while i < limit {
                if i % 6 == 5 {
                    prime[((i + 1) / 6 - 1) * 2] = false;
                }
                if i % 6 == 1 {
                    prime[((i + 1) / 6 - 1) * 2 + 1] = false;
                }
                i += val;
            }
        }
        p += 1;
    }
}

fn main() {
    println!("Enter the limit for prime finding:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let limit: usize = input.trim().parse().expect("Please enter a valid number");

    println!("Enter file path to save the prime numbers:");
    let mut file_path = String::new();
    io::stdin().read_line(&mut file_path).expect("Failed to read line");
    let file_path = file_path.trim();

    let start = std::time::Instant::now();

    prime_finder(limit, file_path);
    let elapsed = start.elapsed();
    println!("Total elapsed time: {:.3}s", elapsed.as_secs_f64());
}