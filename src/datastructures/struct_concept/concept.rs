#[derive(Debug)]
struct Sport<'a> {
    title: &'a str,
    player: Vec<Option<&'a str>>,
    country: Vec<Option<&'a str>>,
}

impl<'a> Sport<'a> {
    fn new(title: &'a str) -> Self {
        Sport {
            title: title,
            player: vec![None; 5],
            country: vec![None; 3],
        }
    }

    fn add_player(&mut self, player_name: &'a str) {
        self.player.push(Some(player_name));
    }
}

pub fn concept() {
    println!("------------------Struct concept ---------------");
    // NOTE: This is also used to make object

    let mut init = Sport::new("UFC");

    init.add_player("Nischal");

    println!("Logging of struct ---{:?}", init);
}
