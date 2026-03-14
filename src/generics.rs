pub mod generic_concept {

    use std::ops::Add;

    #[derive(Debug)]
    struct Coordinates<T> {
        x: T,
        y: T,
    }

    #[derive(Debug)]
    enum Input<T> {
        Some(T),
        None,
    }

    fn sum<T: Add<Output = T>>(inp1: T, inp2: T) -> T {
        inp1 + inp2
    }

    pub fn generic_fn() {
        println!("Generic fn called");

        let coordinate_int_obj = Coordinates { x: 2, y: 3 };

        println!("Logging of coordinate_obj: {:?}", coordinate_int_obj);

        let coordinate_float_obj = Coordinates { x: 1.20, y: 3.30 };

        println!(
            "Logging of coordinate_float_obj: {:?}",
            coordinate_float_obj
        );

        let user_inp = Input::Some("Hello");
        println!("Logging enum: {:?}", user_inp);

        let user_int_inp = Input::Some(23);
        println!("Logging enum user_int_inp: {:?}", user_int_inp);

        let user_float_inp = Input::Some(25.45);
        println!("Logging enum user_float_inp: {:?}", user_float_inp);

        let add_generic_fn = sum(12, 13);
        println!("using generics sum fn: {}", add_generic_fn);

        //generate error string has refernce
        // let string_concatinate = sum("nischal", " baniya");
        // println!("string concatinate: {}", string_concatinate);
    }
}
