use std::fs;

//use advent_of_code::parse_number;
use advent_of_code::parse_commasep_numbers;
use advent_of_code::parse_whitespacesep_numbers;
use advent_of_code::parse_grid_numbers;

pub fn parse_day4_grid(i: &str) -> nom::IResult<&str, Vec<Vec<u64>>> {
    let (i,_) = nom::character::complete::newline(i)?;
    let (i,_) = nom::character::complete::newline(i)?;
    let (i,g) = parse_grid_numbers(i)?;

    Ok((i,g))
}

pub fn parse_day4_grids(i: &str) -> nom::IResult<&str, Vec<Vec<Vec<u64>>>> {
    nom::multi::many1(parse_day4_grid)(i)
}

pub fn parse_day4(i: &str) -> nom::IResult<&str, (Vec<u64>, Vec<Vec<Vec<u64>>>)> {
    let (i,ns) = parse_commasep_numbers(i)?;
    let (i,gs) = parse_day4_grids(i)?;

    Ok((i,(ns,gs)))
}
  

pub fn parse(filename: &str) -> Result<Vec<u64>, ()> {
    let text = fs::read_to_string(filename).unwrap();

    match parse_day4(&text) {
        Ok((_,(ns,gs))) => {
            println!("{:?}", ns);
            println!("{:?}", gs);
            Err(())
            //Ok(x)
        }
        Err(_) => Err(())
    }
}
