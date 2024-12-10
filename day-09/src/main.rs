#[derive(Clone, Copy, Debug)]
struct File {
    id: usize,   // file id #
    node: usize, // staring node # of the file
    len: u8,     // length of file in blocks
}

impl File {
    fn new(id: usize, node: usize, len: u8) -> Self {
        Self { id, node, len }
    }
}
fn main() {
    // get our compact disk map in memory
    let disk_map = String::from(include_str!("../input.txt"));
    // create a file directory to store file information in from the disk_map
    let mut dir: Vec<File> = Vec::new();
    parse_disk_map(disk_map, &mut dir);
    println!("{} files on disk.", dir.len());
    // now that we have a parsed directory, let's build a block map in memory
    let mut block_map = build_block_map(&dir);
    //println!("Block map: {:?}", block_map);
    // compress it!
    compress(&mut block_map);
    // compute the checksum
    let check = checksum(&block_map);
    println!("Part 1 checksum: {}", check);

    // Part 2 - compress without fragmentation!
    // rebuild the block map in memory
    let mut block_map = build_block_map(&dir);
    // compress it (without fragmenting files this time)
    compress_no_frag(&mut dir, &mut block_map);
    // compute the checksum
    let check = checksum(&block_map);
    println!("Part 2 checksum: {}", check);
}

fn parse_disk_map(disk_map: String, dir: &mut Vec<File>) {
    let mut node = 0;
    for id in 0..(disk_map.len() + 1) / 2 {
        let start = id * 2;
        let mut end = (id + 1) * 2;
        if end > disk_map.len() {
            end -= 1;
        }
        let entry = &disk_map[start..end];
        let len = entry[0..1].parse().unwrap();
        let empty: u8 = if (end - start) == 2 {
            entry[1..2].parse().unwrap()
        } else {
            0
        };
        let file = File::new(id, node, len);
        dir.push(file);
        node = node + len as usize + empty as usize;
    }
}

fn build_block_map(dir: &Vec<File>) -> Vec<usize> {
    // compute total # of blocks required
    let last = dir[dir.len() - 1];
    let blocks_required = last.node + last.len as usize;
    let mut block_map = Vec::with_capacity(blocks_required);

    // set all block values to 0 to signify empty
    for _i in 0..blocks_required {
        block_map.push(0);
    }
    for file in dir {
        for i in 0..file.len {
            block_map[file.node + i as usize] = file.id + 1;
        }
    }
    // return the created block map
    block_map
}

fn compress(block_map: &mut Vec<usize>) {
    let mut no_more_empties = false;
    let len = block_map.len();
    // starting at the end of the block map, try and move a # > 0 to the first block = 0
    for i in 0..len {
        // if there are no more empty blocks
        if no_more_empties {
            // then we are done
            break;
        }
        // if the current block is already empty
        if block_map[len - i - 1] == 0 {
            // then we can move down to the next block
            continue;
        }
        // ok, we have a block with a non-zero value
        // find the first zero block and move it there!
        no_more_empties = true;
        for j in 0..len - i - 1 {
            if block_map[j] == 0 {
                // found an empty!
                block_map[j] = block_map[len - i - 1];
                block_map[len - i - 1] = 0;
                no_more_empties = false;
                //println!("Block map: {:?}", block_map);
                break;
            }
        }
    }
}

fn compress_no_frag(dir: &mut Vec<File>, block_map: &mut Vec<usize>) {
    // loop through all files in our directory in reverse order
    println!("Block map: {:?}", block_map);
    for i in (0..dir.len()).rev() {
        let mut file = dir[i];
        let new = find_empty_blocks(file.node, file.len, &block_map);
        if new > 0 {
            println!("Move file #{} to {}.", file.id + 1, new);
            move_file(&mut file, new, block_map);
            //println!("Block map: {:?}", block_map);
        }
    }
}

fn find_empty_blocks(last: usize, len: u8, block_map: &Vec<usize>) -> usize {
    // search through the block_map to find file.len empty blocks
    let mut count = 0_u8;
    let mut start = 0;
    for j in 0..last {
        if block_map[j] == 0 {
            count += 1;
            start = if count == 1 { j } else { start }
        } else {
            count = 0;
        }
        if count == len {
            break;
        }
    }
    if count == len { start } else { 0 }
}

fn move_file(file: &mut File, new: usize, block_map: &mut Vec<usize>) {
    for n in 0..file.len {
        block_map[new + n as usize] = block_map[file.node + n as usize];
        block_map[file.node + n as usize] = 0;
    }
    file.node = new;
}

fn checksum(block_map: &Vec<usize>) -> usize {
    let mut result = 0;
    for i in 0..block_map.len() {
        if block_map[i] == 0 {
            continue;
        }
        result = result + i * (block_map[i] - 1);
    }
    result
}
