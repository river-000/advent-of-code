#[cfg(test)]
mod day1;
#[cfg(test)]
mod day2;
#[cfg(test)]
mod day3;

#[cfg(test)]
mod tests {
    use crate::day1;
    use crate::day2;
    use crate::day3;

    #[test]
    fn day1() {
        let filename = "data/day1.txt";
        let data = day1::day1_parse(filename).unwrap();

        let answer = day1::day1_part1_solve(&data);
        println!("{}", answer);
        assert_eq!(answer, 1553);

        let answer = day1::day1_part2_solve(&data);
        println!("{}", answer);
        assert_eq!(answer, 1597);
    }

    #[test]
    fn day2() {
        let filename = "data/day2.txt";
        let data = day2::day2_parse(filename).unwrap();

        let answer = day2::day2_part1_solve(&data);
        println!("{}", answer);
        assert_eq!(answer, 2272262);

        let answer = day2::day2_part2_solve(&data);
        println!("{}", answer);
        assert_eq!(answer, 2134882034);
    }

    #[test]
    fn day3() {
        let filename = "data/day3.txt";
        let data = day3::day3_parse(filename).unwrap();

        let answer = day3::day3_part1_solve(&data);
        println!("{}", answer);
        assert_eq!(answer, 3374136);

        let answer = day3::day3_part2_solve(&data);
        println!("{}", answer);
        assert_eq!(answer, 4432698);
    }
}

mod day4;

fn main() {
    let filename = "data/day4.txt";
    let (numbers, grids) = day4::parse(filename).unwrap();
    let answer = day4::solve(&numbers, &grids);
    println!("{:?}", answer);

    println!("hello world!");
}
