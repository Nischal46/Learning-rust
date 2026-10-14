//binary search algo

pub fn init(){
    let arr = [12,23,34,45,56,67];

    let mut lowerbound = 0;
    let mut upperbound = arr.len() - 1;
    let search_element = 23;

    while lowerbound <= upperbound {
        let middle = (lowerbound + upperbound)/2; 

        if arr[middle] == search_element {
            println!("found found at: {}", middle);
            return;
        }

        else if arr[middle] > search_element {
            upperbound = middle - 1;
        }
        else {
            lowerbound = middle + 1;
        }
    }

}