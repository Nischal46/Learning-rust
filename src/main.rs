//NOTE: The shortcut of thandling of the error in rust beside using match And return Result <T, E> is unwrap();
//unwrap acts similar to Result

use std::fs::File;

fn check_validation(a: u8, b: u8) -> Result<String, String> {
    if a > b {
        Err("Error alert. a smaller than b".to_string())
    }
    else{
        Ok("Pass. a smaller than b".to_string())
    }
}

fn main() {

    let a = 2 > 3;

    println!("{}", a);

    let a= 5;
    let b = 3;

    let calling_fn = check_validation(a, b);
    println!("Here here {:#?}", calling_fn);

    let mut vector = vec![1,2,3,4,5];

    for n in 10..15 {
        vector.push(n);
    }

    println!("After mutating vector array data {:?} ---- ", vector);

    // println!("{:#?}", open_file);
}