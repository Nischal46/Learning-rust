pub fn vector_array(arr: [i32; 4]){
    let mut new_vector_Array = vec![1,2,3,4,5];

    println!("Displaying of the data inside of the vector before merging, {:?}", new_vector_Array);
    println!("Accepting as parameter of array: {:?}", arr);
    println!("-----------------------------------------------");

    for num in arr {
        new_vector_Array.push(num);
    }

    println!("At last after meging array in vector in heap---- {:?}", new_vector_Array);

    println!("Calling of fn inside from another module");
    println!("---------------------------------------------");


    let product1 = Product::new("Laptop".to_string(), 65000, "Dell".to_string());
    println!("Getting detail from constructor function --- {:?}", product1);
    println!("Getting value from method also------ {:?}", product1.product_details());
}

#[derive(Debug)]
struct Product {
    item: String,
    price: i32,
    brand: String
}

impl Product {
    // add code here
    //making as constructor function

    fn new(item: String, price: i32, brand: String) -> Self {
        Self {
            item: item.to_string(),
            price: price,
            brand: brand.to_string(),
        }
    }

    fn product_details(&self) {
        println!("--------Product Details-----------");
        println!("Item: {}, Price: {}, Brand: {}", self.item, self.price, self.brand);
    }
}
