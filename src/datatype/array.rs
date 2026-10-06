pub fn init() {
    let menu = ["Home", "settings", "profile", "about"];

    let sorted_array = [23, 34, 56, 67, 78, 90];

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
}
