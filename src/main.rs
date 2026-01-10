enum ATMError {
    InvalidPIN,
    InsuffecientAmount
}

fn atm_machine (pin: i32, amount: i64) -> Result<i64, ATMError>{
    let available_amount = 1000;
    if pin != 1234 {
        return Err(ATMError::InvalidPIN);
    }

    if available_amount < amount {
        return Err(ATMError::InsuffecientAmount);
    }

    Ok(available_amount - amount)
}

fn main() {

    let withdraw_money = atm_machine(1237, 500);

    match withdraw_money {

        Ok(response) => {
            println!("{} withdraw successfully", response);
        }

        Err(ATMError::InsuffecientAmount) => {
            println!("Insufficient balance");
        }

        Err(ATMError::InvalidPIN) => {
            println!("Invalid PIN");
        }

    }
}
