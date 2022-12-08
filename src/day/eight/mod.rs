use std::io::Lines;

type Grid = Vec<Vec<usize>>;
type VizGrid = Vec<Vec<bool>>;

fn generate_tree_rows_from_input_string(input: &str) -> Grid {
    let mut tree_grid_rows: Vec<Vec<usize>> = Vec::new();
    let mut row = 0;

    for line in input.lines() {
        // println!("{line}");
        for char in line.chars() {
            let no: usize = char.to_digit(10).unwrap() as usize;
            if tree_grid_rows.get(row).is_some() {
                tree_grid_rows[row].push(no);
            } else {
                tree_grid_rows.push(vec![no]);
            }
        }
        row += 1;
    }
    tree_grid_rows
}

fn generate_tree_cols_from_rows(rows: &Vec<Vec<usize>>) -> Grid {
    let mut tree_grid_cols: Vec<Vec<usize>> = Vec::new();
    let mut col = 0;
    for row in rows {
        // println!("{row:?}");
        for item in row.iter() {
            if tree_grid_cols.get(col).is_some() {
                tree_grid_cols[col].push(*item);
            } else {
                tree_grid_cols.push(vec![*item]);
            }
            // tree_grid_cols[col][i] = *item;
            col += 1;
        }
        col = 0;
    }
    tree_grid_cols
}

fn calculate_visibility_grid(
    tree_grid_rows: Vec<Vec<usize>>,
    tree_grid_cols: Vec<Vec<usize>>,
) -> VizGrid {
    let width = tree_grid_rows.first().unwrap().len();
    let height = tree_grid_rows.len();
    let mut vis_grid = vec![vec![true; width]; height];

    let mut row = 1;
    while row < vis_grid.len() - 1 {
        let vis_grid_row = &vis_grid[row];
        for index in 1..vis_grid_row.len() - 1 {
            let test = tree_grid_rows[row][index];
            let (left_row, right_row) = tree_grid_rows[row].split_at(index);
            let visible_in_row_left = !left_row.iter().any(|tree| test <= *tree);
            let visible_in_row_right = !right_row
                .split_first()
                .unwrap()
                .1
                .iter()
                .any(|tree| test <= *tree);

            let (left_col, right_col) = tree_grid_cols[index].split_at(row);
            let visible_in_col_left = !left_col.iter().any(|tree| test <= *tree);
            let visible_in_col_right = !right_col
                .split_first()
                .unwrap()
                .1
                .iter()
                .any(|tree| test <= *tree);

            let visible_in_row = visible_in_row_left || visible_in_row_right;
            let visible_in_col = visible_in_col_left || visible_in_col_right;
            vis_grid[row][index] = visible_in_col || visible_in_row;
        }
        row += 1;
    }

    vis_grid
}

fn calculate_visibility_score_for_trees(trees: &[usize], target: usize) -> i32 {
    if trees.len() == 0 {
        0
    } else if trees.len() == 1 {
        1
    } else {
        let mut score = 0;
        for tree in trees {
            score += 1;
            if *tree >= target {
                break;
            }
        }
        score
    }
}

fn calculate_visibility_score(tree_grid_cols: Grid, tree_grid_rows: Grid) -> i32 {
    let width = tree_grid_rows.first().unwrap().len();
    let height = tree_grid_rows.len();
    let mut score_grid = vec![vec![0; width]; height];

    let mut row = 0;
    while row < score_grid.len() {
        let vis_grid_row = &score_grid[row];
        for index in 0..vis_grid_row.len() {
            let test = tree_grid_rows[row][index];

            let (left_row, right_row) = tree_grid_rows[row].split_at(index);
            let mut reversed_left_row = left_row.to_vec();
            reversed_left_row.reverse();
            let left_row_score = calculate_visibility_score_for_trees(&reversed_left_row, test);
            let right_row_score =
                calculate_visibility_score_for_trees(right_row.split_first().unwrap().1, test);

            let (left_col, right_col) = tree_grid_cols[index].split_at(row);
            let mut reversed_left_col = left_col.to_vec();
            reversed_left_col.reverse();
            let left_col_score = calculate_visibility_score_for_trees(&reversed_left_col, test);
            let right_col_score =
                calculate_visibility_score_for_trees(right_col.split_first().unwrap().1, test);

            let score = left_col_score * left_row_score * right_col_score * right_row_score;
            // if row == 1 && index == 2 {
            //     println!("target {test}");
            //     println!(
            //         "left {}, right {}, up {}, down {}",
            //         left_row_score, right_row_score, left_col_score, right_col_score
            //     );
            //     println!("{:?} {:?}", left_row, right_row.split_first().unwrap().1);
            //     println!("{:?} {:?}", left_col, right_col.split_first().unwrap().1);
            //     panic!("lol");
            // }
            score_grid[row][index] = score
        }
        row += 1;
    }

    let mut flat_grid = score_grid.iter().flatten().copied().collect::<Vec<i32>>();
    // println!("flat {flat_grid:?}");
    flat_grid.sort();
    // println!("sorted {flat_grid:?}");
    *flat_grid.last().unwrap()
}

pub fn run() {
    println!("Day 8");

    {
        println!("Part 1");
        let tree_grid_rows = generate_tree_rows_from_input_string(include_str!("input"));
        let tree_grid_cols = generate_tree_cols_from_rows(&tree_grid_rows);
        let vis_grid: VizGrid = calculate_visibility_grid(tree_grid_rows, tree_grid_cols);
        println!(
            "{:#?}",
            vis_grid.iter().flatten().filter(|val| **val).count()
        );
        println!("expected: 1672");
    }

    {
        println!("\nPart 2");
        let tree_grid_rows = generate_tree_rows_from_input_string(include_str!("input"));
        let tree_grid_cols = generate_tree_cols_from_rows(&tree_grid_rows);
        let scenic_score = calculate_visibility_score(tree_grid_cols, tree_grid_rows);

        println!("Score: {scenic_score}");
        println!("Expected: 327180")
    }
}
