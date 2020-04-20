use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::process;

fn main() {
    let mut pozicia_x;
    let mut pozicia_y;
    let panak = 1;
    println!("Program vygeneruje svet a panaka.");
    println!("Zadaj rozmer stvorcoveho sveta: ");
    let mut rozmer = String::new();
        
    io::stdin().read_line(&mut rozmer)
        .expect("Failed to read line");

    let rozmer: usize = match rozmer.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Nespravne zadana hodnota !\n");
                return;
            },
    };
    let mut svet= vec![vec![0; rozmer+panak]; rozmer+panak];
    if rozmer % 2 == 0 {
        pozicia_x = rozmer/2 - panak;
        pozicia_y = pozicia_x;
        svet[pozicia_x][pozicia_y] = panak;
    }

    else {
        pozicia_x = (rozmer - panak) / 2 * panak;
        pozicia_y = pozicia_x;
        svet[pozicia_x][pozicia_y] = panak;
    }
    
    let mut kroky = 0;        

    while (pozicia_x + pozicia_y) > 0 && (pozicia_x + pozicia_y) < 2 * rozmer - 2 * panak  {
        if svet[0][0] == panak || svet[rozmer][rozmer] == panak {
            break;
        }
        
        let smer = rand::thread_rng().gen_range(1, 5);
        if smer == 1 && pozicia_x < rozmer - panak {
            pozicia_x += 1;
            kroky += 1;
            svet[pozicia_x][pozicia_y] += panak;
            
        }

        if smer == 2 && pozicia_x > 0 {
            pozicia_x -= 1;
            kroky += 1;
            svet[pozicia_x][pozicia_y] += panak;
            
        }

        if smer == 3 && pozicia_y < rozmer - panak {
            pozicia_y += 1;
            kroky += 1;
            svet[pozicia_x][pozicia_y] += panak;
            
        }

        if smer == 4 && pozicia_y > 0 {
            pozicia_y -= 1;
            kroky += 1;
            svet[pozicia_x][pozicia_y] += panak;
            
        }

    }
    print!("\n");
    for index in 0..rozmer {
        for index_2 in 0 .. rozmer{
            print!("{:3}", svet[index_2][index]);
        }
        print!("\n");
    }

    print!("\n");
    match svet[0][0].cmp(&panak) {
        Ordering::Less => println!("Presiel {} krokov a prisiel domov.", kroky),
        Ordering::Greater => {println!("Chyba!!!");
                              process::exit(1);},
        Ordering::Equal => println!("Presiel {} krokov a prisiel do baru.", kroky),
    }
}
