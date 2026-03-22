// struct concept

#[derive(Debug)]
struct Product<'a> {
    title: &'a str,
    brand: &'a str,
    price: u32,
}

pub fn struct_concept() {
    let mut product_vector_list: Vec<Product<'_>> = Vec::new();

    let product1 = Product {
        title: "Laptop",
        brand: "Dell",
        price: 62000,
    };

    let product2 = Product {
        title: "Mouse",
        brand: "Fantech",
        price: 1500,
    };

    product_vector_list.push(product1);
    product_vector_list.push(product2);

    println!(
        "Logging of the product vector ------ {:?}",
        product_vector_list
    );
}
