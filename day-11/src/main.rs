use std::collections::BTreeMap;

fn main() {
    let mut stones = BTreeMap::from([
        (510613, 1),
        (358, 1),
        (84, 1),
        (40702, 1),
        (4373582, 1),
        (2, 1),
        (0, 1 ),
        (1584, 1)
    ]);
    // let mut stones = BTreeMap::from([ 
    //     (125,1),
    //     (17,1)
    // ]);    
    //println!("Initial arrangement:");
    //println!("{:?}\n", stones);
    for i in 0..75 {
        stones = blink_stones(stones);
        //println!("After {} blink(s):", i + 1);
        //println!("Stones: {:?}", stones);
        if i == 24 {
            println!("Part 1 stone count: {:}\n", stone_count(&stones));
        }
    }
    //println!("Stones: {:?}", stones);
    //println!("Unique stones: {}\n", stones.len());
    println!("Part 2 stone count: {}\n", stone_count(&stones));
}

fn blink_stones(stones: BTreeMap<usize, usize>) -> BTreeMap<usize, usize> {
    let mut results = BTreeMap::new();

    for (v, count) in stones {
        let s = v.to_string();
        if v == 0 {
            add_stone(1, count, &mut results);
        } else if s.len() % 2 == 0 {
            let s1 = &s[..s.len() / 2];
            let s2 = &s[s.len() / 2..];
            //println!("s: {}, s1: {}, s2: {}", s, s1, s2);
            let v1 = s1.parse().unwrap();
            let v2 = s2.parse().unwrap();
            add_stone(v1, count, &mut results);
            add_stone(v2, count, &mut results);
        } else {
            add_stone(v * 2024, count, &mut results);
        }

    }
    
    results
}

fn add_stone(stone: usize, count: usize, stones: &mut BTreeMap<usize, usize>) {
    let exists = stones.insert(stone, count);
    let i = exists.unwrap_or(0);
    if i > 0 {
        stones.insert(stone, count + i);
    }
}

fn stone_count(stones: &BTreeMap<usize, usize>) -> usize {
    let mut result = 0;
    for (_v, count) in stones {
        result += count;
    }
    result
}