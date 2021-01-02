use std::io;
use std::io::{BufRead};

fn compute_min_refills(d: i32, m: i32, stops: &Vec<i32>) -> i32 {
    if d <= m {
        return 0;
    }
    let mut num_stops = 0;
    let mut last_stop = 0;
    let mut p: usize = 0;

    while last_stop + m < d {
        if p >= stops.len() {
            num_stops = -1;
            break;
        }
        let mut seg: Vec<&i32> = Vec::new();
        for stop in stops[p..].iter() {
            if stop <= &(last_stop + m) {
                seg.push(stop);
                p += 1;
            } else {
                break;
            }
        }
        if seg.len() == 0 {
            num_stops = -1;
            break;
        } else {
            last_stop = *seg[seg.len() - 1];
            num_stops += 1;
        }
    }

    num_stops
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_min_refills() {
        assert_eq!(compute_min_refills(10, 3, &vec![1, 2, 5, 9]), -1);
        assert_eq!(compute_min_refills(950, 400, &vec![200, 375, 550, 750]), 2);
        assert_eq!(compute_min_refills(800, 400, &vec![200, 400, 550, 750]), 1);
        assert_eq!(compute_min_refills(700, 200, &vec![100, 200, 300, 400]), -1);
    }
}

fn main() {
    let reader = io::stdin();
    let mut handle = reader.lock();
    let mut d: i32 = 0;
    let mut m: i32 = 0;
    let mut _n: i32 = 0;
    let mut stops: Vec<i32> = Vec::new();

    loop {
        // parse input for d
        let mut d_buffer = String::new();
        let result: Result<usize, io::Error> = handle.read_line(&mut d_buffer);
        if result.is_err() {
            println!("failed to read your input")
        }
        let mut trimmed: &str = d_buffer.trim();
        let d_opt: Option<i32> = trimmed.parse::<i32>().ok();
        match d_opt {
            Some(i) => d = i,
            None => println!("input is not an integer")
        }

        // parse input for m
        let mut m_buffer = String::new();
        let result: Result<usize, io::Error> = handle.read_line(&mut m_buffer);
        if result.is_err() {
            println!("failed to read your input")
        }
        trimmed = m_buffer.trim();
        let m_opt: Option<i32> = trimmed.parse::<i32>().ok();
        match m_opt {
            Some(i) => m = i,
            None => println!("input is not an integer")
        }

        // parse input for n
        let mut n_buffer = String::new();
        let result: Result<usize, io::Error> = handle.read_line(&mut n_buffer);
        if result.is_err() {
            println!("failed to read your input")
        }
        trimmed = n_buffer.trim();
        let n_opt: Option<i32> = trimmed.parse::<i32>().ok();
        match n_opt {
            Some(i) => _n = i,
            None => println!("input is not an integer")
        }

        // parse input for stops
        let mut stops_buffer = String::new();
        let result: Result<usize, io::Error> = handle.read_line(&mut stops_buffer);
        if result.is_err() {
            println!("failed to read your input")
        }
        // make a vector of strings from input
        let split: Vec<&str> = stops_buffer.trim().split(" ").collect();

        for stop in split.iter() {
            let opt: Option<i32> = stop.parse::<i32>().ok();
            match opt {
                Some(i) => stops.push(i),
                None => println!("input is not an integer")
            }
        }

        break;
    }

    println!("{}", compute_min_refills(d, m, &stops));
}