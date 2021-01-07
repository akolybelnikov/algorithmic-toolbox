use std::io;
use std::io::BufRead;

fn optimal_points(mut slots: Vec<Slot>) -> Vec<i64> {
    // order the segments by end
    &slots.sort_by(|a, b| a.end
        .partial_cmp(&b.end)
        .unwrap());

    // declare result array starting with the first slot's end
    let mut points: Vec<i64> = vec![slots[0].end];
    // Loop over slots
    for slot in slots {
        if slot.start > points[points.len() - 1] {
            points.push(slot.end);
        }
    }

    return points;
}

#[derive(Debug, Default, Copy, Clone)]
struct Slot {
    start: i64,
    end: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_optimal_value() {
        assert_eq!(optimal_points(3, vec![Slot { start: 1, end: 3 }, Slot { start: 2, end: 5 }, Slot { start: 3, end: 6 }]), vec![3]);
        assert_eq!(optimal_points(4, vec![Slot { start: 4, end: 7 }, Slot { start: 1, end: 3 }, Slot { start: 2, end: 5 }, Slot { start: 5, end: 6 }]), vec![3, 6]);
        assert_eq!(optimal_points(7, vec![Slot { start: 12, end: 13 }, Slot { start: 10, end: 11 }, Slot { start: 8, end: 11 }, Slot { start: 4, end: 10 }, Slot { start: 3, end: 6 }, Slot { start: 1, end: 3 }, Slot { start: 1, end: 5 }]), vec![3, 10, 12]);
    }
}

fn main() {
    let reader = io::stdin();
    let mut handle = reader.lock();
    let mut num: usize = 0;
    let mut segments: Vec<Slot> = Vec::new();

    loop {
        let mut buffer = String::new();
        let result: Result<usize, io::Error> = handle.read_line(&mut buffer);
        if result.is_err() {
            println!("failed to read your input")
        }
        let split = buffer.trim().split(" ").collect::<Vec<&str>>();
        let num_opt: Option<usize> = split[0].parse::<usize>().ok();
        match num_opt {
            Some(i) => num = i,
            None => println!("input is not an integer")
        }

        let mut n = num;
        loop {
            if n == 0 {
                break;
            }
            let mut buffer = String::new();

            let result: Result<usize, io::Error> = handle.read_line(&mut buffer);
            if result.is_err() {
                println!("failed to read your input")
            }
            let mut slot: Slot = Slot::default();
            let split = buffer.trim().split(" ").collect::<Vec<&str>>();
            let start: Option<i64> = split[0].parse::<i64>().ok();
            match start {
                Some(i) => slot.start = i,
                None => println!("1 is not an integer")
            }
            let end: Option<i64> = split[1].parse::<i64>().ok();
            match end {
                Some(i) => slot.end = i,
                None => println!("2 is not an integer")
            }
            segments.push(slot);
            n -= 1;
        }
        break;
    }

    let op = optimal_points(segments);
    println!("{:?}", op.len());
    for p in op {
        print!("{:?} ", p);
    }
}