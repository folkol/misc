/*
 - inspired by: https://github.com/Rezmason/matrix
 - symbols don't move, they are just 'illuminated' by the raindrops
 - bloom + tone mapped to green
 - can have multiple raindrops in the same column,they don't collide
 - characters change in a repeating pattern
*/

use std::io::{stdout, BufWriter, Write};
use std::thread;
use std::time::{Duration, SystemTime};

use rand::distributions::Alphanumeric;
use rand::prelude::SliceRandom;
use rand::{thread_rng, Rng};
use termion::color::{Fg, Reset, Rgb};
use termion::cursor::Show;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::IntoAlternateScreen;

const TAIL: i32 = 20;
const MAX_FADE: f32 = 0.2;
const VISCOSITY: u32 = 30;

fn main() {
    let (cols, rows) = termion::terminal_size().unwrap();
    let stdout = BufWriter::new(stdout().lock());
    let mut stdout = stdout
        .into_alternate_screen()
        .unwrap()
        .into_raw_mode()
        .unwrap();
    writeln!(stdout, "{}", termion::cursor::Hide).unwrap();
    let mut events = termion::async_stdin().events();
    let mut rng = thread_rng();
    let mut grid: Vec<(char, f32)> = vec![(' ', 0.0); (rows * cols) as usize];
    grid.iter_mut().for_each(|(c, _)| {
        *c = rng.sample(Alphanumeric) as char;
    });

    let mut droplets: Vec<(usize, i32, u32, SystemTime)> =
        vec![(1, TAIL + rows as i32, 0, SystemTime::now()); (cols / 5) as usize];
    loop {
        if events.next().is_some() {
            break;
        }
        for y in 0..rows {
            for x in 0..cols {
                if rng.gen::<f32>() < 0.001 {
                    let i = y * cols + x;
                    let c = rng.sample(Alphanumeric) as char;
                    grid[i as usize].0 = c;
                    write!(
                        stdout,
                        "{}{}{}",
                        termion::cursor::Goto(x + 1, y + 1),
                        Fg(Rgb(0, (grid[i as usize].1 * 255.0) as u8, 0)),
                        c
                    )
                    .unwrap();
                }
            }
        }
        let now = SystemTime::now();
        for (x, y, v, next_move) in droplets.iter_mut() {
            if *next_move < now {
                *next_move = now + Duration::from_millis(*v as u64);
                *y += 1;
                for i in ((*y - TAIL)..=*y).rev() {
                    let step = *y - i;
                    if i >= 1 && i <= rows as i32 {
                        let mut c = 1.0 - step as f32 * (1.0 / TAIL as f32);
                        if c < MAX_FADE {
                            c = MAX_FADE;
                        }
                        let pos = (i as usize - 1) * cols as usize + (*x - 1);

                        if pos < grid.len() {
                            let cell = &mut grid[pos];
                            cell.1 = c;
                            let a = *x as u16;
                            let b = i as u16;
                            if a >= 1 && a <= cols && b >= 1 && b <= rows {
                                write!(
                                    stdout,
                                    "{}{}{}",
                                    termion::cursor::Goto(a, b),
                                    Fg(Rgb(0, (c * 255.0) as u8, 0)),
                                    cell.0
                                )
                                .unwrap();
                            }
                        }
                    }
                }
            }
        }
        let mut free_lanes: Vec<_> = (1..=cols)
            .filter(|x| !droplets.iter().map(|d| d.0).any(|d| d == *x as usize))
            .collect();
        free_lanes.shuffle(&mut rng);
        let mut next_lanes = free_lanes.iter();
        for (x, y, v, next_move) in droplets.iter_mut() {
            if *y >= cols as i32 + TAIL {
                *y = rng.gen_range(-(rows as i32)..0);
                *x = *(next_lanes.next().unwrap()) as usize;
                *v = rng.gen_range(1..=3) * VISCOSITY;
                *next_move = now + Duration::from_millis(*v as u64);
            }
        }

        stdout.flush().unwrap();
        thread::sleep_ms(1);
    }
    writeln!(stdout, "{}{}", Show, Fg(Reset)).unwrap();
}
