extern crate colored;

use colored::*;
use std::char;

const MATRIX_SIZE: usize = 10;

#[derive(Clone)]
struct Pos {
    i: usize,
    j: usize,
}

struct Matrix {
    matrix: [[char; MATRIX_SIZE]; MATRIX_SIZE],
    si: usize,
    sj: usize,
    ki: usize,
    kj: usize,
}

fn build_matrix() -> Matrix {
    let mut matrix: [[char; MATRIX_SIZE]; MATRIX_SIZE] = [['0'; MATRIX_SIZE]; MATRIX_SIZE];
    for col in matrix.iter_mut() {
        for val in col.iter_mut() {
            *val = char::from_digit(rand::random::<u32>() % 2, 10).unwrap();
        }
    }
    let si = rand::random::<usize>() % MATRIX_SIZE;
    let sj = rand::random::<usize>() % MATRIX_SIZE;
    let ki = rand::random::<usize>() % MATRIX_SIZE;
    let kj = rand::random::<usize>() % MATRIX_SIZE;
    matrix[si][sj] = 'S';
    matrix[ki][kj] = 'K';
    let ret = Matrix {
        matrix,
        si,
        sj,
        ki,
        kj,
    };
    ret
}

fn print_matrix(matrix: &Matrix) {
    for col in matrix.matrix.iter() {
        for v in col.iter() {
            if *v == 'S' {
                print!("{} ", "S".green().bold());
            } else if *v == 'K' {
                print!("{} ", "K".red().bold());
            } else {
                print!("{} ", v);
            }
        }
        println!();
    }
}

fn print_matrix_color_path(matrix: &Matrix, p: &Vec<Pos>) {
    for (i, col) in matrix.matrix.iter().enumerate() {
        for (j, v) in col.iter().enumerate() {
            if *v == 'S' {
                print!("{} ", "S".green().bold());
            } else if *v == 'K' {
                print!("{} ", "K".red().bold());
            } else {
                let cur = Pos { i, j };
                if p.iter().any(|x| x.i == cur.i && x.j == cur.j) {
                    print!("{} ", v.to_string().yellow());
                } else {
                    print!("{} ", v);
                }
            }
        }
        println!();
    }
}

fn walk_matrix(m: &Matrix, p: &mut Vec<Vec<Pos>>, pid: usize) {
    let si = p[pid].last().unwrap().i;
    let sj = p[pid].last().unwrap().j;
    let i = si as i32;
    let j = sj as i32;
    // start from S point
    if i + 1 < MATRIX_SIZE as i32 && (m.matrix[si + 1][sj] == '1' || m.matrix[si + 1][sj] == 'K') {
        //println!("can go down");
        continue_walk(m, p, pid, si + 1, sj);
    }
    if i - 1 >= 0 && (m.matrix[si - 1][sj] == '1' || m.matrix[si - 1][sj] == 'K') {
        //println!("can go up");
        continue_walk(m, p, pid, si - 1, sj);
    }
    if j + 1 < MATRIX_SIZE as i32 && (m.matrix[si][sj + 1] == '1' || m.matrix[si][sj + 1] == 'K') {
        //println!("can go right");
        continue_walk(m, p, pid, si, sj + 1);
    }
    if j - 1 >= 0 && (m.matrix[si][sj - 1] == '1' || m.matrix[si][sj - 1] == 'K') {
        //println!("can go left");
        continue_walk(m, p, pid, si, sj - 1);
    }
}

fn continue_walk(m: &Matrix, p: &mut Vec<Vec<Pos>>, pid: usize, i: usize, j: usize) {
    if !unique_in_path(p, pid, i, j) {
        return;
    }

    let mut new_path = p[pid].clone();
    new_path.push(Pos { i, j });
    p.push(new_path);

    walk_matrix(m, p, p.len() - 1);
}

fn unique_in_path(p: &Vec<Vec<Pos>>, pid: usize, i: usize, j: usize) -> bool {
    for pos in p[pid].iter() {
        if pos.i == i && pos.j == j {
            return false;
        }
    }
    return true;
}

#[test]
fn unique_in_path_test() {
    let paths = build_paths(1, 2);
    assert_eq!(unique_in_path(&paths, 0, 1, 2), false);
    assert_eq!(unique_in_path(&paths, 0, 2, 2), true);
}

fn build_paths(si: usize, sj: usize) -> Vec<Vec<Pos>> {
    let mut path_root = Vec::new();
    let i = si;
    let j = sj;
    path_root.push(Pos { i, j });
    let mut paths = Vec::new();
    paths.push(path_root);
    paths
}

fn print_path(p: &Vec<Pos>) {
    for node in p.iter() {
        print!("({},{})", node.i, node.j);
    }
    println!();
}

#[allow(dead_code)]
fn print_paths(p: &Vec<Vec<Pos>>) {
    for path in p.iter() {
        print_path(path);
    }
}

fn find_shortest(m: &Matrix, p: &Vec<Vec<Pos>>) {
    let mut shortest_path: &Vec<Pos> = &Vec::new();
    let mut shortest_len = std::usize::MAX;

    for path in p.iter() {
        if path.len() < shortest_len
            && path.last().unwrap().i == m.ki
            && path.last().unwrap().j == m.kj
        {
            shortest_path = path;
            shortest_len = path.len();
        }
    }

    if shortest_len == std::usize::MAX {
        println!("no path to target found");
    } else {
        println!("shortest path to target has {} nodes", shortest_len);
        print_path(shortest_path);
        print_matrix_color_path(m, shortest_path);
    }
}

fn main() {
    let matrix = build_matrix();
    print_matrix(&matrix);
    let mut paths = build_paths(matrix.si, matrix.sj);

    walk_matrix(&matrix, &mut paths, 0);

    //print_paths(&paths);
    println!("Found {} paths", paths.len());

    find_shortest(&matrix, &paths);
}
