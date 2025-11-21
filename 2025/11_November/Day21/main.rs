#[derive(Debug)]
struct Movie {
    title: String,
    genre: String,
    duration: u8
}

trait IMDB {
    fn imdb_rating(&self) -> String;
}

impl IMDB for Movie {
    fn imdb_rating(&self) -> String {
        let response = format!("{} has got {} reviews", &self.title, 8);
        response
    }
}

fn main() {
    let movie1 = Movie {
        title: "Intestellar".to_string(),
        genre: "Sci-fi".to_string(),
        duration: 120
    };

    println!("{:#?}", movie1);
    println!("{}", movie1.imdb_rating());

    let data1 = 22;
    let data2 = 34;
    moving_ownership(data1);
    borrow_only(&data2);
    println!("At first data1 has ownership inside main function scope")
}

fn moving_ownership(inp: i8){
    println!("After moving ownership data1 has transferred to moving_ownership and value is {}", inp)
}

fn borrow_only(inp: &i8){
    println!("Only borrowing reference {}", inp);
}