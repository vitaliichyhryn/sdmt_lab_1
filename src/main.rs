use std::{env, error, fs, io, process};

fn get_discriminant(a: f64, b: f64, c: f64) -> f64 {
    b.powi(2) - 4.0 * a * c
}

fn get_quadratic_equation_roots(a: f64, b: f64, c: f64) -> (Option<f64>, Option<f64>) {
    let d = get_discriminant(a, b, c);

    match d.total_cmp(&0.0) {
        std::cmp::Ordering::Less => (None, None),
        std::cmp::Ordering::Equal => (Some(-b / (2.0 * a)), None),
        std::cmp::Ordering::Greater => (
            Some((-b + d.sqrt()) / (2.0 * a)),
            Some((-b - d.sqrt()) / (2.0 * a)),
        ),
    }
}

fn get_coef_from_stdin(name: &str) -> Result<f64, Box<dyn error::Error>> {
    println!("Enter coefficient {name}: ");

    let mut val = String::new();
    io::stdin().read_line(&mut val)?;

    let val: f64 = val.trim().parse()?;
    match (name, val) {
        ("a", 0.0) => Err("coefficient a should be non-zero".into()),
        _ => Ok(val),
    }
}

fn get_coefs_from_stdin() -> (f64, f64, f64) {
    let names = ["a", "b", "c"];
    let coefs: Vec<f64> = names
        .iter()
        .map(|&name| loop {
            match get_coef_from_stdin(name) {
                Ok(val) => break val,
                Err(err) => {
                    eprintln!("Error: {err}")
                }
            }
        })
        .collect();
    (coefs[0], coefs[1], coefs[2])
}

fn get_coefs_from_file(filename: &str) -> Result<(f64, f64, f64), Box<dyn error::Error>> {
    let coefs: Vec<f64> = fs::read_to_string(filename)?
        .split_whitespace()
        .map(str::parse)
        .collect::<Result<_, _>>()?;
    let count = coefs.len();
    if count != 3 {
        return Err(format!("expected 3 coefficients, found {count}").into());
    }
    match coefs[0] {
        0.0 => Err("coefficient a should be non-zero".into()),
        _ => Ok((coefs[0], coefs[1], coefs[2])),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let (a, b, c) = match args.len() {
        1 => get_coefs_from_stdin(),
        2 => {
            let filename = &args[1];
            match get_coefs_from_file(filename) {
                Ok(vals) => vals,
                Err(err) => {
                    eprintln!("Error: {err}");
                    process::exit(1);
                }
            }
        }
        _ => {
            eprintln!("Usage: {} [filename]", args[0]);
            process::exit(1);
        }
    };

    println!("Equation: ({a}) * x ^ 2 + ({b}) * x + ({c})");

    match get_quadratic_equation_roots(a, b, c) {
        (None, None) => println!("There are no real roots"),
        (Some(x), None) => println!("There is one real root: {x}"),
        (Some(x1), Some(x2)) => println!("There are two real roots: {x1} and {x2}"),
        _ => unreachable!("Unexpected root combination"),
    }
}
