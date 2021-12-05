use advent_of_code::parse_number;
use advent_of_code::read_lines;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
pub struct Line {
    from: (i64, i64),
    to: (i64, i64),
}

fn is_horizontal_or_vertical(l: &Line) -> bool {
    l.from.0 == l.to.0 || l.from.1 == l.to.1
}

fn parse_point(i: &str) -> nom::IResult<&str, (i64, i64)> {
    let (i, a) = parse_number(i)?;
    let (i, _) = nom::bytes::complete::tag(",")(i)?;
    let (i, b) = parse_number(i)?;

    Ok((i, (a, b)))
}

// parse a vector of bits
fn parse_line(i: &str) -> nom::IResult<&str, Line> {
    let (i, a) = parse_point(i)?;
    let (i, _) = nom::character::complete::space1(i)?;
    let (i, _) = nom::bytes::complete::tag("->")(i)?;
    let (i, _) = nom::character::complete::space1(i)?;
    let (i, b) = parse_point(i)?;

    Ok((i, Line { from: a, to: b }))
}

pub fn parse(filename: &str) -> Result<Vec<Line>, ()> {
    let mut result = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(line) = line {
                match parse_line(&line) {
                    Ok((_, m)) => {
                        result.push(m);
                    }
                    Err(_) => return Err(()),
                }
            }
        }
    }

    Ok(result)
}

struct LineIterator<'a> {
    line: &'a Line,
    first: bool,
    pos: (i64, i64),
}

impl<'a> LineIterator<'a> {
    fn new(line: &'a Line) -> Self {
        LineIterator {
            line: line,
            first: true,
            pos: line.from,
        }
    }
}

impl<'a> Iterator for LineIterator<'a> {
    type Item = (i64, i64);

    fn next(&mut self) -> Option<Self::Item> {
        let mut dx = 0;
        let mut dy = 0;

        if self.first {
            self.first = false;
            return Some(self.pos);
        }

        if (self.pos.0 == self.line.to.0) && (self.pos.1 == self.line.to.1) {
            return None;
        }

        if self.pos.0 < self.line.to.0 {
            dx = 1;
        }

        if self.pos.1 < self.line.to.1 {
            dy = 1;
        }

        if self.pos.0 > self.line.to.0 {
            dx = -1;
        }

        if self.pos.1 > self.line.to.1 {
            dy = -1;
        }

        self.pos = (self.pos.0 + dx, self.pos.1 + dy);
        Some(self.pos)
    }
}

fn map_increment<K>(m: &mut HashMap<K, i64>, k: K) -> ()
where
    K: Hash + Eq,
{
    *m.entry(k).or_default() += 1
    /*
    match m.get(&k) {
        None => m.insert(k, 1),
        Some(x) => m.insert(k, x + 1),
    };
    */
}

fn measure_grid(lines: &Vec<Line>) -> (i64, i64) {
    let mut mx = 0;
    let mut my = 0;

    for line in lines {
        mx = std::cmp::max(mx, line.from.0);
        mx = std::cmp::max(mx, line.to.0);
        my = std::cmp::max(my, line.from.1);
        my = std::cmp::max(my, line.to.1);
    }

    (mx + 1, my + 1)
}

fn solve_part1_and_part2(lines: &Vec<Line>) -> (i64, i64) {
    let (cols, rows) = measure_grid(lines);
    let mut map1 = advent_of_code::zeros((cols * rows) as u32);
    let mut map2 = advent_of_code::zeros((cols * rows) as u32);

    for line in lines {
        for point in LineIterator::new(&line).into_iter() {
            if is_horizontal_or_vertical(&line) {
                map1[(point.0 + cols * point.1) as usize] += 1
            }
            map2[(point.0 + cols * point.1) as usize] += 1
        }
    }

    (map1.iter().filter(|&&v| v >= 2).count() as i64, map2.iter().filter(|&&v| v >= 2).count() as i64)
}

fn solve_part1(lines: &Vec<Line>) -> i64 {
    solve_part1_and_part2(lines).0
}

fn solve_part2(lines: &Vec<Line>) -> i64 {
    solve_part1_and_part2(lines).1
}

//

//use advent_of_code::implement_day;
use advent_of_code::implement_day_twoforone;
#[cfg(test)]
use advent_of_code::implement_test;

const NO: usize = 5;

pub fn day() {
    implement_day_twoforone(NO, "", parse, solve_part1_and_part2);
}

#[cfg(test)]
mod tests {
    use crate::day5::*;

    #[test]
    pub fn part1() {
        implement_test(NO, "", parse, solve_part1, 4421);
    }

    #[test]
    pub fn part2() {
        implement_test(NO, "", parse, solve_part2, 18674);
    }
}
