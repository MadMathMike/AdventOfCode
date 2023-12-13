mod tile_parser;

use std::time::Instant;

use crate::tile_parser::*;
use crate::tile_parser::TileType::*;

fn main() {
    let start = Instant::now();

    let input = include_str!("../part1.txt");
    let map = convert_to_tile_map(&parse_input(input));

    let length = find_pipe_length(&map);
    
    dbg!(length);

    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}

fn find_start(map: &Vec<Vec<Tile>>) -> Tile{
    for x in 0..map.len() {
        for y in 0..map[0].len() {
            if map[x][y].t == Start {
                return map[x][y]
            }
        }
    }

    panic!("Start not found on the map!");
}

fn find_next(map: &Vec<Vec<Tile>>, current: Tile, previous: Option<Tile>) -> Tile {
    let y_upper_bound = map[0].len() - 1;
    let x_upper_bound = map.len() - 1;

    let north_candidate = 
        if current.y < y_upper_bound && current.t.has_north_connector() { 
            let x = current.x;
            let y = current.y + 1;

            if previous == None || previous.unwrap().x != x || previous.unwrap().y != y {
                let tile = map[x][y];
                if tile.t.has_south_connector(){
                    Some(tile)
                } else {
                    None
                }
            } else {
                None
            }
        } else { 
            None 
        };
    let east_candidate = 
        if current.x < x_upper_bound && current.t.has_east_connector() { 
            let x = current.x + 1;
            let y = current.y;

            if previous == None || previous.unwrap().x != x || previous.unwrap().y != y {
                let tile = map[x][y];
                if tile.t.has_west_connector(){
                    Some(tile)
                } else {
                    None
                }
            } else {
                None
            } 
        } 
        else { 
            None 
        };
    let south_candidate = 
        if current.y > 0 && current.t.has_south_connector() { 
            let x = current.x;
            let y = current.y - 1;

            if previous == None || previous.unwrap().x != x || previous.unwrap().y != y {
                let tile = map[x][y];
                if tile.t.has_north_connector(){
                    Some(tile)
                } else {
                    None
                }
            } else {
                None
            }
        }
        else { 
            None 
        };
    let west_candidate = 
        if current.x > 0 && current.t.has_west_connector(){
            let x = current.x - 1;
            let y = current.y;

            if previous == None || previous.unwrap().x != x || previous.unwrap().y != y {
                let tile = map[x][y];
                if tile.t.has_east_connector(){
                    Some(tile)
                } else {
                    None
                }
            } else {
                None
            }
        }
        else { None };

    let candidates = [north_candidate, east_candidate, south_candidate, west_candidate];
    
    let candidates = candidates.iter()
         .filter(|c| !None.eq(c))
         .map(|c| c.unwrap())
         .collect::<Vec<Tile>>();

    assert!(candidates.len() <= 2);

    candidates[0]
}

fn find_pipe_length(map: &Vec<Vec<Tile>>) -> u32 {
    let start = find_start(&map);
    let mut length = 1;

    let mut previous: Option<Tile> = None;
    let mut current = start;
    
    loop {
        let next = find_next(&map, current, previous);
        
        if next == start {
            break;
        }

        length += 1;
        previous = Some(current);
        current = next;
    }

    length
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Tile {
    pub x: usize,
    pub y: usize,
    pub t: TileType
}

fn convert_to_tile_map(map: &Vec<Vec<TileType>>) -> Vec<Vec<Tile>> {
    let mut tile_map = Vec::<Vec::<Tile>>::new();

    for x in 0..map.len() {
        let mut column = Vec::<Tile>::new();

        for y in 0..map[x].len() {
            column.push(Tile{x: x, y: y, t: map[x][y]});
        }

        tile_map.push(column);
    }

    tile_map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_start_position() {
        let sample_input = include_str!("../sample_input.txt");
        let map = convert_to_tile_map(&parse_input(sample_input));

        let start = find_start(&map);

        assert_eq!(start, Tile{x: 0, y: 2, t: Start });
    }

    // TODO: probs add more tests for this...
    #[test]
    fn find_next_from_start_returns_order_of_N_E_S_W() {
        let sample_input = include_str!("../sample_input.txt");
        let map = convert_to_tile_map(&parse_input(sample_input));

        let start = find_start(&map);

        let next = find_next(&map, start, None);

        assert_eq!(next, Tile{x: 1, y: 2, t: N_W });
    }

    #[test]
    fn find_next_gets_next() {
        let sample_input = include_str!("../sample_input.txt");
        let map = convert_to_tile_map(&parse_input(sample_input));

        let start = find_start(&map);

        let current = find_next(&map, start, None);

        let next = find_next(&map, current, Some(start));

        assert_eq!(next, Tile{x: 1, y: 3, t: E_S});
    }

    #[test]
    fn find_length_returns_sixteen() {
        let sample_input = include_str!("../sample_input.txt");
        let map = convert_to_tile_map(&parse_input(sample_input));

        let length = find_pipe_length(&map);

        assert_eq!(length, 16);
    }
}