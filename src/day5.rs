use advent_of_code::parse_number;
use advent_of_code::read_lines;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::hash_set::HashSet;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(PartialEq, Debug)]
enum LineType {
    Horizontal,
    Vertical,
    DiagonalDR,
    DiagonalDL,
}

#[derive(Debug)]
pub struct Line {
    from: (i64, i64),
    to: (i64, i64),
    typ: LineType,
}

impl Line {
    fn new(from: (i64, i64), to: (i64, i64)) -> Line {
        match (from.0.cmp(&to.0), from.1.cmp(&to.1)) {
            (Ordering::Less, Ordering::Less) => Line {
                from: from,
                to: to,
                typ: LineType::DiagonalDR,
            },
            (Ordering::Equal, Ordering::Less) => Line {
                from: from,
                to: to,
                typ: LineType::Vertical,
            },
            (Ordering::Greater, Ordering::Less) => Line {
                from: from,
                to: to,
                typ: LineType::DiagonalDL,
            },

            (Ordering::Less, Ordering::Equal) => Line {
                from: from,
                to: to,
                typ: LineType::Horizontal,
            },
            (Ordering::Equal, Ordering::Equal) => Line {
                from: from,
                to: to,
                typ: LineType::Horizontal,
            },
            (Ordering::Greater, Ordering::Equal) => Line {
                from: to,
                to: from,
                typ: LineType::Horizontal,
            },

            (Ordering::Less, Ordering::Greater) => Line {
                from: to,
                to: from,
                typ: LineType::DiagonalDL,
            },
            (Ordering::Equal, Ordering::Greater) => Line {
                from: to,
                to: from,
                typ: LineType::Vertical,
            },
            (Ordering::Greater, Ordering::Greater) => Line {
                from: to,
                to: from,
                typ: LineType::DiagonalDR,
            },
        }
    }
}

fn line_dx_dy(l: &LineType) -> (i64, i64) {
    match l {
        LineType::Horizontal => (1, 0),
        LineType::Vertical => (0, 1),
        LineType::DiagonalDR => (1, 1),
        LineType::DiagonalDL => (-1, 1),
    }
}

fn line_len(l: &Line) -> i64 {
    std::cmp::max(l.to.1 - l.from.1, l.to.0 - l.from.0)
}

fn is_horizontal_or_vertical(l: &Line) -> bool {
    l.typ == LineType::Horizontal || l.typ == LineType::Vertical
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

    Ok((i, Line::new(a, b)))
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

fn intersect_lines(intpoints: &mut HashSet<(i64, i64)>, a: &Line, b: &Line) {
    intersect_lines_helper(intpoints, a, b, false);
}

fn mirror_if(mirror: bool, a: (i64, i64)) -> (i64, i64) {
    if mirror {
        (a.1, a.0)
    } else {
        a
    }
}

fn mirror_line(a: &Line) -> Line {
    Line::new(mirror_if(true, a.from), mirror_if(true, a.to))
}

fn intersect_lines_helper(intpoints: &mut HashSet<(i64, i64)>, a: &Line, b: &Line, mirror: bool) {
    //    intpoints.insert(a.from);
    match (&a.typ, &b.typ) {
        (LineType::Horizontal, LineType::Horizontal) => {
            // aaaaaaaaaaaaa
            //    bbbbbbbbbbbbbbb
            //    L        R
            //
            // aaaaaaaaaaaaa
            //      bbbb
            //      L  R
            //
            //       aaaaaaaa
            //   bbbbbbb
            //       L R
            //
            // aaaaa
            //       bbbbbb
            //     R L
            // ensure y is aligned

            if a.from.1 != b.from.1 {
                return;
            }

            let l = std::cmp::max(a.from.0, b.from.0);
            let r = std::cmp::min(a.to.0, b.to.0);

            for xc in (l..r + 1) {
                intpoints.insert(mirror_if(mirror, (xc, a.from.1)));
            }
        }
        (LineType::Horizontal, LineType::Vertical) => {
            //      b
            //      b
            // aaaaaxaaaaa
            //      b
            //      b
            //      b

            if a.from.0 <= b.from.0 && b.from.0 <= a.to.0 {
                if b.from.1 <= a.from.1 && a.from.1 <= b.to.1 {
                    intpoints.insert(mirror_if(mirror, (b.from.0, a.from.1)));
                }
            }
        }
        (LineType::Horizontal, LineType::DiagonalDR) => {
            //      b
            //       b
            // aaaaaaaxaaa
            //        ^b
            //        | b
            //       xc  b  = b.from.0 + 2
            //                  2 = a.from.1 - b.from.1

            let xc = b.from.0 + a.from.1 - b.from.1;

            if a.from.0 <= xc && xc <= a.to.0 {
                if b.from.1 <= a.from.1 && a.from.1 <= a.to.1 {
                    intpoints.insert(mirror_if(mirror, (xc, a.from.1)));
                }
            }
        }
        (LineType::Horizontal, LineType::DiagonalDL) => {
            //      b
            //     b
            // aaaxaaaaaaa
            //   b

            let xc = b.from.0 - (a.from.1 - b.from.1);

            if a.from.0 <= xc && xc <= a.to.0 {
                if b.from.1 <= a.from.1 && a.from.1 <= a.to.1 {
                    intpoints.insert(mirror_if(mirror, (xc, a.from.1)));
                }
            }
        }

        (LineType::Vertical, LineType::Horizontal) => intersect_lines(intpoints, b, a),
        (LineType::Vertical, LineType::Vertical) => {
            // similar to hori hori

            if a.from.0 != b.from.0 {
                return;
            }

            let l = std::cmp::max(a.from.1, b.from.1);
            let r = std::cmp::min(a.to.1, b.to.1);

            for yc in (l..r + 1) {
                intpoints.insert((a.from.0, yc));
            }
        }
        (LineType::Vertical, LineType::DiagonalDR) => {
            if mirror {
                panic!("mirrored twice!");
            }
            intersect_lines_helper(intpoints, &mirror_line(a), &mirror_line(b), true);
        }
        (LineType::Vertical, LineType::DiagonalDL) => {
            if mirror {
                panic!("mirrored twice!");
            }
            intersect_lines_helper(intpoints, &mirror_line(a), &mirror_line(b), true);
        }

        (LineType::DiagonalDR, LineType::Horizontal) => intersect_lines(intpoints, b, a),
        (LineType::DiagonalDR, LineType::Vertical) => intersect_lines(intpoints, b, a),
        (LineType::DiagonalDR, LineType::DiagonalDR) => {
            // a <- p1
            //  a
            //   b <- p2
            //    b
            //     a

            // How to detect that p1 and p2 are on the same line?
            // check dy = dx

            if a.from.1 - b.from.1 == a.from.0 - b.from.0 {
                // same line, situation is very similar to hori hori

                let lx = std::cmp::max(a.from.0, b.from.0);
                let rx = std::cmp::min(a.to.0, b.to.0);

                let ly = std::cmp::max(a.from.1, b.from.1);
                let ry = std::cmp::min(a.to.1, b.to.1);

                for xc in (lx - lx..rx - lx + 1) {
                    intpoints.insert(mirror_if(mirror, (lx + xc, ly + xc)));
                }
            }
        }
        (LineType::DiagonalDR, LineType::DiagonalDL) => {
            // case 1
            // a   b
            //  a b
            //   x
            //  b a
            //     a
            //
            // a
            //  a
            //   a   b
            //    a b
            //     x
            //    b a
            //       a
            //
            // a
            //  a
            //   a
            //    x
            //   b
            //
            // case 2
            // a  b
            //  ab
            //  ba
            // b  a
            //     a

            // (ax + t, ay + t) = (bx - u, by + u)
            // (ax-bx, ay-by) = (-t-u, -t+u)
            //
            // bx-ax = t+u
            // by-ay = t-u
            //
            // bx-ax + by-ay = 2t
            // bx-ay - (by-ay) = 2u
            //
            // need to check t >= 0 and <= length of line a

            let twot = b.from.0 - a.from.0 + b.from.1 - a.from.1;
            if twot % 2 == 0 {
                let t = twot / 2;

                if t >= 0 && t <= (a.to.0 - a.from.0) {
                    let x = a.from.0 + t;
                    let y = a.from.1 + t;
                    intpoints.insert(mirror_if(mirror, (x, y)));
                }
            }
        }

        (LineType::DiagonalDL, LineType::Horizontal) => intersect_lines(intpoints, b, a),
        (LineType::DiagonalDL, LineType::Vertical) => intersect_lines(intpoints, b, a),
        (LineType::DiagonalDL, LineType::DiagonalDR) => intersect_lines(intpoints, b, a),
        (LineType::DiagonalDL, LineType::DiagonalDL) => {
            //     a <- p1
            //    a
            //   b <- p2
            //  b
            // a

            if a.from.1 - b.from.1 == -(a.from.0 - b.from.0) {
                let ly = std::cmp::max(a.from.1, b.from.1);
                let ry = std::cmp::min(a.to.1, b.to.1);

                let lx = std::cmp::min(a.from.0, b.from.0);
                let rx = std::cmp::max(a.to.0, b.to.0);

                for xc in (ly - ly..ry - ly + 1) {
                    intpoints.insert(mirror_if(mirror, (lx - xc, ly + xc)));
                }
            }
        }
    }
}

fn solve_part1_and_part2(lines: &Vec<Line>) -> (i64, i64) {
    let mut intpoints: HashSet<(i64, i64)> = HashSet::new();

    for (a, b) in lines.into_iter().tuple_combinations() {
        intersect_lines(&mut intpoints, a, b);

        /*
        if intpoints.get(&(2,4)).is_some() {
            println!("DEBUG INFO: {:?} {:?}", a, b)
        }
        */
    }
/*
    
    let mut grid: Vec<Vec<bool>> = vec![vec![false; 9]; 9];
    let mut my_intpoints: HashSet<(i64, i64)> = HashSet::new();

    //let (a, b) = (Line::new((0, 0), (8, 0)), Line::new((3, 1), (5, 1)));
    //let (a, b) = (Line::new((0, 0), (8, 0)), Line::new((3, 0), (5, 0)));
    //let (a, b) = (Line::new((4, 0), (4, 8)), Line::new((4, 3), (4, 5)));
    //let (a, b) = (Line::new((1, 5), (6, 5)), Line::new((4, 2), (4, 7)));
    //let (a, b) = (Line::new((1, 1), (8, 8)), Line::new((3, 3), (6, 6)));
    //let (a, b) = (Line::new((1, 1), (8, 8)), Line::new((4, 3), (7, 6)));
    //let (a, b) = (Line::new((1, 5), (8, 5)), Line::new((1, 1), (8, 8)));
    //let (a, b) = (Line::new((1, 5), (8, 5)), Line::new((8, 1), (1, 8)));
    //let (a, b) = (Line::new((0, 0), (0, 8)), Line::new((0, 3), (0, 5)));
    //let (a, b) = (Line::new((0, 0), (8, 8)), Line::new((5, 3), (3, 5)));
    //let (a, b) = (Line::new((0, 0), (8, 8)), Line::new((5+1, 3), (3+1, 5)));
    //let (a, b) = (Line::new((0, 8), (8, 0)), Line::new((7, 1), (6, 2)));
    //let (a, b) = (Line::new((0, 8), (5, 8)), Line::new((0, 8), (2, 8)));
    let (a, b) = (Line::new((2, 1), (2, 2)), Line::new((1, 4), (3, 4)));

    intersect_lines(&mut my_intpoints, &a, &b);

    {
        let (dx, dy) = line_dx_dy(&a.typ);
        let len = line_len(&a);
        for t in (0..=len) {
            grid[(a.from.0 + dx * t) as usize][(a.from.1 + dy * t) as usize] = true;
        }
    }
    {
        let (dx, dy) = line_dx_dy(&b.typ);
        let len = line_len(&b);
        for t in (0..=len) {
            grid[(b.from.0 + dx * t) as usize][(b.from.1 + dy * t) as usize] = true;
        }
    }

    for j in 0..9 {
        for i in 0..9 {
            if my_intpoints.get(&(i, j)).is_some() {
                print!("x");
            } else {
                if grid[i as usize][j as usize] {
                    print!("o");
                } else {
                    print!(".");
                }
            }
        }
        println!("");
    }
    

    for j in 0..10 {
        for i in 0..10 {
            if intpoints.get(&(i, j)).is_some() {
                print!("x");
            } else {
                print!(".");
            }
        }
        println!("");
    }
    */

    (0, intpoints.len() as i64)
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
    //implement_day_twoforone(NO, "day5.example.txt", parse, solve_part1_and_part2);
    implement_day_twoforone(NO, "", parse, solve_part1_and_part2);
    //implement_day_twoforone(NO, "evil/5-50000-10000000.in", parse, solve_part1_and_part2);
    //implement_day_twoforone(NO, "evil/5-20000-6400-fixed.in", parse, solve_part1_and_part2);
    //implement_day_twoforone(NO, "evil/5-50000-10000.in", parse, solve_part1_and_part2);
    //implement_day_twoforone(NO, "evil/5-50000-10000000.in", parse, solve_part1_and_part2);
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

/*
0123456789

1.1....11.
.111...2..
..2.1.111.
...1.2.2..
.112313211
...1.2....
..1...1...
.1.....1..
1.......1.
222111....


..........
.......2..
..2.......
.....2.2..
...23.32..
.....2....
..........
..........
..........
222.......

..........
.......x..
..x.......
.....x.x..
..xxx.xx..
.....x....
..........
..........
..........
xxx.......


  ....x....x
  .........x
  xxxxx....x
  ....x....x
  ....x....x
  ...xxx...x
  ....x....x
  ....xx..xx
  ....x....x
  ....x....x

    (2, 2)
    (7, 1)
    (5, 3)(7, 3)x
    (3, 4)(4, 4)(6, 4)(7, 4)
    (5, 5)
    (0, 9)(1, 9)(2, 9)


    (5, 9), (7, 5), (7, 9), (8, 9), (6, 4), (9, 9),
    (9, 4), (4, 9), (0, 4), (2, 3), (5, 5), (6, 9),
    (3, 9), (0, 9), (8, 4), (7, 8), (2, 4), (3, 4),
    (2, 1), (5, 4), (7, 4), (2, 0), (4, 4), (2, 9),
    (1, 9), (5, 3), (2, 2)

    To debug, implement a helper that takes 2 lines
    prints them out plus the intersections
    and go through all the cases

    */
