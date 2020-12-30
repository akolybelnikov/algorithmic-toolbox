use std::io;

fn get_change(m: i32) -> i32 {
    m / 10 + (m % 10) / 5 + (m % 10) % 5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_change() {
        assert_eq!(get_change(128), 16);
        assert_eq!(get_change(5), 1);
        assert_eq!(get_change(1), 1);
        assert_eq!(get_change(0), 0);
    }
}

fn main() {
    let reader: io::Stdin = io::stdin();
    let mut input_text: String = String::new();
    let result: Result<usize, io::Error> = reader.read_line(&mut input_text);
    if result.is_err() {
        println!("failed to read your input")
    }
    let trimmed: &str = input_text.trim();
    let option: Option<i32> = trimmed.parse::<i32>().ok();
    match option {
        Some(i) => println!("{:?}", get_change(i)),
        None => println!("input is not an integer: {:?}", trimmed)
    }
}