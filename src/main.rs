pub mod closureconcept;
pub mod datatype;
pub mod dsa;
pub mod enumconcept;
pub mod generics;
pub mod inputfromstd;
pub mod loopconcept;
pub mod matchcondition;
pub mod minigrep;
pub mod modandfunction;
pub mod structconcept;

fn main() {
    //datatype::rust_data_type();
    //modandfunction::check_user_details("nischal@dev.com".to_owned(), "qwerty");
    //inputfromstd::take_input_from_user();
    // //loopconcept::loop_exercise();
    // closureconcept::closure_concept();
    // matchcondition::match_condition();
    // enumconcept::enum_concept();
    //struct_conceptructconcept::struct_concept();
    //generics::generic_concept::generic_fn();
    minigrep::minigrep();
    dsa::dsa();
    structconcept::struct_concept();
}
