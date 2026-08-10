pub fn init() {
    //array in rust

    println!("This is array in rust");
    let mut st_array = [std::option::Option::None; 5];

    st_array[0] = Some(Student {
        roll_no: 1,
        student_name: "nischal",
    });
    st_array[1] = Some(Student {
        roll_no: 2,
        student_name: "baniya",
    });

    println!("Logging of the array: {:?}", st_array);
}

#[derive(Clone, Copy, Debug)]
struct Student<'a> {
    roll_no: u32,
    student_name: &'a str,
}
