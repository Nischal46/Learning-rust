mod datastructures;
//mod projects;
mod algorithm;

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
    //
    // datastructures::enum_concept::concept1::concept1();
    // datastructures::hashmap_concept::concept::concept();
    // datastructures::trait_concept::concept::concept();
    //datastructures::box_concept::concept::concept();

    //datastructures::struct_concept::concept::concept();
    //algorithm::queue::linearqueue::init();
    //datastructures::anonymous_fn_concept::init::init();
    datastructures::thread_concept::init::init();
}
