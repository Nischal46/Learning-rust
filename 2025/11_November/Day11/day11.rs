//learning vector data type

fn main() {
    let mut vec_data_type = Vec::new();
    vec_data_type.push(1);
    vec_data_type.push(3);

    //basically this tries to check left with right if true program continues 
    //or if false thread panicked

    //assert! or assert_eq! are macro for testing and even debugging
    assert_eq!(vec_data_type.len(), 2); //true
    // assert_eq!(vec_data_type.len(), 3); //false :-> thread panicked

    assert_eq!(vec_data_type[1], 3);

    assert_eq!(vec_data_type.pop(), Some(3));

    vec_data_type.extend([3,1,2,4,5]);
    println!("{:#?}", vec_data_type)

   
}