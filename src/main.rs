use std::env;

fn print_square(size: usize) {
    for _ in 0..size {
        for _ in 0..size {
            print!("* ");
        }
        println!();
    }
}

fn print_triangle(size: usize) {
    for i in 0..size {
        for _ in 0..i+1 {
            print!("* ");
        }
        println!();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <pattern> <size>", args[0]);
        println!("Pattern can be 'square' or 'triangle'");
        return;
    }

    let pattern = &args[1];
    let size: usize = args[2].parse().unwrap_or(0);

    match pattern.as_str() {
        "square" => print_square(size),
        "triangle" => print_triangle(size),
        _ => println!("Unknown pattern '{}'. Use 'square' or 'triangle'.", pattern),
    }
}
