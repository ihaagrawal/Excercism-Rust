pub fn annotate(garden: &[&str]) -> Vec<String> {
    let rows = garden.len();
    let cols = if rows > 0 { garden[0].len() } else { 0 };

    let mut grid = vec![vec![0u8; cols]; rows];

    let dirs = [
        (-1,-1), (-1,0), (-1,1),
        (0,-1),          (0,1),
        (1,-1),  (1,0),  (1,1),
    ];

    // Step 1: mark flowers and increment neighbors
    for i in 0..rows {
        for j in 0..cols {
            if garden[i].as_bytes()[j] == b'*' {
                grid[i][j] = b'*';

                for (di, dj) in dirs {
                    let ni = i as i32 + di;
                    let nj = j as i32 + dj;

                    if ni >= 0 && nj >= 0 &&
                       (ni as usize) < rows &&
                       (nj as usize) < cols {

                        let cell = &mut grid[ni as usize][nj as usize];
                        if *cell != b'*' {
                            *cell += 1;
                        }
                    }
                }
            }
        }
    }

    // Step 2: build output
    let mut result = Vec::new();

    for i in 0..rows {
        let mut row = String::new();
        for j in 0..cols {
            match grid[i][j] {
                b'*' => row.push('*'),
                0 => row.push(' '),
                n => row.push((b'0' + n) as char),
            }
        }
        result.push(row);
    }

    result
}