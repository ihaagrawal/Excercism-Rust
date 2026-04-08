pub fn annotate(garden: &[&str]) -> Vec<String> {
    let dirs = [
        (-1,-1), (-1,0), (-1,1),
        (0,-1),          (0,1),
        (1,-1),  (1,0),  (1,1),
    ];

    let mut result = Vec::new();

    for i in 0..garden.len() {
        let mut row = String::new();

        for j in 0..garden[i].len() {
            let cell = garden[i].as_bytes()[j];

            if cell == b'*' {
                row.push('*');
                continue;
            }

            let mut count = 0;

            for (di, dj) in dirs {
                let ni = i as i32 + di;
                let nj = j as i32 + dj;

                if ni >= 0
                    && nj >= 0
                    && (ni as usize) < garden.len()
                    && (nj as usize) < garden[i].len()
                {
                    if garden[ni as usize].as_bytes()[nj as usize] == b'*' {
                        count += 1;
                    }
                }
            }

            if count == 0 {
                row.push(' ');
            } else {
                row.push((b'0' + count as u8) as char);
            }
        }

        result.push(row);
    }

    result
}