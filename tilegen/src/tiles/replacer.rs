use rand::prelude::*;

use super::context::TileContext;
use super::grid::TileGrid;
use super::replacement_rule::ReplacementRule;

#[allow(unused)]
/// Given a source grid, and a set of constraints, update the destination grid
/// Note that source and destination grid MUST have the same size for this to work
pub fn replace_tiles<T: PartialEq + Eq + Clone>(
    grid: &mut TileGrid<T>,
    layer: usize,
    rules: Vec<ReplacementRule<T>>,
    rng: &mut impl Rng,
) {
    // create a copy so that comparisions aren't using the modified version
    let source = grid.clone();
    let width = grid.width();
    let height = grid.height();

    let mut apply_counts: Vec<u16> = vec![0; rules.len()];

    for x in 0..width {
        for y in 0..height {
            for (i, rule) in rules.iter().enumerate() {
                // if the rule has applied the max number of times, skip it
                if rule
                    .apply_count
                    .map(|c| c <= apply_counts[i])
                    .unwrap_or(false)
                {
                    continue;
                }

                let src = TileContext {
                    x: x as i32,
                    y: y as i32,
                    z: layer as i32,
                    grid: &grid,
                    tile: &source[x][y][layer],
                };
                let dest = TileContext {
                    x: x as i32,
                    y: y as i32,
                    z: layer as i32,
                    grid: &source,
                    tile: &source[x][y][layer],
                };

                // check conditions
                if (rule.condition)(&src, &dest) && rng.random_bool(rule.chance) {
                    // apply replacements
                    for replacement in rule.replacements.clone() {
                        let rx = x as i32 + replacement.delta_x;
                        let ry = y as i32 + replacement.delta_y;
                        let rz = layer as i32 + replacement.delta_z;

                        if rx >= 0
                            && ry >= 0
                            && rz >= 0
                            && (rx as usize) < grid.len()
                            && grid.len() > 0
                            && (ry as usize) < grid[0].len()
                            && grid[0].len() > 0
                            && (rz as usize) < grid.len()
                        {
                            grid.tiles[rx as usize][ry as usize][rz as usize] =
                                replacement.replacement;
                        }
                    }

                    // increment how many times this has been applied
                    apply_counts[i] += 1;

                    // skip remaining constraints since this one matched
                    break;
                }
            }
        }
    }
}

#[allow(unused)]
pub(crate) fn is_edge(x: usize, y: usize, width: usize, height: usize) -> bool {
    x == 0 || y == 0 || x == (width - 1) || y == (height - 1)
}
