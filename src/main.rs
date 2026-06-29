enum Continent {
    Asia,
    Africa,
    Europe,
    SouthAmerica,
    NorthAmerica,
}

#[derive(Debug)]
struct Country<'a> {
    country: &'a str,
    continent: &'a str,
}

#[derive(Debug)]
struct Team<'a> {
    data: Vec<Country<'a>>,
    count: usize,
}

fn main() {
    let country_one = Country {
        country: "Japan",
        continent: "Asia",
    };

    println!("{:?}", country_one);
}
