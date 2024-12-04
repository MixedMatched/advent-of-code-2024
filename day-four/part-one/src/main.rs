use anyhow::Result;

fn xmas_search_recursive(
    word_search: &Vec<Vec<char>>,
    loc: &(usize, usize),
    direction: &(i32, i32),
    finding: char,
) -> Option<bool> {
    let new_loc: (usize, usize) = (
        ((loc.0 as i32) + direction.0) as usize,
        ((loc.1 as i32) + direction.1) as usize,
    );
    if *word_search.get(new_loc.0)?.get(new_loc.1)? == finding {
        let next = match finding {
            'M' => 'A',
            'A' => 'S',
            'S' => return Some(true),
            _ => unimplemented!(),
        };
        xmas_search_recursive(word_search, &new_loc, direction, next)
    } else {
        Some(false)
    }
}

fn xmas_search(word_search: &Vec<Vec<char>>, loc: &(usize, usize)) -> Option<u32> {
    if *word_search.get(loc.0)?.get(loc.1)? != 'X' {
        return Some(0);
    }
    Some(
        [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            // no (0, 0)
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .iter()
        .map(|dir| {
            if xmas_search_recursive(word_search, loc, dir, 'M').unwrap_or_else(|| false) {
                1
            } else {
                0
            }
        })
        .sum(),
    )
}

fn main() -> Result<()> {
    let data = include_str!("input.txt");

    let mut word_search = Vec::new();
    for line in data.lines() {
        word_search.push(
            line.chars()
                .map(|c| match c {
                    'X' | 'M' | 'A' | 'S' => c,
                    _ => '.',
                })
                .collect::<Vec<_>>(),
        );
    }

    let mut x_locs = Vec::new();
    word_search.iter().enumerate().for_each(|(col_id, row)| {
        row.iter().enumerate().for_each(|(row_id, c)| {
            if *c == 'X' {
                x_locs.push((col_id, row_id));
            }
        })
    });

    let mut sum = 0;
    for x_loc in x_locs {
        if let Some(xmases) = xmas_search(&word_search, &x_loc) {
            sum += xmases;
        }
    }

    println!("Sum is: {}", sum);

    Ok(())
}
