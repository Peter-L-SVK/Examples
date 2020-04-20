use std::io;
use std::io::Write; 

fn main() {
    let mut hodnota = String::new();
    let mut vysledky: Vec<u64> = Vec::new(); 

    println!("Collatzov algoritmus.\n");
    println!("Zadajte cele cislo: ");
    io::stdin().read_line(&mut hodnota)
        .expect("Failed to read line");

    let mut hodnota: u64 = match hodnota.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Nespravne zadana hodnota!");
            return;
        },
    };
    vysledky.push(hodnota);
    
    let mut cyklus = 0;

    while hodnota > 1 {
        if hodnota % 2 == 0 {
	    hodnota /= 2;
            cyklus += 1; 
        }
        else {
            if hodnota <= 1 {
	        break;
            }
	    hodnota = 3 * hodnota + 1;
            cyklus += 1;
        }
        vysledky.push(hodnota);
    }
    println!("\nVysledni clenovia postupnosti: \n");
    let mut pocet = 0;
    for index in 0..cyklus {
        print!("{}, ", vysledky[index]);
        io::stdout().flush().unwrap();
        pocet += 1;
    }
    println!("\n\nPotupnost ma {} clenov.", pocet);
}
