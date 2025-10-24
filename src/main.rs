fn main(){
    //vec! is rust macro that help to provide initial val
    let mut num = vec![2,3,5];
    num.push(22);

    // let third = &num[2];

    for i in &mut num {
        //NOTE: first dereference i
        *i += 3;
    }

    num.push(55);

    let mut v = Vec::new();
    // v.push(2);
    v.push("hello");

    //NOTE: Concept string

    let mut string_defined = String::from("hello");
    let new_string = "later added";
    string_defined.push_str(new_string);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    //NOTE: The key difference of using format macro is that 
    //it doesnot take ownership and it makes easy to read

    let s = format!("{}-{}-{}", s1, s2, s3);

    let ss = s1 + "*" + &s2 + "*" + &s3;

    println!("Printinf of the format macro, {}", s);
    println!("{}", ss);
    // println!("Logging of third element by referencing, {:?}", third);

    println!("Printing of vec data type: {:?}", num);
    println!("Printing of vec data type through new function, {:?}", v);

    println!("Logging of the string, {}", string_defined);
    println!("logging of the new string, {}", new_string);
}