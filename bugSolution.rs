fn main() {
    let mut x = 5;
    let y = &mut x; // mutable reference
    
    *y += 1; // Modifying x through y
    println!("x = {}", x); // Output: x = 6
    
    //To use immutable reference you need to create a copy of x and use it.
    let z = x;
    let z1 = &z;
    println!("z = {}", z);
    println!("z1 = {}", *z1);
} 