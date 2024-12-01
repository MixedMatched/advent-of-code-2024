use anyhow::Result;
use counter::Counter;

fn main() -> Result<()> {
    let input = include_str!("input.txt");

    let mut l1 = Vec::new();
    let mut l2 = Vec::new();

    for line in input.lines() {
        if let Some((s1, s2)) = line.split_once(" ") {
            println!("s1: \"{}\", s2: \"{}\"", s1, s2);
            l1.push(s1.trim().parse::<i32>()?);
            l2.push(s2.trim().parse::<i32>()?);
        } else {
            println!("Error parsing line: {}", line);
        }
    }

    let c: Counter<i32, i32> = l2.into_iter().collect::<Counter<_, _>>();

    let mut similarity = 0;
    for n in l1 {
        if let Some(instances) = c.get(&n) {
            similarity += n * instances;
        }
    }

    println!("similarity is: {}", similarity);

    Ok(())
}
