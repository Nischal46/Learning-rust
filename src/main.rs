pub mod closureconcept;
pub mod datatype;
pub mod enumconcept;
pub mod inputfromstd;
pub mod loopconcept;
pub mod matchcondition;
pub mod modandfunction;
pub mod structconcept;

fn main() {
    //datatype::rust_data_type();
    //modandfunction::check_user_details("nischal@dev.com".to_owned(), "qwerty");
    //inputfromstd::take_input_from_user();
    //loopconcept::loop_exercise();
    closureconcept::closure_concept();
    matchcondition::match_condition();
    enumconcept::enum_concept();
    structconcept::struct_concept();
}
