use std::cmp::Reverse;
use std::collections::BinaryHeap;

use crate::runner;
use crate::util::DynResult;

runner!();

struct Block {
    length: usize,
    data: Option<usize>,
}

type ParsedData = Vec<Block>;
fn parse(input: &str) -> DynResult<ParsedData> {
    let mut blocks = Vec::new();

    let mut free = false;
    let mut id = 0;
    for c in input.trim().chars() {
        let length = c.to_digit(10).unwrap() as usize;

        let data = if free {
            None
        } else {
            id += 1;
            Some(id - 1)
        };
        free = !free;

        blocks.push(Block { length, data });
    }
    // If the last block is empty it doesn't matter.
    // Might as well pop it so the list always ends with a full block
    blocks.pop_if(|b| b.data.is_none());

    Ok(blocks)
}

fn calc_checksum(start: usize, len: usize, id: usize) -> usize {
    if len == 0 {
        return 0;
    }

    let triangle = (len * (len - 1)) / 2;
    let rectangle = start * len;

    (triangle + rectangle) * id
}

fn part1(blocks: &ParsedData) -> usize {
    //  The checksum of file zero is always 0, so it can just be skipped
    let mut checksum = 0;
    let mut pos = blocks[0].length;

    let mut first_free = 1;
    let mut last_full = blocks.len() - 1;

    let mut free_size = blocks[first_free].length;
    let mut full_size = blocks[last_full].length;

    loop {
        if free_size == 0 {
            first_free += 2;
            free_size = blocks[first_free].length;

            if first_free > last_full {
                break;
            }

            let skipped = &blocks[first_free - 1];
            checksum += calc_checksum(pos, skipped.length, skipped.data.unwrap());
            pos += skipped.length;
        }
        if full_size == 0 {
            last_full -= 2;
            full_size = blocks[last_full].length;
        }

        let taken = full_size.min(free_size);

        free_size -= taken;
        full_size -= taken;

        checksum += calc_checksum(pos, taken, blocks[last_full].data.unwrap());
        pos += taken;
    }

    checksum += calc_checksum(pos, full_size, blocks[last_full].data.unwrap());

    checksum
}
fn part2(blocks: &ParsedData) -> usize {
    let mut checksum = 0;
    let mut pos = 0;

    // Starting index of blocks with free size of n, including 0 for ease of indexing
    let mut free_blocks = vec![BinaryHeap::new(); 10];

    for block in blocks {
        if block.data.is_none() && block.length != 0 {
            free_blocks[block.length].push(Reverse(pos));
        }
        pos += block.length;
    }

    for block in blocks.iter().rev() {
        pos -= block.length;

        if block.data.is_none() {
            continue;
        }

        let spot = free_blocks
            .iter_mut()
            .enumerate()
            .skip(block.length)
            .filter(|(_, v)| !v.is_empty())
            .filter(|(_, v)| v.peek().unwrap().0 < pos)
            .min_by(|(_, v1), (_, v2)| v1.peek().unwrap().0.cmp(&v2.peek().unwrap().0));

        let Some((free_size, free_vec)) = spot else {
            checksum += calc_checksum(pos, block.length, block.data.unwrap());
            continue;
        };

        let Reverse(free_pos) = free_vec.pop().unwrap();
        checksum += calc_checksum(free_pos, block.length, block.data.unwrap());

        let free_size = free_size - block.length;

        if free_size > 0 {
            let free_pos = free_pos + block.length;
            free_blocks[free_size].push(Reverse(free_pos));
        }
    }

    checksum
}
