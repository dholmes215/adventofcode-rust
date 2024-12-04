//
// Copyright (c) 2024 David Holmes (dholmes at dholmes dot us)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//

use adventofcode_rust::aoc::SolutionResult;
use itertools::Itertools;

pub fn day04(input: &[u8]) -> SolutionResult {
    let lines_e = input
        .split(|c| *c == b'\n' || *c == b'\r')
        .map_into::<Vec<u8>>()
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>();
    let lines_w = reverse_lines(&lines_e);

    let (h, w): (i32, i32) = (lines_e.len() as i32, lines_e.first().unwrap().len() as i32);

    let lines_s = flip_diagonal(&lines_e, w);
    let lines_n = reverse_lines(&lines_s);

    // Diagonals
    let lines_ne = diagonals_ne(&lines_e, h, w);
    let lines_nw = diagonals_ne(&lines_w, h, w);
    let lines_sw = reverse_lines(&lines_ne);
    let lines_se = reverse_lines(&lines_nw);

    let mut a = 0;

    {
        let all_line_vecs = [
            &lines_e, &lines_w, &lines_s, &lines_n, &lines_ne, &lines_nw, &lines_sw, &lines_se,
        ];
        for lines in all_line_vecs.iter() {
            for line in lines.iter() {
                a += line.windows(4).filter(|&s| s == b"XMAS").count();
            }
        }
    }

    // Part 2
    let mut b = 0;

    let w = w as usize;
    let h = h as usize;
    let pattern = vec![b"M.S".to_vec(), b".A.".to_vec(), b"M.S".to_vec()];
    for x in 0..(w - 2) {
        for y in 0..(h - 2) {
            let mut candidate = vec![
                lines_e[y][x..(x + 3)].to_vec(),
                lines_e[y + 1][x..(x + 3)].to_vec(),
                lines_e[y + 2][x..(x + 3)].to_vec(),
            ];
            candidate[0][1] = b'.';
            candidate[1][0] = b'.';
            candidate[2][1] = b'.';
            candidate[1][2] = b'.';
            
            let flipped = flip_diagonal(&candidate, 3);

            // print_lines(&candidate);
            if candidate == pattern { b += 1 }
            if reverse_lines(&candidate) == pattern { b += 1 }
            if flipped == pattern { b += 1 }
            if reverse_lines(&flipped) == pattern { b += 1 }
        }
    }

    SolutionResult::new(a, b)
}

struct Vec2 {
    x: i32,
    y: i32,
}

fn reverse_lines(lines: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut lines_w = lines.clone();
    lines_w.iter_mut().for_each(|l| l.reverse());
    lines_w
}

fn flip_diagonal(lines: &[Vec<u8>], w: i32) -> Vec<Vec<u8>> {
    let lines_s = (0..w)
        .map(|i| lines.iter().map(|l| l[i as usize]).collect::<Vec<u8>>())
        .collect::<Vec<_>>();
    lines_s
}

fn diagonals_ne(lines_e: &Vec<Vec<u8>>, h: i32, w: i32) -> Vec<Vec<u8>> {
    let mut lines_ne: Vec<Vec<u8>> = Vec::new();
    for i in 0..(h * 2 - 1) {
        let mut line: Vec<u8> = Vec::new();
        let mut pos = Vec2 { x: 0, y: i };
        while pos.y >= 0 {
            if pos.y < h && pos.x < w {
                line.push(lines_e[pos.y as usize][pos.x as usize]);
            }

            pos.x += 1;
            pos.y -= 1;
        }
        lines_ne.push(line);
    }
    lines_ne
}

pub fn bytes_to_str(input: &[u8]) -> String {
    String::from_utf8_lossy(input).to_string()
}

#[allow(dead_code)]
fn print_lines(lines: &Vec<Vec<u8>>) {
    for l in lines {
        println!("{:>}", bytes_to_str(l));
    }
    println!();
}
