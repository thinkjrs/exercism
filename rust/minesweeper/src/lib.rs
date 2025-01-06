pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut result = vec![];

    minefield.iter().enumerate().for_each(|(_, row)| {
        let mut row_result = String::new();
        row.chars().enumerate().for_each(|(_, c)| {
            row_result.push_str(&c.to_string());
        });
        result.push(row_result);
    });

    // now fill in the blanks with counts
    for i in 0..result.len() {
        for j in 0..result[i].len() {
            if result[i].chars().nth(j).unwrap() == '*' {
                continue;
            }

            let mut count = 0;
            for x in -1..=1 {
                for y in -1..=1 {
                    if i as i32 + x >= 0
                        && i as i32 + x < result.len() as i32
                        && j as i32 + y >= 0
                        && j as i32 + y < result[i].len() as i32
                        && result[(i as i32 + x) as usize]
                            .chars()
                            .nth((j as i32 + y) as usize)
                            .unwrap()
                            == '*'
                    {
                        count += 1;
                    }
                }
            }

            if count > 0 {
                result[i].replace_range(j..=j, &count.to_string());
            }
        }
    }
    result
}
