pub mod closures_modules {
    pub fn closure_concept() {
        let add = |x, y| {x + y};
        let sum_result = add(2,3);

        println!("Logging of the sum of two number: {:?}", sum_result);
    }
}