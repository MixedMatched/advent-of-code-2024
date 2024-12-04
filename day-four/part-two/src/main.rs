use anyhow::Result;

fn xmas_search(word_search: &Vec<Vec<char>>, loc: &(usize, usize)) -> Option<u32> {
    if *word_search.get(loc.0)?.get(loc.1)? != 'A' {
        return Some(0);
    }
    Some(
        if ((*word_search
            .get(loc.0.checked_sub(1)?)?
            .get(loc.1.checked_sub(1)?)?
            == 'M'
            && *word_search
                .get(loc.0.checked_add(1)?)?
                .get(loc.1.checked_add(1)?)?
                == 'S')
            || (*word_search
                .get(loc.0.checked_sub(1)?)?
                .get(loc.1.checked_sub(1)?)?
                == 'S'
                && *word_search
                    .get(loc.0.checked_add(1)?)?
                    .get(loc.1.checked_add(1)?)?
                    == 'M'))
            && ((*word_search
                .get(loc.0.checked_add(1)?)?
                .get(loc.1.checked_sub(1)?)?
                == 'M'
                && *word_search
                    .get(loc.0.checked_sub(1)?)?
                    .get(loc.1.checked_add(1)?)?
                    == 'S')
                || (*word_search
                    .get(loc.0.checked_add(1)?)?
                    .get(loc.1.checked_sub(1)?)?
                    == 'S'
                    && *word_search
                        .get(loc.0.checked_sub(1)?)?
                        .get(loc.1.checked_add(1)?)?
                        == 'M'))
        {
            1
        } else {
            0
        },
    )
}

fn main() -> Result<()> {
    let data = include_str!("input.txt");

    let mut word_search = Vec::new();
    for line in data.lines() {
        word_search.push(
            line.chars()
                .map(|c| match c {
                    'M' | 'A' | 'S' => c,
                    _ => '.',
                })
                .collect::<Vec<_>>(),
        );
    }

    let mut a_locs = Vec::new();
    word_search.iter().enumerate().for_each(|(col_id, row)| {
        row.iter().enumerate().for_each(|(row_id, c)| {
            if *c == 'A' {
                a_locs.push((col_id, row_id));
            }
        })
    });

    let mut sum = 0;
    for a_loc in a_locs {
        if let Some(xmases) = xmas_search(&word_search, &a_loc) {
            sum += xmases;
        }
    }

    println!("Sum is: {}", sum);

    Ok(())
}
