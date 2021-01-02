use std::io;
use std::io::{BufRead};

fn max_dot_product(n:usize, a: &mut Vec<i64>, b: &mut Vec<i64>) -> i64 {
    let mut r = 0;
    if a.len() == 0 || b.len() == 0 {return r;}
    if a.len() != n || b.len() != n {return r;}
    a.sort();
    b.sort();
    for i in 0..n {
        r += a[i] * b[i];
    }
    return r;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_dot_product() {
        assert_eq!(max_dot_product(3,&mut vec![1, 2, 5, 9], &mut vec![1, 2, 5, 9]), 0);
        assert_eq!(max_dot_product(3,&mut vec![1, 3, -5], &mut vec![-2, 4, 1]), 23);
        assert_eq!(max_dot_product(4,&mut vec![], &mut vec![200, 400, 550, 750]), 0);
        assert_eq!(max_dot_product(1,&mut vec![23], &mut vec![39]), 897);
        assert_eq!(max_dot_product(3,&mut vec![2, 3, 9], &mut vec![7, 2, 4]), 79);
    }
}

fn main() {
    let reader = io::stdin();
    let mut handle = reader.lock();
    let mut n: usize = 0;
    let mut prices: Vec<i64> = Vec::new();
    let mut clicks: Vec<i64> = Vec::new();

    loop {
        // parse input for n
        let mut n_buffer = String::new();
        let result: Result<usize, io::Error> = handle.read_line(&mut n_buffer);
        if result.is_err() {
            println!("failed to read your input")
        }
        let trimmed: &str = n_buffer.trim();
        let n_opt: Option<usize> = trimmed.parse::<usize>().ok();
        match n_opt {
            Some(i) => n = i,
            None => println!("input is not an integer")
        }

        // parse input for prices
        let mut p_buffer = String::new();
        let result: Result<usize, io::Error> = handle.read_line(&mut p_buffer);
        if result.is_err() {
            println!("failed to read your input")
        }
        // make a vector of strings from input
        let split: Vec<&str> = p_buffer.trim().split(" ").collect();

        for price in split.iter() {
            let opt: Option<i64> = price.parse::<i64>().ok();
            match opt {
                Some(i) => prices.push(i),
                None => println!("input is not an integer")
            }
        }

        // parse input for clicks
        let mut c_buffer = String::new();
        let result: Result<usize, io::Error> = handle.read_line(&mut c_buffer);
        if result.is_err() {
            println!("failed to read your input")
        }
        // make a vector of strings from input
        let split: Vec<&str> = c_buffer.trim().split(" ").collect();

        for click in split.iter() {
            let opt: Option<i64> = click.parse::<i64>().ok();
            match opt {
                Some(i) => clicks.push(i),
                None => println!("input is not an integer")
            }
        }

        break;
    }

    println!("{}", max_dot_product(n, &mut prices, &mut clicks));
}