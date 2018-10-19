extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value)
        }

        Guess { value }
    }
    pub fn value(&self) -> u32 {
        self.value
    }
}

fn main() {
    println!("Adivinhe o número:");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Digite seu palpite: ");

        let mut palpite = String::new();

        io::stdin()
            .read_line(&mut palpite)
            .expect("Falha ao ler o palpite");

        let palpite: Guess = match palpite.trim().parse() {
            Ok(num) => Guess::new(num),
            Err(_) => continue,
        };

        println!("Você disse: {}", palpite.value());

        match palpite.value().cmp(&secret_number) {
            Ordering::Less => println!("Muito baixo!"),
            Ordering::Greater => print!("Muito alto!"),
            Ordering::Equal => {
                println!("Você acertou!");
                break;
            }
        }
    }
}
