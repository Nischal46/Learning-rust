#[allow(unused_variables)]
pub fn loop_exercise() {
    println!(
        "
        ------------ choose option below --------- 
        1. loop for 15 times 
        2. loop for 25 times 
        3. loop for 35 times 
        "
    );

    let input = &mut String::from("");
    std::io::stdin().read_line(input).unwrap();
    let num_input = input.replace("\n", "").parse::<u8>().unwrap();

    if num_input == 1 {
        println!("You had choosen option 1.");
        let mut j = 1;
        for i in 1..16 {
            println!("using FOR loop: {} time ", j);

            j += 1;
        }

        println!("Dantana FOR loop ends");
    } else if num_input == 2 {
        println!("You had choosen option 2.");
        let mut i = 1;

        while i < 26 {
            println!("using while loop: {} time", i);
            i += 1;
        }

        println!("Dhantana while loop ends");
    } else if num_input == 3 {
        println!("You had choosen option 3.");
        let mut i = 1;
        let mut bool;

        loop {
            if i == 36 {
                break;
            }

            if i % 2 == 0 {
                bool = "Even";
            } else {
                bool = "Odd";
            }

            println!("using only loop: {} times -> {}", i, bool);
            i += 1;
        }

        println!("Dhantan naaa. Loop ends");
    }
}
