use super::functional_tiles::UtilityTile;

pub struct TileGrid;

impl TileGrid {
    pub fn get_width<T>(grid: &Vec<Vec<Vec<Option<T>>>>) -> usize {
        grid.len()
    }

    pub fn get_height<T>(grid: &Vec<Vec<Vec<Option<T>>>>) -> usize {
        if Self::get_width(grid) > 0 {
            grid[0].len()
        } else {
            0
        }
    }

    pub fn get_depth<T>(grid: &Vec<Vec<Vec<Option<T>>>>) -> usize {
        if Self::get_height(grid) > 0 {
            grid[0][0].len()
        } else {
            0
        }
    }

    /// Wrap grid in padding
    pub fn pad<T: Clone>(
        input: &Vec<Vec<Vec<Option<T>>>>,
        top: u8,
        right: u8,
        bottom: u8,
        left: u8,
    ) -> Vec<Vec<Vec<Option<T>>>> {
        let width = Self::get_width(input) + left as usize + right as usize;
        let height = Self::get_height(input) + top as usize + bottom as usize;
        let depth = Self::get_depth(input);
        let mut output = vec![vec![vec![None; depth]; height]; width];

        for x in 0..width {
            if x >= left as usize && x < (width - right as usize) {
                let input_x = x - left as usize;
                for y in 0..height {
                    for z in 0..depth {
                        if y >= top as usize && y < (height - bottom as usize) {
                            let input_y = y - top as usize;
                            output[x][y][z] = input[input_x][input_y][z].clone();
                        }
                    }
                }
            }
        }

        output
    }

    pub fn add_layer(grid: &mut Vec<Vec<Vec<Option<UtilityTile>>>>) {
        let height = Self::get_height(grid);
        for x in 0..Self::get_width(grid) {
            for y in 0..height {
                let depth = grid[x][y].len();
                for _ in 0..depth {
                    grid[x][y].push(None);
                }
            }
        }
    }
}
