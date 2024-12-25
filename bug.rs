fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &mut x; // z is also a mutable reference to x

    *y += 1; // This is allowed and modifies x
    *z += 1; // This will cause a compiler error because multiple mutable references to x exist 
}