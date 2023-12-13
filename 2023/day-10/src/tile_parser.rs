#[derive(Debug, Copy, Clone, PartialEq, Eq)] //PartialOrd, Ord
pub enum TileType {
    N_E,
    N_S,
    N_W,
    E_S,
    E_W,
    S_W,
    Start,
    Ground
}

impl TileType {
    pub fn has_north_connector(self: &Self) -> bool {
        match self {
            TileType::Start | TileType::N_E | TileType::N_S | TileType::N_W => true,
            _ => false
        }
    }

    pub fn has_east_connector(self: &Self) -> bool {
        match self {
            TileType::Start | TileType::N_E | TileType::E_S | TileType::E_W => true,
            _ => false
        }
    }

    pub fn has_south_connector(self: &Self) -> bool {
        match self {
            TileType::Start | TileType::N_S | TileType::E_S | TileType::S_W => true,
            _ => false
        }
    }

    pub fn has_west_connector(self: &Self) -> bool {
        match self {
            TileType::Start | TileType::N_W | TileType::E_W | TileType::S_W => true,
            _ => false
        }
    }
}

/// Rows are read from bottom to top so that the grid can be 
/// treated like the first quadrant of the cartesian plane.
pub fn parse_input(input: &str) -> Vec<Vec<TileType>> {
    invert_x_and_y_indices(
        input.lines()
            .rev()
            .map(|l| l.chars().map(char_to_tile).collect())
            .collect())
}

fn invert_x_and_y_indices(grid: Vec<Vec<TileType>>) -> Vec<Vec<TileType>>{
    let mut corrected_grid = Vec::<Vec::<TileType>>::new();

    for i in 0..grid[0].len() {
        let column = grid.iter().map(|row| row[i]).collect::<Vec<TileType>>();
        corrected_grid.push(column);
    }

    corrected_grid
}

fn char_to_tile(c: char) -> TileType {
    match c {
        'S' => TileType::Start,
        '|' => TileType::N_S,
        'L' => TileType::N_E,
        'J' => TileType::N_W,
        'F' => TileType::E_S,
        '-' => TileType::E_W,
        '7' => TileType::S_W,
        '.' => TileType::Ground,
        _ => panic!("Invalid character found in map: {}", c)
    }
}

#[cfg(test)]
mod tests {
    use assertx::assert_contains_exactly;

    use super::*;
    use super::TileType::*;

    #[test]
    fn parses_sample_input() {
        let sample_input = include_str!("../sample_input.txt");

        let result = parse_input(sample_input);

        // .
        // .
        // S
        // |
        // L
        assert_contains_exactly!(result[0], vec![N_E, N_S, Start, Ground, Ground]);
        // .
        // F
        // J
        // F
        // J
        assert_contains_exactly!(result[1], vec![N_W, E_S, N_W, E_S, Ground]);
        // F
        // J
        // .
        // -
        // .
        assert_contains_exactly!(result[2], vec![Ground, E_W, Ground, N_W, E_S]);
        // 7
        // |
        // L
        // -
        // .
        assert_contains_exactly!(result[3], vec![Ground, E_W, N_E, N_S, S_W]);
        // .
        // .
        // 7
        // J
        // .
        assert_contains_exactly!(result[4], vec![Ground, N_W, S_W, Ground, Ground]);

        // Vector is accessed as (x, y)
        assert_eq!(result[0][2], Start);
    }
}
