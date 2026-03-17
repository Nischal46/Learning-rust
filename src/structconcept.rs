// struct concept

#[derive(Debug)]
struct DayStruct<'a> {
    month: u8,
    day: &'a str,
}

pub fn struct_concept() {
    let mut vec_day: Vec<DayStruct<'_>> = Vec::new();

    let obj = DayStruct {
        month: 3,
        day: "Tuesday",
    };

    let obj2 = DayStruct {
        month: 3,
        day: "friday",
    };

    vec_day.push(obj);
    vec_day.push(obj2);

    println!("Logging of vec_day: {:?}", vec_day);
}
