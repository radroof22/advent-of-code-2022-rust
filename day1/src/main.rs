use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("day1.input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut deers: Vec<i32> = vec![0];
    let mut deer_i = 0;

    for line in contents.lines() {
        match line {
            "" => {
                deers.push(0);
                deer_i += 1;
            },
            _ => {
                let curr_int:i32 = line.parse().unwrap();
                deers[deer_i] += curr_int;
            },
        }
    }

    let max_payload = deers.iter().max().unwrap();
    println!("The maximum payload for a reindeer is {}!", max_payload);

    // Part 2
    let mut largest = [0, 0, 0];
    for amnt in deers {
        if amnt > largest[0] || amnt > largest[1] || amnt > largest[2] {
            // identify the lowest value of the largests
            let min_val = largest.iter().min().unwrap();
            let mut lowest_i = 0;
            for (i, el) in largest.iter().enumerate() {
                if el == min_val {
                    lowest_i = i;
                    break;
                }
            }
            largest[lowest_i] = amnt;
        }
    }
    

    let sum:i32 = largest.iter().sum();
    println!("The sum of the three largest reindeer's capacity is: {}", sum);
}
