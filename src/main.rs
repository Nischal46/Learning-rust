#[derive(Debug)]
enum Payment {
    Cash, 
    Esewa,
    Khalti
}

#[derive(Debug)]
struct Product <'a> {
    product_type: &'a str,
    price: i32
}

fn main(){
    let product_array: Vec<Product> = vec![{Product {
        product_type: "Tshirt",
        price: 1200
    }}, {Product {
        product_type: "Cardo pants",
        price: 1800
    }}];

    let user_choice = &product_array[1];

    println!("Product array: {:#?}", product_array);

    //if greater than 1500 paid with wallet otherwise cash
    if user_choice.price > 1500 {
        println!("{:#?}", Payment::Esewa);
    }
    else {
        println!("{:#?}", Payment::Cash);
    }

   }
