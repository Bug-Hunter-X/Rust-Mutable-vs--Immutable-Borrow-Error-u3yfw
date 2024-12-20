fn main() {
    let mut x = 5;
    let y = &mut x; // mutable reference
    let z = &x; // immutable reference

    *y += 1; // Modifying x through y
    println!("x = {}", x); // Output: x = 6

    // Following line will cause error because z is immutable
    *z += 1; 
}