pub mod datatype;
pub mod modandfunction;

fn main() {
    datatype::rust_data_type();
    modandfunction::check_user_details("nischal@dev.com".to_owned(), "qwerty");
}
