//NOTE: Lifetimes
fn main() {
    let x = 15;
    let y = 22;

    let single = Borrowed(&x);
    let double = ObjectLike {x: &x, y: &y};

    println!("x is borrowed in {:?}", single);
    println!("x and y are borrowed in {:?}", double);
    
}

#[derive(Debug)]
struct Borrowed<'a>(&'a i32);

#[derive(Debug)]
struct ObjectLike<'a> {
    x: &'a i32,
    y: &'a i32
}