use std::io;

fn main() {
    println!("**********************************");
    println!("Program riesi kvadraticke rovnice.");
    println!("**********************************\n");   
    println!("Zadaj clena a:");

    let mut a = String::new();

    io::stdin().read_line(&mut a)
        .expect("Failed to read line");

    let a: f64 = match a.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Zle zadana hodnota!");
            return;
        },
    };
    println!("Zadaj clena b:");

    let mut b = String::new();

    io::stdin().read_line(&mut b)
        .expect("Failed to read line");

    let b: f64 = match b.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Zle zadana hodnota!");
            return;
        },
    };
    println!("Zadaj clena c:");

    let mut c = String::new();

    io::stdin().read_line(&mut c)
        .expect("Failed to read line");

    let c: f64 = match c.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Zle zadana hodnota!");
            return;
        },
    };
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