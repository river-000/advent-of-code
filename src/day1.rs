
use advent_of_code::read_lines;

pub fn day1_parse(filename: &str) -> Result<Vec<i64>, ()> {
    let mut v = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                let my_int: i64 = ip.parse().unwrap();
                v.push(my_int);
            }
        }
    }

    return Ok(v);
}

pub fn day1_part1_solve(data: &Vec<i64>) -> i64 {
    let mut counter = 0;
    for (prev_line, line) in data.into_iter().zip(data.into_iter().skip(1)) {
        if line > prev_line {
            counter = counter + 1;
        }
    }

    return counter;
}

struct WindowSum<'a> {
    data: &'a Vec<i64>,
    window_size: usize,
    window_sum: i64,
    index: usize,
    first_element: bool,
}

impl<'a> WindowSum<'a> {
    pub fn new(data: &'a Vec<i64>, window_size: usize) -> Self {
        let mut window_sum = 0;

        for i in 0..window_size {
            window_sum = window_sum + &data[i];
        }

        Self {
            data: data,
            window_size: window_size,
            window_sum: window_sum,
            index: 0,
            first_element: true,
        }
    }
}

impl<'a> Iterator for WindowSum<'a> {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first_element {
            self.first_element = false;
            Some(self.window_sum)
        } else if self.index >= self.data.len() - self.window_size {
            return None;
        } else {
            self.window_sum = self.window_sum - &self.data[self.index]
                + &self.data[self.index + self.window_size];

            self.index = self.index + 1;

            Some(self.window_sum)
        }
    }
}

pub fn day1_part2_solve(data: &Vec<i64>) -> i64 {
    let mut counter = 0;
    for (prev_line, line) in WindowSum::new(data, 3).zip(WindowSum::new(data, 3).skip(1)) {
        if line > prev_line {
            counter = counter + 1;
        }
    }

    return counter;
}

//

use advent_of_code::implement_day;
#[cfg(test)]
use advent_of_code::implement_test;

const NO: usize = 1;

pub fn day() {
    implement_day(NO, "", day1_parse, day1_part1_solve, day1_part2_solve);
}

#[cfg(test)]
mod tests {
    use crate::day1::*;

    #[test]
    pub fn part1() {
        implement_test(NO, "", day1_parse, day1_part1_solve, 1553);
    }

    #[test]
    pub fn part2() {
        implement_test(NO, "", day1_parse, day1_part2_solve, 1597);
    }
}
