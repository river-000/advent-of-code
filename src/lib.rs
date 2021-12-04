fn filename(no: usize, example: &str) -> String {
  format!("data/day{}{}.txt", no, example)
}

pub fn implement_day<T>(no: usize, example: &str, parse: fn(&str) -> Result<T, ()>, part1: fn(&T) -> i64, part2: fn(&T) -> i64) {
  let name = filename(no, example);
  let data: T = parse(&name).unwrap();
  let pt1 = part1(&data);
  let pt2 = part2(&data);
  println!("{} {}", pt1, pt2);
}

pub fn implement_test<T>(no: usize, example: &str, parse: fn(&str) -> Result<T, ()>, part: fn(&T) -> i64, expect: i64) {
  let name = filename(no, example);
  let data = parse(&name).unwrap();
  let answer = part(&data);
  assert_eq!(answer, expect);
}

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
pub fn parse_number(i: &str) -> nom::IResult<&str, i64> {
    let (i,s) = nom::bytes::complete::take_while1(nom::AsChar::is_dec_digit)(i)?;
    let n = i64::from_str_radix(s, 10).unwrap();
    Ok((i, n))
}

pub fn parse_commasep_numbers(i: &str) -> nom::IResult<&str, Vec<i64>> {
  nom::multi::separated_list1(nom::bytes::complete::tag(","), parse_number)(i)
}

pub fn parse_whitespacesep_numbers(i: &str) -> nom::IResult<&str, Vec<i64>> {
  let (i,_) = nom::character::complete::space0(i)?;
  nom::multi::separated_list1(nom::character::complete::space1, parse_number)(i)
}

pub fn parse_grid_numbers(i: &str) -> nom::IResult<&str, Vec<Vec<i64>>> {
  nom::multi::separated_list1(nom::character::complete::newline, parse_whitespacesep_numbers)(i)
}
