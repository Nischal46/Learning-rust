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
}