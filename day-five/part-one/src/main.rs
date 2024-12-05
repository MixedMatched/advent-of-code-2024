use std::collections::HashMap;

use anyhow::Result;

fn main() -> Result<()> {
    let data = include_str!("input.txt");

    let mut relations: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut updates = Vec::new();

    enum ParsePhase {
        Relation,
        Update,
    }

    let mut parse_phase = ParsePhase::Relation;
    for line in data.lines() {
        match (line, &parse_phase) {
            ("", ParsePhase::Relation) => parse_phase = ParsePhase::Update,
            (_, ParsePhase::Relation) => {
                let st = line.split_once('|').expect("bad relation");
                let nt = (st.0.parse::<u32>()?, st.1.parse::<u32>()?);
                relations
                    .entry(nt.1)
                    .and_modify(|v| v.push(nt.0))
                    .or_insert(vec![nt.0]);
            }
            (_, ParsePhase::Update) => updates.push(
                line.split(',')
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect::<Vec<_>>(),
            ),
        }
    }

    let mut sum = 0;
    'outer: for update in updates {
        let mut blacklist = Vec::new();
        for page in update.iter() {
            if blacklist.contains(page) {
                continue 'outer;
            }
            blacklist.append(&mut relations.get(page).unwrap_or(&Vec::new()).clone());
        }
        sum += update
            .get(update.len() / 2)
            .expect("update doesn't contain its own half?");
    }

    println!("Sum is: {}", sum);

    Ok(())
}
