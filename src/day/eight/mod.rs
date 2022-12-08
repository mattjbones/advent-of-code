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

            // print!("target: {test}, row {row}, col {index}\t");
            // println!(
            //     "rl {}, rr {}, cl {}, cr {}",
            //     visible_in_row_left,
            //     visible_in_row_right,
            //     visible_in_col_left,
            //     visible_in_col_right
            // );

            // if row == 3 && index == 1 {
            //     println!("{:#?}, {:#?}", left_col, right_col.split_first().unwrap().1);
            //     // println!("{:#?}, {:#?}", left_row, right_row.split_first().unwrap().1);

            //     panic!("ded")
            // }

            let visible_in_row = visible_in_row_left || visible_in_row_right;
            let visible_in_col = visible_in_col_left || visible_in_col_right;
            // let visible_in_row = tree_grid_rows[row]
            //     .iter()
            //     .enumerate()
            //     .all(|(i, tree)| test >= *tree);

            // let visible_in_col = tree_grid_cols[index]
            //     .iter()
            //     .enumerate()
            //     .all(|(i, tree)| test >= *tree);

            // println!("column {:#?}", visible_in_col);
            // println!("visible {:#?}", visible_in_row);
            // println!("visible from outside {}", visible_in_col && visible_in_row);
            vis_grid[row][index] = visible_in_col || visible_in_row;
            // println!(
            //     "test {test} is col: {}, row: {}",
            //     visible_in_col, visible_in_row
            // );
            // panic!(
            //     "cols {:#?}, rows {:#?}",
            //     tree_grid_cols[index], tree_grid_rows[row]
            // );
        }
        row += 1;
    }

    vis_grid
}

fn calcualte_visibility_score(tree_grid_cols: Grid, tree_grid_rows: Grid) -> i32 {
    0
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
        println!("Part 2");
        let tree_grid_rows = generate_tree_rows_from_input_string(include_str!("sample"));
        let tree_grid_cols = generate_tree_cols_from_rows(&tree_grid_rows);
        let scenic_score = calcualte_visibility_score(tree_grid_cols, tree_grid_rows);

        println!("Score: {scenic_score}");
        println!("Expected: 8")
    }
}
