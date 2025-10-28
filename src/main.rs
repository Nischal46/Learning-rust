#[derive(Debug)]
struct OS {
    name: String,
    launch_year: u16,
    laptop: OSType,
}

#[derive(Debug)]
enum OSType {
    Dell,
    Mac,
    Lenovo,
}

fn main() {
    let os_obj = OS {
        name: "Linux".to_string(),
        launch_year: 1990,
        laptop: OSType::Dell,
    };

    let ownership_change = &os_obj;

    println!("Logging of the object ---- {:#?}", ownership_change);
    println!("Logging of the object ---- {:#?}", os_obj);

    for n in 0..10 {
        println!("{}n loop", n)
    }

    let mut vec_alloc = Vec::<i32>::with_capacity(5);

    for n in 10..18 {
        vec_alloc.push(n);
    }

    println!("Printing vec_alloc after inserting: {:#?}", vec_alloc);
}
