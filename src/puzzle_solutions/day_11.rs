use crate::Problem;

pub struct DayEleven;

impl Problem for DayEleven {
    fn part_one(&self, _input: &str) -> String {
        "This is just a template part one".to_string()
    }
    fn part_two(&self, _input: &str) -> String {
        "This is just a template part two".to_string()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Universe {
    map: Vec<Vec<char>>,
    empty_rows: Vec<usize>,
    empty_cols: Vec<usize>,
}

impl Universe {
    fn new(input: &str) -> Self {
        let i = input.lines().map(|x| x.trim()).collect::<Vec<_>>();
        let mut m = Vec::new();
        for line in i {
            m.push(line.chars().collect::<Vec<_>>())
        }
        let e_r: Vec<usize> = Vec::new();
        let e_c: Vec<usize> = Vec::new();
        Universe {
            map: m,
            empty_rows: e_r,
            empty_cols: e_c,
        }
    }

    fn find_empty_rows(&mut self) {
        for (i, line) in self.map.iter().enumerate() {
            let mut is_empty = true;
            for c in line {
                if c == &'#' {
                    is_empty = false;
                }
            }
            if is_empty {
                self.empty_rows.push(i);
            }
        }
    }

    fn find_empty_cols(&mut self) {
        for i in 0..self.map[0].len() {
            let mut is_empty = true;
            for l in &self.map {
                if l[i] == '#' {
                    is_empty = false;
                }
            }
            if is_empty {
                self.empty_cols.push(i);
            }
        }
    }

    fn expand(&mut self) {
        let l = self.map[0].len();
        let mut offset = 0;

        for e_r in &self.empty_rows {
            self.map.insert(*e_r + offset, vec!['.'; l]);
            offset += 1;
        }

        offset = 0;
        for e_c in &self.empty_cols {
            for l in 0..self.map.len() {
                self.map[l].insert(*e_c + offset, '.')
            }
            offset += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_universe() {
        let input = "...#......
        .........#";

        let expected = Universe {
            map: vec![
                vec!['.', '.', '.', '#', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
            ],
            empty_rows: vec![],
            empty_cols: vec![],
        };

        assert_eq!(expected, Universe::new(input));
    }

    #[test]
    fn test_find_empty_space() {
        let mut u = Universe::new(
            "...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....",
        );
        u.find_empty_rows();
        assert_eq!(vec![3, 7], u.empty_rows);
        u.find_empty_cols();
        assert_eq!(vec![2, 5, 8], u.empty_cols);
    }

    #[test]
    fn test_expand_universe() {
        let mut u = Universe {
            map: vec![vec!['#', '#'], vec!['#', '#']],
            empty_rows: vec![0, 1],
            empty_cols: vec![1, 2],
        };

        let expected = vec![
            vec!['.', '.', '.', '.'],
            vec!['#', '.', '#', '.'],
            vec!['.', '.', '.', '.'],
            vec!['#', '.', '#', '.'],
        ];

        u.expand();

        assert_eq!(expected, u.map)
    }
    #[test]
    fn test_expand_integration() {
        let mut u = Universe::new(
            "...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....",
        );

        u.find_empty_cols();
        u.find_empty_rows();
        u.expand();

        let expected = vec![
            vec![
                '.', '.', '.', '.', '#', '.', '.', '.', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '#', '.', '.', '.',
            ],
            vec![
                '#', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '.', '.', '.', '#', '.', '.', '.', '.',
            ],
            vec![
                '.', '#', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '#',
            ],
            vec![
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            ],
            vec![
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '#', '.', '.', '.',
            ],
            vec![
                '#', '.', '.', '.', '.', '#', '.', '.', '.', '.', '.', '.', '.',
            ],
        ];

        assert_eq!(expected, u.map);
    }
    #[test]
    fn test_part_one() {
        let input = "This is just some input part one";

        let day = DayEleven {};
        assert_eq!(day.part_one(input), "This is just a template part one");
    }
    #[test]
    fn test_part_two() {
        let input = "This is just some input part two";

        let day = DayEleven {};
        assert_eq!(day.part_two(input), "This is just a template part two");
    }
}
