use std::io;
use std::io::{BufRead};

fn get_optimal_value(mut s: f32, items: &Vec<Item>) -> String {
    let mut v: f32 = 0.0;
    let mut u = items
        .iter()
        .enumerate()
        .map(|(i, &x)| (i, x.utility()))
        .collect::<Vec<_>>();

    &u.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    for it in u.into_iter() {
        if s >= items[it.0].weight {
            v += items[it.0].value;
            s -= items[it.0].weight;
            if s == 0.0 { break; }
        } else {
            v += it.1 * s;
            break;
        }
    };

    format!("{:.4}", v)
}

#[derive(Debug, Default, Copy, Clone)]
struct Item {
    weight: f32,
    value: f32,
}

impl Item {
    fn utility(&self) -> f32 {
        self.value / self.weight
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_optimal_value() {
        assert_eq!(get_optimal_value(50.0, &vec![Item{ weight: 20.0, value: 60.0 }, Item{ weight: 50.0, value: 100.0 }, Item{ weight: 30.0, value: 120.0 }]), "180.0000".to_owned());
        assert_eq!(get_optimal_value(10.0, &vec![Item{ weight: 30.0, value: 500.0 }]), "166.6667".to_owned());
    }
}

fn main() {
    let reader = io::stdin();
    let mut handle = reader.lock();
    let mut num = 2;
    let mut w = 0.0;
    let mut items = Vec::new();

    loop {
        let mut buffer = String::new();
        let result: Result<usize, io::Error> = handle.read_line(&mut buffer);
        if result.is_err() {
            println!("failed to read your input")
        }
        let split = buffer.trim().split(" ").collect::<Vec<&str>>();
        let num_opt: Option<i32> = split[0].parse::<i32>().ok();
        match num_opt {
            Some(i) => num = i,
            None => println!("input is not an integer")
        }
        let w_opt: Option<f32> = split[1].parse::<f32>().ok();
        match w_opt {
            Some(i) => w = i,
            None => println!("input is not an integer")
        }
        loop {
            if num == 0 {
                break;
            }
            let mut buffer = String::new();
            let result: Result<usize, io::Error> = handle.read_line(&mut buffer);
            if result.is_err() {
                println!("failed to read your input")
            }
            let mut item = Item::default();
            let split = buffer.trim().split(" ").collect::<Vec<&str>>();
            let val_opt: Option<f32> = split[0].parse::<f32>().ok();
            match val_opt {
                Some(i) => item.value = i,
                None => println!("input is not an integer")
            }
            let weight_opt: Option<f32> = split[1].parse::<f32>().ok();
            match weight_opt {
                Some(i) => item.weight = i,
                None => println!("input is not an integer")
            }
            items.push(item);
            num -= 1;
        }
        break;
    }

    println!("{}", get_optimal_value(w, &items));
}