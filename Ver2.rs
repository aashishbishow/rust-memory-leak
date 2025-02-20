fn make_static_safe<T>(input: T) -> &'static T {
    Box::leak(Box::new(input))
}

fn main() {
    let evil = make_static_safe(vec![0; 1 << 20]); // Safe static reference
    println!("{:?}", evil);
}
