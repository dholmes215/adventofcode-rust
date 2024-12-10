//
// Copyright (c) 2024 David Holmes (dholmes at dholmes dot us)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//

use adventofcode_rust::aoc::SolutionResult;
use itertools::Itertools;
use std::iter::once;

pub fn day09(input: &str) -> SolutionResult {
    let mut file_id = 0i64;
    let mut blocks = input
        .as_bytes()
        .trim_ascii()
        .iter()
        .chain(once(&b'0')) // append a zero-length free space to make .tuples() happy
        .map(|c| c - b'0')
        .tuples::<(_, _)>()
        .flat_map(|(file_len, gap_len)| {
            let file_blocks = itertools::repeat_n(file_id, file_len as usize);
            file_id += 1;
            let gap_blocks = itertools::repeat_n(-1, gap_len as usize);
            file_blocks.chain(gap_blocks)
        })
        .collect_vec();
    let mut blocks_b = blocks.clone();

    {
        let (mut left, mut right) = (0usize, blocks.len() - 1);
        while left < right {
            if (blocks[left] != -1) {
                left += 1;
                continue;
            }
            if blocks[right] == -1 {
                right -= 1;
                continue;
            }
            blocks.swap(left, right);
        }
    }

    let a = checksum(&blocks);

    {
        let mut blocks_slice = blocks_b.as_mut_slice();

        loop {
            // Trim empty space at back
            let mut end = blocks_slice.len();
            if end == 0 {
                break;
            }
            while blocks_slice[end - 1] == -1 {
                end -= 1;
            }
            blocks_slice = &mut blocks_slice[..end];

            // Locate the last file
            let mut last_file_begin = end - 1;
            while blocks_slice[last_file_begin] == blocks_slice[last_file_begin - 1] {
                last_file_begin -= 1;
                if last_file_begin == 0 {
                    break;
                }
            }

            {
                let (front, last_file) = blocks_slice.split_at_mut(last_file_begin);

                // Attempt to find a space to move the last file
                match front
                    .chunk_by_mut(|a, b| *a == *b )
                    .find(|chunk| *chunk.first().unwrap() == -1 && chunk.len() >= last_file.len())
                {
                    None => {}
                    Some(window) => {
                        last_file.swap_with_slice(&mut window[..last_file.len()]);
                    }
                }
            }

            // Trim the space the last file occupied, whether we moved it or not
            blocks_slice = &mut blocks_slice[..last_file_begin];
        }
    }

    let b = checksum(&blocks_b);

    SolutionResult::new(a, b)
}

fn checksum(blocks: &[i64]) -> u64 {
    let a: u64 = blocks
        .iter()
        .enumerate()
        .filter(|(_, b)| **b != -1)
        .map(|(i, b)| (i * *b as usize) as u64)
        .sum();
    a
}
