use std::env;
use std::io;

fn main() {
    if env::args().any(|x| x == "--p1") {
        println!("Part 1:");
        p1();
    } else {
        println!("Part 2:");
        p2();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir { N = 0, E = 1, S = 2, W = 3 }

#[derive(Debug, Clone, Copy)]
struct Pipe {
    ds: [Dir; 2]
}

type Grid<T> = Vec<Vec<T>>;

#[derive(Debug, Clone, Copy)]
enum Tile {
    Start,
    Pipe(Pipe),
    Empty,
}

fn read_grid_and_start_from_stdin() -> (Grid<Tile>, (i32, i32)) {
    let mut start = (0, 0);
    let grid: Grid<Tile> = io::stdin().lines().enumerate().map(|(r, l)| {
        let l = l.unwrap();
        let row: Vec<Tile> = l.chars().enumerate().map(|(c, ch)| {
            match ch {
                '.' => Tile::Empty,
                '|' => Tile::Pipe(Pipe { ds: [Dir::N, Dir::S] }),
                '-' => Tile::Pipe(Pipe { ds: [Dir::W, Dir::E] }),
                'L' => Tile::Pipe(Pipe { ds: [Dir::N, Dir::E] }),
                'J' => Tile::Pipe(Pipe { ds: [Dir::W, Dir::N]}),
                '7' => Tile::Pipe(Pipe { ds: [Dir::S, Dir::W]}),
                'F' => Tile::Pipe(Pipe { ds: [Dir::S, Dir::E]}),
                'S' => {
                    start = (r as i32, c as i32);
                    Tile::Start
                }
                _ => panic!("Unhandled tile char {}.", ch),
            }
        }).collect();
        row
    }).collect();
    (grid, start)
}

fn pipe_has_in_dir(p: Pipe, d: Dir) -> bool {
    p.ds.contains(&dir_rotate(d, 2))
}

fn pipe_out_dir(pipe: Pipe, in_d: Dir) -> Dir {
    let flipped_from_d = dir_rotate(in_d, 2);
    pipe.ds.iter().cloned().filter(|&d| d != flipped_from_d).next().unwrap()
}

fn dir_to_delta(d: Dir) -> (i32, i32) {
    match d {
        Dir::N => (-1,  0),
        Dir::E => ( 0,  1),
        Dir::S => ( 1,  0),
        Dir::W => ( 0, -1),
    }
}

fn pos_shift((r, c): (i32, i32), d: Dir) -> (i32, i32) {
    let (dr, dc) = dir_to_delta(d);
    (r + dr, c + dc)
}

fn dir_rotate(d: Dir, t: i32) -> Dir {
    let int_d = (d as i32 + t) % 4;
    match int_d {
        0 => Dir::N,
        1 => Dir::E,
        2 => Dir::S,
        3 => Dir::W,
        _ => panic!("Unexpected int_d value {}.", int_d)
    }
}

fn grid_get<T: Clone>(grid: &Grid<T>, (r, c): (i32, i32)) -> T {
    grid[r as usize][c as usize].clone()
}

fn grid_set<T>(grid: &mut Grid<T>, (r, c): (i32, i32), v: T) {
    grid[r as usize][c as usize] = v;
}

fn grid_width<T>(grid: &Grid<T>) -> i32 {
    grid[0].len() as i32
}

fn grid_height<T>(grid: &Grid<T>) -> i32 {
    grid.len() as i32
}

fn grid_is_inbound<T>(grid: &Grid<T>, (r, c): (i32, i32)) -> bool {
    let w = grid_width(grid);
    let h = grid_height(grid);
    0 <= r && r < h && 0 <= c && c < w 
}

fn p1() {
    let (grid, start) = read_grid_and_start_from_stdin();
    let mut p = start;
    let mut in_d = Dir::N;
    for d in [Dir::N, Dir::S, Dir::E, Dir::W] {
        let p_next = pos_shift(p, d);
        if !grid_is_inbound(&grid, p_next) {
            continue;
        }
        let pipe = match grid_get(&grid, p_next) {
            Tile::Pipe(p) => p,
            _ => panic!("Expected pipe."),
        };
        if pipe_has_in_dir(pipe, d) {
            in_d = d;
            p = p_next;
            break;
        }
    }
    let mut loop_len = 1;
    while p != start {
        let pipe = match grid_get(&grid, p) {
            Tile::Pipe(p) => p,
            _ => panic!("Expected pipe."),
        };
        let out_d = pipe_out_dir(pipe, in_d);
        p = pos_shift(p, out_d);
        in_d = out_d;
        loop_len += 1;
        for d in [Dir::N, Dir::S, Dir::E, Dir::W] {
            if d == in_d {
                continue;
            }
            if pipe_has_in_dir(pipe, d) {
                break;
            }
        }
    }
    println!("{}", loop_len / 2);
}

fn flood(coloring: &mut Vec<Vec<i8>>, p: (i32, i32), v: i8) {
    if grid_get(coloring, p) != 0 {
        return;
    }
    grid_set(coloring, p, v);
    for d in [Dir::N, Dir::E, Dir::S, Dir::W] {
        let next_p = pos_shift(p, d);
        if grid_is_inbound(coloring, next_p) {
            flood(coloring, next_p, v);
        }
    }
}

fn p2() {
    let (grid, start) = read_grid_and_start_from_stdin();
    let mut p = start;
    let mut path = vec![];
    let mut in_d = Dir::N;
    for d in [Dir::N, Dir::S, Dir::E, Dir::W] {
        let p_next = pos_shift(p, d);
        if !grid_is_inbound(&grid, p_next) {
            continue;
        }
        let pipe = match grid_get(&grid, p_next) {
            Tile::Pipe(p) => p,
            _ => panic!("Expected pipe."),
        };
        if pipe_has_in_dir(pipe, d) {
            p = p_next;
            in_d = d;
            break;
        }
    }

    while p != start {
        let pipe = match grid_get(&grid, p) {
            Tile::Pipe(p) => p,
            _ => panic!("Expected pipe."),
        };
        let out_d = pipe_out_dir(pipe, in_d);
        path.push((p, in_d, out_d));
        p = pos_shift(p, out_d);
        in_d = out_d;
    }
    
    path.push((start, in_d, path[0].1));

    let mut coloring = vec![
        vec![0i8; grid_width(&grid) as usize];
        grid_height(&grid) as usize
    ];
    for &(p, _, _) in path.iter() {
        grid_set(&mut coloring, p, -1);
    }

    for &(p, in_d, out_d) in path.iter() {
        for d in [in_d, out_d] {
            let left = pos_shift(p, dir_rotate(d, 3));
            if grid_is_inbound(&grid, left) && grid_get(&coloring, left) == 0 {
                flood(&mut coloring, left, 1);
            }
            let right = pos_shift(p, dir_rotate(d, 1));
            if grid_is_inbound(&grid, right) && grid_get(&coloring, right) == 0 {
                flood(&mut coloring, right, 2);
            }
        }
    }

    let outer = grid_get(&coloring, (0, 0));
    let count = coloring.iter().flatten().filter(|&&v| v != outer && v != -1).count();
    println!("{}", count);
}