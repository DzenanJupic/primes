#![allow(unused_variables)]
use std::fs::*;
use std::io::Write;

fn main() {
    let mut primes: Vec<u64> = vec![2, 3];
    let mut current: Option<u64> = None;
    let mut res: f64;
    
    for i in 4..100000000 {
        for prime in &primes[1..] {
            if i % 2 == 0 {
                current = None;
                break;
            }
            
            res = i as f64/ *prime as f64;
            
            if res == (res as u64) as f64 {
                current = None;
                break;
            } else {
                current = Some(i);
            }
        }
        
        if let Some(n) = current {
            primes.push(n);
        }
    }
    let mut str_primes = String::new();
    for prime in primes {
        str_primes.push_str(format!("{}, ", prime).as_str());
    }
    //let mut file = File::create("C:/Users/info/Desktop/result.csv").unwrap();
    //file.write(str_primes.as_bytes()).unwrap();
    print!("{}", str_primes);
    
}