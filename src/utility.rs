
pub fn print_as_board(number: u64) {
    // Convert the number to a 64-bit binary string, padded with zeros if necessary
    let binary_string = format!("{:064b}", number);
    println!("   a b c d e f g h");
    for row in (0..8).rev() {
        print!("{}| ", row+1);
        for col in (0..8).rev() {
            let index = row * 8 + col;
            print!("{} ", &binary_string[index..index + 1]);
        }
        println!();
    }
    println!("   a b c d e f g h");
    println!("Bitboard: {}", number);

}

