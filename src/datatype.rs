pub fn rust_data_type() {
    //number type

    let mut a = 23;
    a = 54; //this is mutable because of mut keyword
    println!("logging value of a: {}", a);

    let mut f = 21.22;
    f = 22.34;
    println!("Logging value of f: {}", f);

    //string type

    let mut str = "i am nischal";

    str = "ae";
    println!("{}", str);

    let mut string_concept = String::from("This is string concept.");
    string_concept.push_str(" can be added text later if mut");
    string_concept.push('!');
    println!("{}", string_concept);

    //tuple comcept

    let tup_concept: (&str, u8, bool) = ("nischal", 24, true);

    println!("{:?}", tup_concept);

    //array concept

    let arr: [i32; 5] = [12, 32, 43, 56, 65];

    println!("Arr: {:?}", arr);
}
