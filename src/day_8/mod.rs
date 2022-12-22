use crate::read_txt_file;

pub fn solve() {
    println!("DAY 8");
    let input = read_txt_file(8, crate::TextEnum::Input);

    let rows = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut visible: u32 = 0;
    let mut max_scenic = 0;

    for (y, row) in rows.iter().enumerate() {
        for (x, t) in row.iter().enumerate() {
            // [left, top, right, bottom]
            let mut visibility = [true; 4];
            let mut scenic = [0; 4];

            // check vertically, top to bottom
            for i in (0..y).rev() {
                scenic[1] += 1;
                if &rows[i][x] >= t {
                    visibility[1] = false;
                    break;
                }
            }

            // checking vertically, bottom to top
            for i in (y + 1)..rows.len() {
                scenic[3] += 1;
                if &rows[i][x] >= t {
                    visibility[3] = false;
                    break;
                }
            }

            // check horizontally, left to right
            for i in (0..x).rev() {
                scenic[0] += 1;
                if row[i] >= *t {
                    visibility[0] = false;
                    break;
                }
            }

            // check horizontally, right to left
            for i in (x + 1)..(row.len()) {
                scenic[2] += 1;
                if row[i] >= *t {
                    visibility[2] = false;
                    break;
                }
            }

            if visibility.contains(&true) {
                visible += 1;
            }

            let mut scenic_product = 1;

            for sum in scenic {
                scenic_product *= sum;
            }

            if scenic_product > max_scenic {
                max_scenic = scenic_product;
            }
        }
    }

    println!("Number of visible trees: {:?}", visible);
    println!("Max scenic attainable: {:?}", max_scenic);
}
