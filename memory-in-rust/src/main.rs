fn main() {
    let rows: i32 = 6;
    let cols: i32 = 6;
    let mut board = [[mut char; cols]; rows];
    println!("Bienvenido a memorama!");
    for _ in 1..rows {
        for _ in 1..cols {
            print!("@ ");
        }
        println!();
    }
}
