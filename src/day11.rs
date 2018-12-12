day!(
    day11,
    "https://adventofcode.com/2018/day/11/input",
    part1,
    part2
);

use crate::vec2::{Vec2i, Vec2us};
use crate::mat2::Mat2;

const SIZE: usize = 300;

fn make_grid(serial_number: i32) -> Mat2<i8> {
    let mut res = Mat2::new(0, Vec2us::from(SIZE));

    for x in 1..=SIZE {
        for y in 1..=SIZE {
            let rack_id: i32 = 10 + (x as i32);
            let mut value = rack_id * (y as i32);
            value += serial_number;
            value *= rack_id;
            value = (value / 100) % 10;
            value -= 5;

            res[x - 1][y - 1] = value as i8;
        }
    }

    res
}

fn part1(input: &str) -> Result<Vec2i> {
    let serial_number: i32 = input.parse()?;
    let grid = make_grid(serial_number);

    let mut sums = Vec::with_capacity((SIZE - 2) * (SIZE - 2));

    for x in 1..=SIZE - 2 {
        for y in 1..=SIZE - 2 {
            let mut sum = 0;
            for ox in 0..3 {
                for oy in 0..3 {
                    sum += grid[x + ox - 1][y + oy - 1];
                }
            }
            sums.push((Vec2i::new(x as i32, y as i32), sum));
        }
    }

    sums.into_iter()
        .max_by_key(|(_, sum)| *sum)
        .map(|(p, _)| p)
        .ok_or(Error::Input("no maximum value"))
}

fn part2(input: &str) -> Result<String> {
    let serial_number: i32 = input.parse()?;
    let grid = make_grid(serial_number);
    let mut integral_grid = Mat2::new(0i32, Vec2us::from(SIZE));

    // Transform grid into an integral grid, where each cell
    // is the sum all numbers in the square from the top-left
    // to the current cell.
    integral_grid[0][0] = grid[0][0] as i32;
    for x in 1..SIZE {
        integral_grid[x][0] = grid[x][0] as i32 + integral_grid[x - 1][0];
    }
    for y in 1..SIZE {
        integral_grid[0][y] = grid[0][y] as i32 + integral_grid[0][y - 1];
    }
    for x in 1..SIZE {
        for y in 1..SIZE {
            integral_grid[x][y] = integral_grid[x - 1][y] + integral_grid[x][y - 1]
                - integral_grid[x - 1][y - 1]
                + grid[x][y] as i32;
        }
    }

    (1..=SIZE)
        .flat_map(|size| {
            (0..=SIZE - size)
                .map(move |x| (size, x))
                .flat_map(|(size, x)| (0..=SIZE - size).map(move |y| (size, x, y)))
        })
        .map(|(size, xmin, ymin)| {
            let xmax = xmin + size - 1;
            let ymax = ymin + size - 1;
            let sum = if xmin == 0 {
                if ymin == 0 {
                    integral_grid[0][0]
                } else {
                    integral_grid[xmax][ymax] - integral_grid[xmax][ymin - 1]
                }
            } else {
                if ymin == 0 {
                    integral_grid[xmax][ymax] - integral_grid[xmin - 1][ymax]
                } else {
                    integral_grid[xmax][ymax]
                        - integral_grid[xmin - 1][ymax]
                        - integral_grid[xmax][ymin - 1]
                        + integral_grid[xmin - 1][ymin - 1]
                }
            };
            (size, xmin, ymin, sum)
        })
        .max_by_key(|(_, _, _, sum)| *sum)
        .map(|(size, x, y, _)| format!("{},{},{}", x + 1, y + 1, size))
        .ok_or(Error::Input("unsolveable"))
}

#[test]
fn day11_test() {
    assert_eq!(make_grid(57)[122 - 1][79 - 1], -5);
    assert_eq!(make_grid(39)[217 - 1][196 - 1], 0);
    assert_eq!(make_grid(71)[101 - 1][153 - 1], 4);

    assert_results!(part1,
        "18" => Vec2i::new(33, 45),
        "42" => Vec2i::new(21, 61),
    );

    assert_results!(part2,
        "18" => "90,269,16",
        "42" => "232,251,12",
    );
}
