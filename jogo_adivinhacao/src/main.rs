use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("\nBem-vindo ao Jogo de Adivinhação");

    let numero_secreto: u32 = rand::thread_rng().gen_range(0..=100);

    loop {
        let mut chute = String::new();
        println!("\nDigite seu chute: ");

        io::stdin()
            .read_line(&mut chute)
            .expect("Entrada inválida");

        let chute: u32 = match chute.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\nDigite um número!");

                continue;
            },
        };

        match chute.cmp(&numero_secreto) {
            Ordering::Less => println!("\nMuito baixo!"),
            Ordering::Greater => println!("\nMuito alto!"),
            Ordering::Equal => {
                println!("\nGanhou!\n");

                break;
            },
        }
    }
}
