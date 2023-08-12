use std::env::args;
use std::io::{Read, stdout, Write};
use std::time::Duration;

use crossterm::{
    ExecutableCommand,
    execute,
    style::Print,
};
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{Event, poll, read};
use crossterm::terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::terminal::size;

const WIDTH: usize = 34;
const HEIGHT: usize = 17;

fn main() -> std::io::Result<()> {
    let walk = args().nth(1).unwrap_or("Hello, world!".to_owned());
    let (cols, rows) = size()?;
    let (cols, rows) = (cols as usize, rows as usize);

    execute!(
        stdout(),
        Hide,
        EnterAlternateScreen,
        Clear(ClearType::All),
    )?;
    crossterm::terminal::enable_raw_mode()?;

    draw_outline(cols, rows)?;
    stdout().flush()?;

    let mut coins = [0usize; WIDTH * HEIGHT];

    let (mut x, mut y) = (WIDTH / 2, HEIGHT / 2);
    'label: for byte in walk.bytes() {
        for step in [byte >> 6 & 0b11, byte >> 4 & 0b11, byte >> 2 & 0b11, byte & 0b11] {
            match step {
                0b00 => {
                    x = x.saturating_sub(1);
                    y = y.saturating_sub(1);
                } // up left
                0b01 => {
                    if x < WIDTH - 1 {
                        x += 1;
                    }
                    y = y.saturating_sub(1);
                } // up right
                0b10 => {
                    x = x.saturating_sub(1);
                    if y < HEIGHT - 1 {
                        y += 1;
                    }
                } // down left
                0b11 => {
                    if x < WIDTH - 1 {
                        x += 1;
                    }
                    if y < HEIGHT - 1 {
                        y += 1;
                    }
                } // down right
                _ => panic!("Unexpected bit pattern!")
            }
            coins[y * WIDTH + x] += 1;
            let symbol = match coins[y * WIDTH + x] {
                0 => ' ',
                1 => ' ',
                2 => '.',
                3 => 'o',
                4 => '+',
                5 => '=',
                6 => '*',
                7 => 'B',
                8 => '0',
                9 => 'X',
                10 => '@',
                11 => '%',
                12 => '&',
                13 => '#',
                14 => '/',
                15 => '^',
                _ => '?'
            };
            let (x, y) = (cols / 2 - WIDTH / 2 + x, rows / 2 - HEIGHT / 2 + y);
            execute!(
                stdout(),
                MoveTo(x as u16, y as u16),
                Print(symbol)
            )?;
            if poll(Duration::from_millis(1000 / 10))? {
                if let Event::Key(_) = read()? { break 'label; }
            }
        }
    }

    crossterm::terminal::disable_raw_mode()?;
    execute!(
        stdout(),
        Show,
        LeaveAlternateScreen
    )?;

    Ok(())
}

fn draw_outline(cols: usize, rows: usize) -> std::io::Result<()> {
    for col in 0..WIDTH {
        let col = (cols / 2 - WIDTH / 2 + col) as u16;
        let row = (rows / 2) as u16;
        execute!(
            stdout(),
            MoveTo(col, row - HEIGHT as u16 / 2 - 1),
            Print("-"),
            MoveTo(col, row + HEIGHT as u16 / 2 + 1),
            Print("-")
        )?;
    }
    for row in 0..HEIGHT {
        let col = (cols / 2) as u16;
        let row = (rows / 2 - HEIGHT / 2 + row) as u16;
        execute!(
            stdout(),
            MoveTo(col - WIDTH as u16 / 2 - 1, row),
            Print("|"),
            MoveTo(col + WIDTH as u16 / 2, row),
            Print("|")
        )?;
    }
    execute!(
            stdout(),
            MoveTo((cols / 2 - WIDTH / 2)  as u16 - 1, (rows / 2 - HEIGHT / 2)  as u16 - 1),
            Print("+"),
            MoveTo((cols / 2 + WIDTH / 2)as u16,( rows / 2 - HEIGHT / 2)  as u16 - 1),
            Print("+"),
            MoveTo((cols / 2 - WIDTH / 2)  as u16 - 1, (rows / 2 + HEIGHT / 2)  as u16 + 1),
            Print("+"),
            MoveTo((cols / 2 + WIDTH / 2)  as u16, (rows / 2 + HEIGHT / 2)  as u16 + 1),
            Print("+"),
        )?;
    Ok(())
}