use super::functional_tiles::UtilityTile;

pub fn flip_horz(input: Vec<Vec<Option<UtilityTile>>>) -> Vec<Vec<Option<UtilityTile>>> {
    let width = input.len();
    let mut output: Vec<Vec<Option<UtilityTile>>> = vec![vec![]; width];

    for x in 0..width {
        let height = input[x].len();
        let invert_x = width - 1 - x;
        for y in 0..height {
            output[x].push(input[invert_x][y]);
        }
    }
    output
}
