use anyhow::Result;

fn check(nums: &[i32]) -> bool {
    let cmp = nums[0].cmp(&nums[1]);
    let mut prev = &nums[0];
    let mut is_safe = true;

    for num in nums.into_iter().skip(1) {
        if !(prev.cmp(&num) == cmp) || (prev - num).abs() < 1 || (prev - num).abs() > 3 {
            is_safe = false;
            break;
        }

        prev = num;
    }

    is_safe
}

fn main() -> Result<()> {
    let data = include_str!("input.txt");

    let mut num_safe = 0;
    for line in data.lines() {
        let nums = line
            .split(" ")
            .map(|s| s.trim().parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        if check(&nums)
            || (0..nums.len()).any(|n| check(&vec![&nums[0..n], &nums[n + 1..]].concat()))
        {
            num_safe += 1;
        }
    }

    println!("num_safe is: {}", num_safe);

    Ok(())
}
