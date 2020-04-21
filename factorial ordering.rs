use std::io;
use std::cmp::Ordering;
use std::process;

fn main() {
    println!("Faktorial cisla.\n");
    println!("Zadaj cele kladne cislo do 20: ");

    let mut hodnota = String::new();
    let mut pomocna: u64 = 1;
    let nula: u64 = 0;

    io::stdin().read_line(&mut hodnota)
        .expect("Failed to read line");

    let hodnota: u64 = match hodnota.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Nespravne zadana hodnota!");
            process::exit(1);
        },
    };

    
    match hodnota.cmp(&nula) {
        Ordering::Equal =>{
            println!("Faktorial 0 je 1.");
        },
        Ordering::Greater => {
            if hodnota > 20 {
                println!("64 bit unsigned limit je 20!.");
                return;
            }
            for i in 1..hodnota {
                pomocna *=  i + 1;
            }
            println!("Faktorial {} je {}.", hodnota, pomocna);
        },
        Ordering::Less =>  {
            println!("Faktorial je mozny iba pri kladnych celych cislach.");
        }
    
    }
}
