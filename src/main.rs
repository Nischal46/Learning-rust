#[derive(Debug)]
struct Student<'a> {
    name: &'a str,
    email: &'a str,
}

#[derive(Debug)]
struct StudentArray<'a> {
    data: Vec<Student<'a>>,
    total_Students: usize,
}

impl<'a> StudentArray<'a> {
    fn add(&mut self, inp: Student<'a>) {
        self.data.push(inp);
        self.total_Students += 1;
    }
}

fn main() {
    println!("-----using cell to change specific data of object------");

    let mut student_array = StudentArray {
        data: Vec::with_capacity(5),
        total_Students: 0,
    };

    let student_one = Student {
        name: "Nischal",
        email: "nischal@dev.com",
    };

    let student_two = Student {
        name: "Baniya",
        email: "baniya@dev.com",
    };

    student_array.add(student_one);
    student_array.add(student_two);

    println!("Logging of the vec: {:?}", student_array);
}
