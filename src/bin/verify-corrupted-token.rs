use std::io;

use timestamp_verify::sign;

fn get_int_from_stdin() -> i64 {
    let mut iline = String::new();
    io::stdin()
        .read_line(&mut iline)
        .expect("Couldn't read input");

    iline.trim().parse().expect("Couldn't parse input")
}

fn main() {
    println!("Timestamp?");
    let dt = get_int_from_stdin();

    let signature = sign(dt);

    println!("Signature Should Look Like {}", signature);
}
