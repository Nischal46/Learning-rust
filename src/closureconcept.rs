pub fn closure_concept() {
    //it is anonymous fn

    //normal
    let closure_fn = |x, y| {
        println!(
            "This is closure concept. user had passed {} and {} as parameter. ",
            x, y
        )
    };

    closure_fn(2, 3);

    //used in iterator

    let mut arr = [1, 2, 5, 4, 3, 9, 8, 7];

    let result = arr.map(|x| {
        if x == 3 {
            println!("Jackpot: found {}", x);
            arr[x - 1] = 22;
        };

        x + 1
    });

    println!("After adding 1 in each iter: {:?}", result);
    println!("Printing original array: {:?}", arr);
    println!("total length of arr: {}", arr.len());

    arr.sort();

    println!("Sorted arr: {:?}", arr);

    fn special_closure_fn() {
        let mut count = 0;

        let mut increment_closure = |x| {
            println!("Accepting as param in closure: {}", x);
            count += x
        };

        for n in 1..5 {
            increment_closure(n);
            println!("for iteration: {}", n);
        }

        println!("At last closure concept total: {}", count);
    }

    special_closure_fn();
}
