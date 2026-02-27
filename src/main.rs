fn main(){
    //unit type 
    let x: () = (); //this is like null type we cannot assign any value
    println!("{:?}", x);

    let f1: f32 = 22.9;
    println!("{}", f1);

    type_coersion();

    char_concept();

    string_concept();

    tuple_concept();

    array_concept();
}

fn type_coersion(){
    let x: f32 = 123.4;

    let y: u8 = x as u8 - 23;

    println!("{}", y);
}

fn char_concept(){
    let only_char: char = 'r';
    println!("{}", only_char);
}

fn string_concept(){

    //string contains two concept

    let my_string = "hello";
    println!("{}", my_string);
}

fn tuple_concept(){
    let datas: (&str, i8) = ("Nischal", 25);

    println!("{:?}", datas);
}

fn array_concept(){
    let arr: [i8; 5] = [12,23,34,45,56];
    println!("Print out array value: {:?}", arr);

    //slices concept

    let slices_arr = &arr[1..3];
    println!("Taking arr slices: {:?}", slices_arr);
}
