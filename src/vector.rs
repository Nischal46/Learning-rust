pub fn vector_array(arr: [i32; 4]){
    let mut new_vector_Array = vec![1,2,3,4,5];

    println!("Displaying of the data inside of the vector before merging, {:?}", new_vector_Array);
    println!("Accepting as parameter of array: {:?}", arr);
    println!("-----------------------------------------------");

    for num in arr {
        new_vector_Array.push(num);
    }

    println!("At last after meging array in vector in heap---- {:?}", new_vector_Array);
}
