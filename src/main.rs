fn main(){
    //about string concept

    let mut word_1 = String::from("Hello");
  //  println!("Before adding Word_1: {}", word_1);

    word_1.push_str(" lll");

    word_1.push('r');

    println!("After adding: Word_1: {}", word_1);

    let a1 = String::from("coffee");
    let a2 = String::from("with");
    let a3 = String::from("Code");

    let a4 = a1 + "-" + &a2 + "-" + &a3; //take ownership
    println!("concatenation concept -- {}", a4);                                         
    
    let a1 = "nischal";
    let a2 = "guitar";
    let a3 = "code";

    let final_word = format!("{}-plays-{}-and {}", a1, a2, a3);

    println!("without taking ownership: {}", final_word);

    let hello = "Здравствуйте";
let answer = &hello[0];

println!("{}", answer);

}
