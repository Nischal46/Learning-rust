#[derive(Debug)]
enum Role {
    User,
    Admin,
    Superadmin,
}

#[derive(Debug)]
enum Shape {
    Rectangle { length: i32, breadth: i32 },
    Square { length: i32 },
}

pub fn concept1() {
    println!("This is from inside of other");
    let choosen_role = Role::Admin;

    println!("Choosen role is {:?}", choosen_role);

    let choosen_shape = Shape::Rectangle {
        length: 5,
        breadth: 4,
    };
    println!("Choosen shape is {:?}", choosen_shape);
}
