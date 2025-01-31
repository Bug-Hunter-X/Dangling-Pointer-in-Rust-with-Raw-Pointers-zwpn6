fn main() {
    let mut v = vec![1, 2, 3];
    let mut v2 = v.clone();
    let ptr = v2.as_mut_ptr();
    unsafe {
        *ptr = 42; 
    }
    println!( "{:?}", v2); // v2 is now [42, 2, 3]
    v.push(4);
    println!( "{:?}", v);
    println!( "{:?}", v2); // v2 is independent
}