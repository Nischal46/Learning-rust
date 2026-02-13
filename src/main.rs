fn main(){
    let arry_num_1 = vec![22,14,6,56,32];

    let arry_num_2 = vec![99, 544, 56, 7777, 3, 5,2,2];

    let arry_num_1_respose = find_largest(&arry_num_1);
    let arry_num_2_respose = find_largest(&arry_num_2);

    let char_list = vec!['w', 'z', 'c', 'b'];

    let char_list_response = find_largest(&char_list);

    println!("Logging of the array 1 response: {}", arry_num_1_respose);
    println!("Logging of the array 2 response: {}", arry_num_2_respose);
    println!("Logging largest char: {}", char_list_response);
}

//Implementing generics types concept
fn find_largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut highest_num = list[0];

    for item in list.iter(){
        if item > highest_num {
            highest_num = item
        }
    }

    &highest_num
}
