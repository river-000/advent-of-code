use std::fs;

//use advent_of_code::parse_number;
use advent_of_code::parse_commasep_numbers;

pub fn parse(filename: &str) -> Result<Vec<u64>, ()> {
    let text = fs::read_to_string(filename).unwrap();

    match parse_commasep_numbers(&text) {
        Ok((_,x)) => {
            println!("{:?}", x);
            //Err(())
            Ok(x)
        }
        Err(_) => Err(())
    }
}
