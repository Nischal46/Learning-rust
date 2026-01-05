fn main() {
    //revising of the anonymous function

    let x = vec![1, 2, 3, 4, 5];

    let iterVectorData: Vec<_> = x.iter().map(|x| x * 2 ).collect();

    let an_x = |x| x+2;

    let new = an_x(5);
    println!("{}", new);

    println!("{:#?}", iterVectorData);

    assert_eq!(x, iterVectorData);
}