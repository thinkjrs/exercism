pub fn annotate(minefield: &[&str]) -> Vec<String> {
    // we specify the relative neighbors for each cell
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    minefield
        .iter()
        .enumerate()
        .map(|(i, row)| {
            let row_bytes = row.as_bytes();
            (0..row_bytes.len())
                .map(|j| {
                    if row_bytes[j] == b'*' {
                        '*'
                    } else {
                        let count = directions
                            .iter()
                            .filter(|&&(di, dj)| {
                                let ni = i as isize + di;
                                let nj = j as isize + dj;
                                ni >= 0
                                    && nj >= 0
                                    && ni < minefield.len() as isize
                                    && nj < row_bytes.len() as isize
                                    && minefield[ni as usize].as_bytes()[nj as usize] == b'*'
                            })
                            .count();
                        if count > 0 {
                            (b'0' + count as u8) as char
                        } else {
                            ' '
                        }
                    }
                })
                .collect()
        })
        .collect()
}
