use std::fs;

fn main() {
    let mut path = String::new();
    std::io::stdin().read_line(&mut path).unwrap();

    match fs::read(path.trim()) {
        Ok(_) => println!("success"),
        Err(_) => println!("failure"),
    }
}
