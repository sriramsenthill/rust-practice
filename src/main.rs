fn main() {
    let large_number1: u64 = 0xffff_fffe; // Changed to u64
    let large_number2: u64 = 3000000000; // Changed to u64
    println!("Large Number 1: {}", large_number1);
    println!("Large Number 2: {}", large_number2);
    let result = large_number1 * large_number2;
    println!("Mul. Result: {}", result);
}
