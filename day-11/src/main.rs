fn main() {
    let mut stones = vec![510613, 358, 84, 40702, 4373582, 2, 0, 1584];

    println!("Initial arrangement:");
    println!("{:?}\n", stones);
    for i in 0..75 {
        blink_stones(&mut stones);
        println!("After {} blink(s):", i + 1);
        println!("{:?}\n", stones.len());
    }
    println!("Total stones: {}\n", stones.len());
}

fn blink_stones(stones: &mut Vec<usize>) {
    let len = stones.len();
    let mut n = 0_usize;
    loop {
        let v = stones[n];
        let s = v.to_string();
        if v == 0 {
            stones[n] = 1;
            n += 1;
        } else if s.len() % 2 == 0 {
            let s1 = &s[..s.len() / 2];
            let s2 = &s[s.len() / 2..];
            //println!("s: {}, s1: {}, s2: {}", s, s1, s2);
            let v1 = s1.parse().unwrap();
            let v2 = s2.parse().unwrap();
            stones[n] = v1;
            n += 1;
            stones.insert(n, v2);
            n += 1;
        } else {
            stones[n] = v * 2024;
            n += 1;
        }
        if n == stones.len() {
            break;
        }
    }
}
