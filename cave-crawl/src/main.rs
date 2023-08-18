use std::io::{self, Stdout, Write};
use std::thread;
use std::time::Duration;

use crossterm::{
    cursor, execute, queue,
    style::{self, Stylize},
    terminal,
};
use crossterm::cursor::{Hide, Show};
use crossterm::event::{Event, KeyCode, poll, read};

const MAP: [&[u8; 60]; 20] = [
    b"############################################################",
    b"#        ###################################################",
    b"#                                                          #",
    b"#        #####  ############################  ########  ####",
    b"####  ########  ############################  ########  ####",
    b"#        ##        ##                  ##        ##        #",
    b"#                  ##                            ##        #",
    b"#        ##        ##                  ##                  #",
    b"#        ##        ##                  ##        ##        #",
    b"####  ###############                  #####################",
    b"####  ###############                  #####################",
    b"#        ##        ##                  ##        ##        #",
    b"#        ##                            ##                  #",
    b"#        ##        ##                  ##        ##        #",
    b"####  ########  ########  ########  ########  ########  ####",
    b"#        ##        ##        ##        ##                  #",
    b"#                  ##        ##                            #",
    b"#        ##        ##        ##        ##                  #",
    b"#        ##        ##        ##        ##                ! #",
    b"############################################################",
];

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();

    terminal::enable_raw_mode()?;
    execute!(
        stdout,
        Hide,
        terminal::EnterAlternateScreen,
        terminal::Clear(terminal::ClearType::All)
    )?;

    let (cols, rows) = terminal::size()?;
    let (mut player_x, mut player_y) = (3, 3);
    let (mut prev_player_x, mut prev_player_y) = (player_x, player_y);
    repaint(&mut stdout, player_x, player_y)?;

    loop {
        if poll(Duration::from_millis(1))? {
            if let Event::Key(e) = read()? {
                match e.code {
                    KeyCode::Up => {
                        if MAP[(player_y - 1) as usize][player_x as usize] == b' ' {
                            player_y -= 1
                        }
                    }
                    KeyCode::Down => {
                        if MAP[(player_y + 1) as usize][player_x as usize] == b' ' {
                            player_y += 1
                        }
                    }
                    KeyCode::Left => {
                        if MAP[player_y as usize][(player_x - 1) as usize] == b' ' {
                            player_x -= 1
                        }
                    }
                    KeyCode::Right => {
                        if MAP[player_y as usize][(player_x + 1) as usize] == b' ' {
                            player_x += 1
                        }
                    }
                    _ => break,
                }
                queue!(
                    stdout,
                    cursor::MoveTo(prev_player_x, prev_player_y),
                    style::PrintStyledContent(" ".black()),
                )?;
                repaint(&mut stdout, player_x, player_y)?;
            }
        }

        stdout.flush()?;
        (prev_player_x, prev_player_y) = (player_x, player_y);

        if player_x > 55 && player_y > 16 {
            execute!(
                stdout,
                cursor::MoveTo(cols / 2, rows / 2),
                terminal::Clear(terminal::ClearType::All),
                style::PrintStyledContent("YOU WIN".magenta()),
            )?;
            thread::sleep_ms(2000);
            break;
        }
    }
    terminal::disable_raw_mode()?;
    execute!(stdout, Show, terminal::LeaveAlternateScreen,)?;
    Ok(())
}

fn repaint(stdout: &mut Stdout, player_x: u16, player_y: u16) -> io::Result<()> {
    for (row, line) in MAP.iter().enumerate() {
        for (col, x) in line.iter().enumerate() {
            if los(col as i32, row as i32, player_x as i32, player_y as i32) {
                queue!(
                    stdout,
                    cursor::MoveTo(col as u16, row as u16),
                    style::SetBackgroundColor(style::Color::DarkGrey),
                    style::Print(*x as char)
                )?
            } else {
                queue!(
                    stdout,
                    cursor::MoveTo(col as u16, row as u16),
                    style::SetBackgroundColor(style::Color::Black),
                    style::Print(' ')
                )?
            }
        }
    }
    queue!(
        stdout,
        cursor::MoveTo(player_x, player_y),
        style::PrintStyledContent("@".green())
    )?;
    Ok(())
}

fn los(x0: i32, y0: i32, x1: i32, y1: i32) -> bool {
    let steps = 100;
    let dx = (x1 - x0) as f32 / steps as f32;
    let dy = (y1 - y0) as f32 / steps as f32;
    let mut i = x0 as f32;
    let mut j = y0 as f32;
    for _ in 0..steps {
        let x = i.round() as usize;
        let y = j.round() as usize;
        if MAP[y][x] == b'#' {
            return false;
        }
        i += dx;
        j += dy;
    }
    return true;
}
