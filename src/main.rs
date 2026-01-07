//Error management

enum ATMError {
    InsufficientBalance,
    InvalidPIN,
}

fn with_draw(amount: i32, pin: i32) -> Result<i32, ATMError> {
    let balance = 100000;
    if pin != 1234 {
        return Err(ATMError::InvalidPIN);
    }

    if amount > balance {
        return Err(ATMError::InsufficientBalance);
    }

    Ok(balance - amount)
}

fn main() {

    let result = with_draw(25000, 1234);

    match result {
        Ok(balance) => {
            println!("{}", balance);
        }

        Err(ATMError::InvalidPIN) => {
            println!("Invalid PIN");
        }

        Err(ATMError::InsufficientBalance) => {
            println!("Insufficient Balance");
        }
    }

    let content = std::fs::read_to_string("config.txt").expect("Failed to read config.txt file");
    println!("{:?}", content);


}


//Key of understanding
// panic means to abort whole program and it is unrecoverable
