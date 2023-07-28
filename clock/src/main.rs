use std::error;
use std::error::Error;
use std::f64::consts::TAU;
use std::io::{stdout, Write};
use std::time::Duration;

use chrono::Timelike;
use crossterm::cursor::MoveTo;
use crossterm::event::poll;
use crossterm::style::Print;
use crossterm::terminal::{enable_raw_mode, size, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{execute, queue};

fn main() {
    match show_clock() {
        Ok(_) => {}
        Err(e) => eprintln!("Something went wrong ({e})"),
    }
    crossterm::terminal::disable_raw_mode().expect("Couldn't disabled raw mode.");
    execute!(stdout(), crossterm::cursor::Show, LeaveAlternateScreen)
        .expect("Couldn't show cursor and leave alternate screen.");
}

fn show_clock() -> Result<(), Box<dyn error::Error>> {
    let (cols, rows) = size()?;
    execute!(stdout(), EnterAlternateScreen, crossterm::cursor::Hide)?;

    enable_raw_mode()?;
    let r: f64 = cols.min(rows) as f64 / 4.;
    let [x, y] = [cols as i32 / 2, rows as i32 / 2];
    draw_face(r, x, y)?;

    let mut dirty: Vec<(i32, i32)> = Vec::new();
    while !poll(Duration::from_millis(1))? {
        for (x, y) in &dirty {
            queue!(stdout(), MoveTo(*x as u16, *y as u16), Print(" "),)?;
        }
        dirty.clear();

        let time = chrono::Local::now();

        for (dx, dy) in get_deltas(r, time.second()) {
            let x = x + dx;
            let y = y + dy;
            dirty.push((x, y));
            queue!(stdout(), MoveTo(x as u16, y as u16), Print("."))?;
        }

        for (dx, dy) in get_deltas(r * 2. / 3., time.minute()) {
            let x = x + dx;
            let y = y + dy;
            dirty.push((x, y));
            queue!(stdout(), MoveTo(x as u16, y as u16), Print("o"))?;
        }

        let hour = (time.hour() % 12) as f64;
        let fraction = time.minute() as f64 / 60.;
        let hour_tick = (hour + fraction) / 12. * 60.;
        for (dx, dy) in get_deltas(r / 2., hour_tick as u32) {
            let x = x + dx;
            let y = y + dy;
            dirty.push((x, y));
            queue!(stdout(), MoveTo(x as u16, y as u16), Print("@"))?;
        }

        stdout().flush()?;
    }
    Ok(())
}

fn get_deltas(r: f64, tick: u32) -> Vec<(i32, i32)> {
    let mut positions: Vec<(i32, i32)> = Vec::new();
    let (dx, dy) = pos_from_tick(r, tick);
    let q = r as u32;
    for n in 1..=q {
        let n = n as f64;
        let i = dx * n / r;
        let j = dy * n / r;
        positions.push((i as i32, j as i32));
    }
    positions
}

fn draw_face(r: f64, x: i32, y: i32) -> Result<(), Box<dyn Error>> {
    for minute in 1..=60 {
        let (dx, dy) = pos_from_tick(r, minute);
        let x = x as f64 + dx;
        let y = y as f64 + dy;
        queue!(stdout(), MoveTo(x as u16, y as u16), Print("."),)?;
    }
    Ok(())
}

fn pos_from_tick(r: f64, tick: u32) -> (f64, f64) {
    let tick = tick as f64;
    let theta = TAU / 4. - TAU / 60. * tick;
    let [dx, dy] = [r * theta.cos() * 2., r * theta.sin()];
    (dx, -dy)
}

#[test]
fn test_ticks() {
    let (x, y) = pos_from_tick(100., 0);
    assert!(x < 0.000001);
    assert_eq!(y, -100.);
    let (x, y) = pos_from_tick(100., 15);
    assert_eq!(x, 200.);
    assert!(y < 0.000001);
    let (x, y) = pos_from_tick(100., 30);
    assert!(x < 0.000001);
    assert_eq!(y, 100.);
    let (x, y) = pos_from_tick(100., 45);
    assert_eq!(x, -200.);
    assert!(y < 0.000001);
}
