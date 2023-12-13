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
    assert_eq!(length, 13402);

    let enclosed_tile_count = count_points_enclosed_by_pipe(&map);
    dbg!(enclosed_tile_count);

    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);

    assert_ne!(enclosed_tile_count, 314);
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

fn find_pipe_length(map: &Vec<Vec<Tile>>) -> usize {
    build_pipe(map).len()
}

fn build_pipe(map: &Vec<Vec<Tile>>) -> Vec<Tile> {
    let mut pipe = Vec::<Tile>::new();
    let start = find_start(&map);

    pipe.push(start);

    let mut previous: Option<Tile> = None;
    let mut current = start;
    
    loop {
        let next = find_next(&map, current, previous);
        
        if next == start {
            break;
        }

        pipe.push(next);

        previous = Some(current);
        current = next;
    }

    pipe
}

fn count_points_enclosed_by_pipe(map: &Vec<Vec<Tile>>) -> i32 {
    let pipe = build_pipe(map);
    let tiles_not_part_of_pipe = map.iter()
        .flatten()
        .map(|tile| *tile)
        .filter(|tile| !pipe.contains(tile))
        .collect::<Vec<Tile>>();
    
    let pipe_segment_locations = collapse_horizontal_pipe_segments(&pipe)
        .iter()
        .map(|pipe_segment|(pipe_segment.x, pipe_segment.y))
        .collect::<Vec<(usize, usize)>>();

    let mut tiles_enclosed_by_loop = 0;

    for tile in tiles_not_part_of_pipe.iter() {
        let mut crossings = 0;
        
        for x in 0..tile.x {
            if pipe_segment_locations.contains(&(x, tile.y)) {
                crossings += 1;
            }
        }
        
        let enclosed_by_loop = crossings % 2 == 1;

        if enclosed_by_loop {
            tiles_enclosed_by_loop += 1;
        }
    }

    tiles_enclosed_by_loop
}

fn collapse_horizontal_pipe_segments(pipe: &Vec<Tile>) -> Vec<Tile> {
    let mut collapsed_pipe = Vec::<Tile>::new();

    // this guarantees that I start somewhere not in a horizontal section
    let index_of_first_vertical_pipe_section = pipe.iter()
        .position(|tile| tile.t == TileType::N_S)
        .unwrap();

    let mut in_horizontal_section = false;
    let mut horizontal_section_start: Option<Tile> = None;

    for i in index_of_first_vertical_pipe_section..(pipe.len() + index_of_first_vertical_pipe_section) {
        let index = i % pipe.len();
        let tile = pipe[index];

        if tile.t == TileType::E_W {
            continue;
        }

        if tile.t == TileType::N_S {
            collapsed_pipe.push(tile);
            continue;
        }

        if in_horizontal_section {
            if !((tile.t.has_north_connector() && horizontal_section_start.unwrap().t.has_north_connector())
                || (tile.t.has_south_connector() && horizontal_section_start.unwrap().t.has_south_connector())) {
                collapsed_pipe.push(Tile {
                    x: horizontal_section_start.unwrap().x,
                    y: horizontal_section_start.unwrap().y,
                    t: TileType::N_S
                });            
            }

            in_horizontal_section = false;
            horizontal_section_start = None;

        } else {
            in_horizontal_section = true;
            horizontal_section_start = Some(tile);
        }
    }

    if horizontal_section_start != None {
        if horizontal_section_start.unwrap().t == TileType::Start {
            collapsed_pipe.push(horizontal_section_start.unwrap());
        } else {
            panic!("Should have used all horizontal sections!")
        }
    }

    collapsed_pipe
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

    #[test]
    fn count_points_enclosed_by_pipe_works_test_1() {
        let sample_input = 
"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
";
        let map = convert_to_tile_map(&parse_input(sample_input));

        let points_enclosed = count_points_enclosed_by_pipe(&map);

        assert_eq!(points_enclosed, 4);
    }

    #[test]
    fn count_points_enclosed_by_pipe_works_test_2() {
        let sample_input = 
".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
";
        let map = convert_to_tile_map(&parse_input(sample_input));

        let points_enclosed = count_points_enclosed_by_pipe(&map);

        assert_eq!(points_enclosed, 8);
    }

    #[test]
    fn count_points_enclosed_by_pipe_works_test_3() {
        let sample_input = 
"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";
        let map = convert_to_tile_map(&parse_input(sample_input));

        let points_enclosed = count_points_enclosed_by_pipe(&map);

        assert_eq!(points_enclosed, 10);
    }

    #[test]
    fn build_pipe_sample_1() {
        let sample_input = 
"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
";
        let map = convert_to_tile_map(&parse_input(sample_input));

        let pipe = build_pipe(&map);
        assert_eq!(pipe.len(), 46);

        assert_eq!(pipe[0], Tile{x: 1, y: 7, t: Start});
        assert_eq!(pipe[1], Tile{x: 2, y: 7, t: E_W});
    }

    #[test]
    fn build_pipe_sample_2() {
        let sample_input = 
".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
";
        let map = convert_to_tile_map(&parse_input(sample_input));

        let pipe = build_pipe(&map);

        assert_eq!(pipe[0], Tile{x: 12, y: 5, t: Start});
        assert_eq!(pipe[1], Tile{x: 13, y: 5, t: S_W});
    }

    #[test]
    fn build_pipe_sample_3() {
        let sample_input = 
"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";
        let map = convert_to_tile_map(&parse_input(sample_input));

        let pipe = build_pipe(&map);

        assert_eq!(pipe[0], Tile{x: 4, y: 9, t: Start});
        assert_eq!(pipe[1], Tile{x: 4, y: 8, t: N_S});
    }

    #[test]
    fn build_pipe_input() {
        let input = include_str!("../part1.txt");
        let map = convert_to_tile_map(&parse_input(input));

        let pipe = build_pipe(&map);

        assert_eq!(pipe[0], Tile{x: 108, y: 114, t: Start});
        assert_eq!(pipe[1], Tile{x: 108, y: 115, t: N_S});
    }

    #[test]
    fn collapse_pipe_sample1() {
        let sample_input = 
"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

        let map = convert_to_tile_map(&parse_input(sample_input));

        let pipe = build_pipe(&map);

        let collapsed_pipe = collapse_horizontal_pipe_segments(&pipe);
        assert_eq!(collapsed_pipe.len(), 18);
        assert_eq!(collapsed_pipe[0], Tile{x: 9, y: 6, t: TileType::N_S});
        assert_eq!(collapsed_pipe[5], Tile{x: 6, y: 2, t: TileType::N_S});
        assert_eq!(collapsed_pipe[6], Tile{x: 6, y: 3, t: TileType::N_S});
        assert_eq!(collapsed_pipe[7], Tile{x: 8, y: 4, t: TileType::N_S});

        assert_eq!(collapsed_pipe[17], Tile{x: 1, y: 6, t: TileType::N_S});
    }
}