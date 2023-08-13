fn main() {
    let mut bitmap = [[0u8; 64]; 64];

    // NOTE(casey): Center and radius of the circle
    let center_x: usize = 32;
    let center_y: usize = 32;
    let r: i32 = 20;

    // NOTE(casey): Loop that draws the circle
    {
        let r2 = r + r;

        let mut pen_x = r;
        let mut pen_y = 0;
        let mut dy: i32 = -2;
        let mut dx = r2 + r2 - 4;
        let mut d = r2 - 1;

        while pen_y <= pen_x {
            {
                let pen_x = pen_x as usize;
                let pen_y = pen_y as usize;
                bitmap[center_y - pen_y][center_x - pen_x] = 1;
                bitmap[center_y - pen_y][center_x + pen_x] = 1;
                bitmap[center_y + pen_y][center_x - pen_x] = 1;
                bitmap[center_y + pen_y][center_x + pen_x] = 1;
                bitmap[center_y - pen_x][center_x - pen_y] = 1;
                bitmap[center_y - pen_x][center_x + pen_y] = 1;
                bitmap[center_y + pen_x][center_x - pen_y] = 1;
                bitmap[center_y + pen_x][center_x + pen_y] = 1;
            }

            d += dy;
            dy -= 4;
            pen_y += 1;

            // #if 0
            // // NOTE(casey): Branching version
            // if D < 0 {
            //     D += dX;
            //     dX -= 4;
            //     X -= 1;
            // }
            // #else
            // NOTE(casey): Branchless version
            let mask: i32 = d >> 31;
            d += dx & mask;
            dx -= 4 & mask;
            pen_x += mask;
            // #endif
        }
    }

    // NOTE(casey): Output the bitmap using roughly square elements
    for row in bitmap {
        for cell in row {
            if cell == 1 {
                print!("XX");
            } else {
                print!("  ");
            }
        }
        println!();
    }
}
