use crate::runner;
use crate::util::DynResult;

runner!();

#[derive(Clone)]
struct Block {
    length: usize,
    data: Option<u16>,
}
impl Block {
    fn checksum(&self, start: u64) -> u64 {
        if self.data.is_none() {
            return 0;
        }

        let len = self.length as u64;
        let id = self.data.unwrap() as u64;

        let triangle = (len * (len - 1)) / 2;
        let rectangle = start * len;

        (triangle + rectangle) * id
    }
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

fn part1(blocks: &ParsedData) -> u64 {
    let mut blocks = blocks.clone();

    let mut first_free = 1;
    let mut last_full = blocks.len() - 1;

    loop {
        if first_free >= last_full {
            break;
        }

        let free_size = blocks[first_free].length;
        let full_size = blocks[last_full].length;

        if free_size == 0 {
            first_free += 2;
            continue;
        }
        if free_size < full_size {
            // We can't fit the entire rest of the block in the free space.
            // Fill the free block and move on to the next free block

            blocks[first_free].data = blocks[last_full].data;
            blocks[last_full].length -= free_size;

            first_free += 2;
        } else if free_size > full_size {
            // The entire block fits in the free space.
            // Move the full block before the free space and get the next full block

            let full_block = blocks.remove(last_full);
            blocks[first_free].length -= full_block.length;
            blocks.insert(first_free, full_block);

            first_free += 1;
            last_full -= 1;
        } else {
            // The blocks have the same size, just swap them and move on.
            blocks.swap(first_free, last_full);

            first_free += 2;
            last_full -= 2;
        }
    }

    let mut sum = 0;
    let mut pos = 0;
    for block in blocks.into_iter() {
        sum += block.checksum(pos);
        pos += block.length as u64;
    }

    sum
}

fn find_file_id(blocks: &ParsedData, search_id: u16, start: usize) -> usize {
    for i in (0..=start).rev() {
        let block = &blocks[i];

        match block.data {
            Some(id) => {
                if id == search_id {
                    return i;
                }
            }
            None => {}
        }
    }

    unreachable!();
}
fn find_free_space(blocks: &ParsedData, size: usize, pos: usize) -> Option<usize> {
    for i in 1..pos {
        let block = &blocks[i];

        if block.length >= size && block.data.is_none() {
            return Some(i);
        }
    }
    None
}
fn part2(blocks: &ParsedData) -> u64 {
    let mut blocks = blocks.clone();

    let mut search_index = blocks.len() - 1;

    for file_id in (1..=blocks.last().unwrap().data.unwrap()).rev() {
        let file_index = find_file_id(&blocks, file_id, search_index);
        let file = &blocks[file_index];

        let free_index = match find_free_space(&blocks, file.length, file_index) {
            Some(i) => i,
            None => continue,
        };
        let free_block = &blocks[free_index];

        if free_block.length == file.length {
            blocks.swap(file_index, free_index);
            search_index = file_index - 1;
        } else {
            blocks[free_index].length -= file.length;

            blocks.insert(free_index, blocks[file_index].clone());
            blocks[file_index + 1].data = None;
        }
    }

    let mut sum = 0;
    let mut pos = 0;
    for block in blocks.into_iter() {
        sum += block.checksum(pos);
        pos += block.length as u64;
    }

    sum
}
