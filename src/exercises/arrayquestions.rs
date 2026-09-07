#[derive(Debug)]
struct Accessories<'a> {
    title: &'a str,
    price: &'a str,
    brand: &'a str,
}

#[derive(Debug)]
struct ArrayContainer<'a> {
    data: [Option<Accessories<'a>>; 5],
}

impl<'a> ArrayContainer<'a> {
    fn init() -> Self {
        // Self {
        //     data: [Some(Accessories {
        //         title: "Lovely laptop",
        //         price: "68000",
        //         brand: "Dell",
        //     }); 5],
        // }
        Self {
            data: std::array::from_fn(|_| None),
        }
    }

    fn add_data_in_specific_index(&mut self) {
        for (ind, it) in self.data.iter_mut().enumerate() {
            if ind == 2 {
                *it = Some(Accessories {
                    title: "Lovely my laptop",
                    price: "68000",
                    brand: "DELL",
                })
            }
        }
    }
}

pub fn init() {
    println!("This revision would contain array related solutions");

    let mut array_container = ArrayContainer::init();
    println!("Logging of the array container:");
    println!("{:?}", array_container);
    array_container.add_data_in_specific_index();
}
