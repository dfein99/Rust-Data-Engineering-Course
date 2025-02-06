/*
This code starts with an initial VecDeque,
converts it to a Vec for shuffling, and then converts it back to a VecDeque.
After that, it pushes "Pomegranate" to the front of the deque, and "Fig" and "Cherry"
to the back of the deque. Finally, it prints out the final fruit salad.

A VecDeque is a double-ended queue, which means that you can push and pop from both ends
of the queue.
*/

use rand::seq::SliceRandom; // rand is a random number generation library in Rust
use rand::rng;
use std::collections::VecDeque;
use rand::rngs::StdRng;
use rand::SeedableRng;
use std::io;
use std::io::Write; // Important for flushing stdout

fn old_main() {
    let mut fruit: VecDeque<&str> = VecDeque::new();
    fruit.push_back("Arbutus");
    fruit.push_back("Loquat");
    fruit.push_back("Strawberry Tree Berry");

    // Scramble (shuffle) the fruit
    let mut rng = rng();
    let mut fruit: Vec<_> = fruit.into_iter().collect();
    fruit.shuffle(&mut rng);

    // Convert it back to VecDeque
    let mut fruit: VecDeque<_> = fruit.into_iter().collect();

    // Add fruits to the both ends of the queue after shuffling
    fruit.push_front("Pomegranate");
    fruit.push_back("Fig");
    fruit.push_back("Cherry");

    // Print out the fruit salad
    println!("Fruit Salad:");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }
}


fn main() {
    let mut rng = StdRng::from_seed([0; 32]);
    let mut fruit_vec = vec!["Arbutus", "Loquat", "Strawberry Tree Berry"];
    fruit_vec.shuffle(&mut rng);
    let mut fruit: VecDeque<&str> = fruit_vec.into_iter().collect();

    loop {
        print!("Add fruit to (front/back/done)? ");
        io::stdout().flush().unwrap(); // Crucial: Flushes the output buffer

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim().to_lowercase();

        if choice == "done" {
            break;
        } else if choice == "front" || choice == "back" {
            print!("Enter fruit name: ");
            io::stdout().flush().unwrap();

            let mut fruit_name = String::new();
            io::stdin().read_line(&mut fruit_name).unwrap();
            let fruit_name = fruit_name.trim();

            if choice == "front" {
                fruit.push_front(fruit_name);
            } else {
                fruit.push_back(fruit_name);
            }
        } else {
            println!("Invalid input. Please enter 'front', 'back', or 'done'.");
        }
    }

    println!("Fruit Salad:");
    println!("{}", fruit.iter().cloned().collect::<Vec<&str>>().join(", "));
}