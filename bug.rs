fn main() {
    let mut v = vec![1, 2, 3];
    let ptr = v.as_mut_ptr();
    unsafe {
        *ptr = 42; // Modifying through raw pointer
    }
    println!( "{:?}", v); // v is now [42, 2, 3]
    v.push(4);
    println!( "{:?}", v); // This might panic due to dangling pointer
}