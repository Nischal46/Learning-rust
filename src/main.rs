use std::{ops::AddAssign, sync::Mutex, thread, time::Duration};

fn main() {
    println!("Working in closure fn.........");

    let mutex_var = Mutex::new(0u16);

    let fn_call = || {
        let mut mut_ref = mutex_var.lock().unwrap();
        for i in 1..3 {
            println!("Closure fn runs... {} times", i);
            mut_ref.add_assign(i);
            thread::sleep(Duration::from_secs(3));
        }

        println!("Printing value of mutex {:?}", mut_ref);
    };

    let fn_call2 = || {
        println!("waitinf for closure 1 to finish");
        let mut another_mut_ref = mutex_var.lock().unwrap();
        println!("Closure fn 2");

        for i in 1..8 {
            println!("Closure fn 2 runs .... {} times", i);
            another_mut_ref.add_assign(i);
        }

        println!("Priniting value of mutex .... {:?}", another_mut_ref);
    };

    fn_call();
    fn_call2();
}

