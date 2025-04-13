use super::functional_tiles::UtilityTile;

#[allow(unused)]
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

/// Wrap grid in padding
pub fn padding(
    input: Vec<Vec<Option<UtilityTile>>>,
    top: u8,
    right: u8,
    bottom: u8,
    left: u8,
) -> Vec<Vec<Option<UtilityTile>>> {
    let width = input.len() + left as usize + right as usize;
    let mut output: Vec<Vec<Option<UtilityTile>>> = vec![vec![]; width];

    for x in 0..width {
        let height = input[0].len() + top as usize + bottom as usize;
        if x >= left as usize && x < (width - right as usize) {
            let input_x = x - left as usize;
            for y in 0..height {
                if y >= top as usize && y < (height - bottom as usize) {
                    let input_y = y - top as usize;
                    output[x].push(input[input_x][input_y]);
                } else {
                    output[x].push(None);
                }
            }
        } else {
            for _ in 0..height {
                output[x].push(None);
            }
        }
    }

    output
}
