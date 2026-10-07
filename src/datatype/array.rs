use std::iter::Sum;

pub fn init() {
    let mut menu = ["Home", "settings", "profile", "about"];

    let mut sorted_array = [23, 34, 56, 67, 78, 90];

    println!("- - -");
    println!("User in bound");
    match menu.get(2) {
        Some(item) => println!("Selected: {}", item),
        None => println!("Invalid option selected"),
    }

    println!("- - -");
    println!("User not in bound");
    match menu.get(6) {
        Some(item) => println!("Selected: {}", item),
        None => println!("Invalid option selected"),
    }

    println!("- - -");
    println!("concept of first and last method");
    println!(
        "first element and last element of menu array are: {:?} & {:?}",
        menu.first(),
        menu.last()
    );

    println!("- - -");
    println!("contains method");
    println!("Yes, {} contains in {:?}", "Home", menu.contains(&"Home"));
    println!(
        "No, {} donot contains in {:?}",
        "contact",
        menu.contains(&"contact")
    );

    println!("- - -");
    println!("binary search method [built-in]");
    println!("{:?}", sorted_array.binary_search(&23));

    println!("- - -");
    menu.swap(1, 3);
    println!("After swapping of the element in array: {:?}", menu);

    println!("- - -");
    sorted_array.rotate_left(2);
    println!("After left rotating element in array: {:?}", sorted_array);

    println!("- - -");
    menu.rotate_right(2);
    println!("After right rotating of the array: {:?}", menu);

    //calculating of the statiscics
    let sum = sorted_array.iter().sum::<i32>();

    println!("Sum of whole total array: {}", sum);
}
