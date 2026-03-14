// struct concept

#[derive(Debug)]
struct ExercisePrototype<'a> {
    title: &'a str,
    sets: u8,
    reps: u8,
}

impl<'a> ExercisePrototype<'a> {
    fn new(title: &'a str, sets: u8, reps: u8) -> Self {
        Self { title, sets, reps }
    }

    fn announce(&self, name: &'a str) -> String {
        let res = format!(
            "{} has done {} of {} sets with {} reps each",
            name, self.title, self.sets, self.reps
        );

        res
    }
}

pub fn struct_concept() {
    const MAIN_LIFETIME: &'static str = "Hello, I am lifetime";
    let exercise_detail = ExercisePrototype::new("Bench press", 3, 12);
    println!("Exercise detials: {:?}", exercise_detail);
    outside_fn(MAIN_LIFETIME);
    println!("Targeting lifetime: {}", MAIN_LIFETIME);

    let res = exercise_detail.announce("Nischal");
    println!("Associative fn method: {}", res);
}

fn outside_fn(lifetime: &str) {
    println!("From other fn: {}", lifetime);
}
