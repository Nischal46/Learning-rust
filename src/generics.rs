pub mod generic_concept {

    #[derive(Debug)]
    struct Rectangle<T, S> {
        length: T,
        breadth: S,
    }

    impl<T, S> Rectangle<T, S>
    where
        T: std::fmt::Debug,
        S: std::fmt::Debug,
    {
        fn describe(&self) {
            println!("Length: {:?} and breadth: {:?}", self.length, self.breadth);
        }
    }

    pub fn generic_fn() {
        let area_rectangle = Rectangle {
            length: 20,
            breadth: 30,
        };

        area_rectangle.describe();
    }
}
