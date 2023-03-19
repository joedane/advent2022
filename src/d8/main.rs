use anyhow::{anyhow, Result};
use itertools::Itertools;

fn is_visible(trees: &[[u32; 99]; 99], inv_trees: &[[u32; 99]; 99], i: usize, j: usize) -> bool {
    // from the right
    let my_val = trees[i][j];
    trees[i][(j + 1)..].iter().all(|x| *x < my_val)
        || trees[i][0..j].iter().all(|x| *x < my_val)
        || inv_trees[j][(i + 1)..].iter().all(|x| *x < my_val)
        || inv_trees[j][0..i].iter().all(|x| *x < my_val)
}

fn part1(trees: &[[u32; 99]; 99], inv_trees: &[[u32; 99]; 99]) -> u32 {
    let mut visible_count = 0;

    for i in 1..trees.len() - 1 {
        for j in 1..trees.len() - 1 {
            if is_visible(&trees, &inv_trees, i, j) {
                visible_count += 1;
            }
        }
    }
    visible_count
}

fn score_plane(my_val: u32, plane: &[u32], i: usize) -> (u32, u32) {
    // moving right
    let mut right_score = 0;
    let mut ii = i + 1;
    while ii < plane.len() {
        right_score += 1;
        if plane[ii] >= my_val {
            break;
        }
        ii += 1;
    }
    // moving left
    let mut left_score = 0;
    if i > 0 {
        let mut ii = i - 1;
        while ii > 0 {
            left_score += 1;
            if plane[ii] >= my_val || ii == 0 {
                break;
            }
            ii -= 1;
        }
    }
    (right_score, left_score)
}

fn score<const N: usize>(
    trees: &[[u32; N]; N],
    inv_trees: &[[u32; N]; N],
    i: usize,
    j: usize,
) -> u32 {
    let my_val = trees[i][j];
    let (right, left) = score_plane(my_val, &trees[i], j);
    let (down, up) = score_plane(my_val, &inv_trees[j], i);
    right * left * up * down
}
fn main() -> Result<()> {
    let data = include_str!("input.txt");
    let mut trees = [[0; 99]; 99];
    let mut inv_trees = [[0; 99]; 99];

    for (i, line) in data.lines().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            trees[i][j] = ch.to_digit(10).ok_or(anyhow!("Bad digit:"))?;
            inv_trees[j][i] = trees[i][j];
        }
    }

    println!(
        "{} trees are visible",
        part1(&trees, &inv_trees) + 99 + 99 + 97 + 97
    );

    let max = (0..99)
        .cartesian_product(0..99)
        .map(|(i, j)| score(&trees, &inv_trees, i, j))
        .max()
        .unwrap();

    println!("max score is {max}");
    Ok(())
}

mod test {

    #[test]
    fn test2() {
        let data: [[u32; 5]; 5] = [
            [3, 0, 3, 7, 3],
            [2, 5, 5, 1, 2],
            [6, 5, 3, 3, 2],
            [3, 3, 5, 4, 9],
            [3, 5, 2, 9, 0],
        ];
        let mut inv_data: [[u32; 5]; 5] = [[0; 5]; 5];
        for i in 0..data.len() {
            for j in 0..data[i].len() {
                inv_data[i][j] = data[j][i];
            }
        }
        assert_eq!(crate::score(&data, &inv_data, 1, 2), 4);
    }
}
