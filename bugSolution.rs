fn main() {
    let mut x = 5;
    {
        let y = &mut x; // y is a mutable reference to x
        *y += 1; // Modify x through y
    }
    {
        let z = &mut x; // z is now a mutable reference to x
        *z += 1; // Modify x through z
    }
    println!("x = {}", x); // x will be 7
} 