use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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


// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


// nom

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
