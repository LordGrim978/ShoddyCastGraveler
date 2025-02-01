use std::sync::mpsc;
use rand::Rng;
use std::thread;

fn main() {
    let mut highest_roll = 0;

    use std::time::Instant;
    let now = Instant::now();

    fn roll_dice(amount_of_trys: i32, rng: &mut rand::rngs::ThreadRng) -> (i32) {
        let mut highest_roll = 0;
        let mut rolls = 0;
        let mut number = 0;
        while highest_roll <= 177 && rolls < amount_of_trys {
            for i in 0..231 {
                let roll = rng.random_range(1..5);
                match roll {
                    1 => {number += 1},
                    _ => {}
                }
            }
            rolls += 1;
            if number > highest_roll {
                highest_roll = number;
            }
            number = 0;
        }
        return highest_roll;
    }
    fn find_highest_number(vector: &Vec<i32>) -> i32 {
        let mut highest_number = 0;
        for i in 0..vector.len() {
            if highest_number < vector[i] {
                highest_number = vector[i];
            }
        }
        return highest_number;
    }


    let num_threads = 16;
    let (tx, rx) = mpsc::channel(); // Channel for collecting results

    let mut handles = Vec::new();

    for i in 0..num_threads {
        let tx_clone = tx.clone();
        let handle = thread::spawn(move || {
            let result = roll_dice(1000000000/num_threads, &mut rand::rng());
            tx_clone.send(result).unwrap();
        });
        handles.push(handle);
    }
    // Drop the original sender so the receiver can exit the loop when all messages are received
    drop(tx);

    let results: Vec<i32> = rx.iter().collect();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Results: {:?}", results);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}