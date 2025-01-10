pub fn run() -> (u32, u32) {
    let input = std::fs::read_to_string("data/day04.txt").unwrap().trim().replace('\r', "");
    let data = input.lines().collect::<Vec<&str>>();

    let x_off = vec![-1, -1, -1, 0, 0, 1, 1, 1];
    let y_off = vec![-1, 0, 1, -1, 1, -1, 0, 1];

    let h = data.len();
    let w = data[0].len();

    let mut c = 0;

    for y_ in 0..h {
        for x_ in 0..w {
            if data[y_].chars().nth(x_).unwrap() != 'X' {
                continue;
            }

            for dir in 0..8 {
                let mut found = true;

                for step in 0..4 {
                    let nx = x_ as isize + step * x_off[dir];
                    let ny = y_ as isize + step * y_off[dir];

                    if nx < 0 || ny < 0 || nx >= w as isize || ny >= h as isize {
                        found = false;
                        break;
                    }

                    let to_get = data[ny as usize].chars().nth(nx as usize).unwrap();
                    let get_got = "XMAS".chars().nth(step as usize).unwrap();

                    if to_get != get_got {
                        found = false;
                        break;
                    }
                }

                if found {
                    c += 1;
                }
            }
        }
    }
    let mut c2 = 0;
    let mas1 = [
        [Some('M'), None, Some('S')],
        [None, Some('A'), None],
        [Some('M'), None, Some('S')],
    ];
    let mas2 = [
        [Some('M'), None, Some('M')],
        [None, Some('A'), None],
        [Some('S'), None, Some('S')],
    ];
    let mas3 = [
        [Some('S'), None, Some('M')],
        [None, Some('A'), None],
        [Some('S'), None, Some('M')],
    ];
    let mas4 = [
        [Some('S'), None, Some('S')],
        [None, Some('A'), None],
        [Some('M'), None, Some('M')],
    ];
    for mas in [mas1, mas2, mas3, mas4] {
        for y in 0..(h-2) {
            for x in 0..(w-2) {
                let mut bad = false;
                for yo in 0..=2 {
                    for xo in 0..=2 {
                        let got = data[(y as i32 + yo) as usize].chars().nth((x as i32 + xo) as usize).unwrap();
                        if mas[yo as usize][xo as usize].unwrap_or(got) != got {
                            bad = true;
                        }
                    }
                }
                if !bad {
                    println!("{x} {y}");
                    c2 += 1;
                }
            }
        }
    }


    (c, c2)
}
