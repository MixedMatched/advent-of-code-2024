use anyhow::Result;

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

    l1.sort();
    l2.sort();

    let mut dist = 0;
    for (n1, n2) in l1.into_iter().zip(l2) {
        dist += (n1 - n2).abs();
    }

    println!("Total distance is: {}", dist);

    Ok(())
}
