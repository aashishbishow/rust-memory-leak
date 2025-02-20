fn helper<'a, 'b, T>(_: &'a &'b (), v: &'b T) -> &'a T {
  v
}

fn make_static<'a, T>(input: &'a T) -> &'static T {
    let f: fn(_, &'a T) -> &'static T = helper;
    f(&&(), input)
}

fn main() {
    let vec = vec![0; 1<< 20];
    let evil = make_static(&vec);
    drop(vec);
    println!("{:?}", evil);
}
