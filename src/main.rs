mod datastructures;
//mod projects;

#[derive(Debug)]
enum Product {
    Monitor,
    Mouse,
}

fn main() {
    let name = "Nischal";
    println!("Hello rust developer {}", name);

    let choosen_product = Product::Monitor;

    println!("choosen product was: {:?}", choosen_product);

    //projects::todo::todo::main_todo();

    //projects::tictactoe::tictactoe::tictactoe();

    //datastructures::enum_concept::concept1::concept1();
    datastructures::hashmap_concept::concept::concept();
}
