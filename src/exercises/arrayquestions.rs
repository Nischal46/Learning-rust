// there are several ways to write array

pub fn init() {
    println!("------ | array concept | -------");
    let mut arr_by_invoking: [u32; 6];

    arr_by_invoking = [12, 43, 54, 25, 36, 87];

    arr_by_invoking[2] = 56;

    //sorting method

    arr_by_invoking.sort();

    println!("Logging of the arr: {:?}", arr_by_invoking);

    //slices method

    let contain = arr_by_invoking.contains(&3);

    println!("At last after using of the slices: {}", contain);

    //first and last method

    let first = arr_by_invoking.first();
    let last = arr_by_invoking.last();

    println!(
        "displaying of the first and the last: {:?} , {:?}",
        first, last
    );
}
