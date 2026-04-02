mod projects;

fn main() {
    let name = "Nischal";
    println!("Hello rust developer {}", name);

    projects::todo::todo::main_todo();
}
