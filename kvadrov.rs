use std::io;
use std::process

fn main() {
    println!("**********************************");
    println!("Program riesi kvadraticke rovnice.");
    println!("**********************************\n");   
    println!("Zadaj clena a:");
    let a = input();
    println!("Zadaj clena b:");
    let b =input();
    println!("Zadaj clena c:");
    let c = input();
    println!("Je nacitana rovnica: {}x^2 + ({}x) + ({}) = 0\n", a, b, c);
    if a == 0.0 {
        if b == 0.0 {
            if c == 0.0 {
                println!("Rovnica ma nekonecne vela rieseni!");
                return;
            }
            println!("Rovnica nema riesenie!");
            return;
        }
        let x = (-1.0 * c) / b;
        println!("Rovnica je linearna a ma jedno riesenie: x = {}.",x);
        return;
    }

    let d: f64 = b.powf(2.0) - 4.0 * a * c;

    if d > 0.0 {
        let x1: f64 = (-1.0 * b + d.sqrt()) / (2.0 * a);
        let x2: f64 = (-1.0 * b - d.sqrt()) / (2.0 * a);

        println!("Riesenim kvadratickej rovnice su korene x1 = {} a x2 = {}", x1, x2);
    }
    else if d == 0.0 {
        let xd = (-1.0 * b) / (2.0 * a);

        println!("Riesenie je koren x1,2 = {}, tzv dvojnasobny koren.", xd);
    }
    else {
        let d = -1.0 * d;
        let xr = b / (2.0 * a);
        let xi = d.sqrt() / (2.0 * a);

        println!("Korene rovnice lezia v obore C (komplexnych cisel), xr = {}, xi = +/-{}.", xr, xi);
    } 
}

fn input() -> f64 {
    let mut i = String::new();

    io::stdin().read_line(&mut i)
        .expect("Failed to read line");

    let i: f64 = match i.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Zle zadana hodnota!");
            process::exit(1);
        },
    };
        return i;
}
