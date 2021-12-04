
// https://docs.rs/nom/latest/nom/
/*
fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
  u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
  c.is_digit(16)
}

fn hex_primary(input: &str) -> IResult<&str, u8> {
  map_res(
    take_while_m_n(2, 2, is_hex_digit),
    from_hex
  )(input)
}
*/

// function to parse a number (not in nom???)
pub fn parse_number(i: &str) -> nom::IResult<&str, u64> {
    let (i,s) = nom::bytes::complete::take_while1(nom::AsChar::is_dec_digit)(i)?;
    let n = u64::from_str_radix(s, 10).unwrap();
    Ok((i, n))
}

pub fn parse_commasep_numbers(i: &str) -> nom::IResult<&str, Vec<u64>> {
  nom::multi::separated_list1(nom::bytes::complete::tag(","), parse_number)(i)
}

pub fn parse_whitespacesep_numbers(i: &str) -> nom::IResult<&str, Vec<u64>> {
  let (i,_) = nom::character::complete::space0(i)?;
  nom::multi::separated_list1(nom::character::complete::space1, parse_number)(i)
}

pub fn parse_grid_numbers(i: &str) -> nom::IResult<&str, Vec<Vec<u64>>> {
  nom::multi::separated_list1(nom::character::complete::newline, parse_whitespacesep_numbers)(i)
}
