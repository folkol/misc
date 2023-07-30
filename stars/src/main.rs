extern crate core;

use std::io::{stdin, stdout, Stdout, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use rand::Rng;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::{AlternateScreen, IntoAlternateScreen};

fn main() {
    let mut alternate = enter_alternate_raw_or_die();

    write!(alternate, "{}", termion::cursor::Hide).unwrap();
    let running = Arc::new(AtomicBool::new(true));
    start_poll_stdin(&running);

    let (w, h) = get_terminal_size();
    let mut stars: Vec<_> = spawn_stars(w, h);
    let mut next_paint = Instant::now();
    while running.load(Ordering::Relaxed) {
        for (x, y, v) in &mut stars {
            let x_pos = *x as u16;
            write!(alternate, "{} ", termion::cursor::Goto(x_pos, *y))
                .expect("Couldn't write to alternate screen");
            *x += *v;
            let w = w as f64;
            if *x >= w {
                *x -= w
            }

            let x_pos = *x as u16;
            write!(alternate, "{}.", termion::cursor::Goto(x_pos, *y))
                .expect("Couldn't write to alternate screen")
        }

        if next_paint > Instant::now() {
            thread::sleep(next_paint - Instant::now())
        }
        alternate.flush().unwrap();
        next_paint += Duration::from_millis(1000 / 60);
    }
    write!(alternate, "{}", termion::cursor::Show).expect("Failed to reset terminal");
}

fn spawn_stars(w: u16, h: u16) -> Vec<(f64, u16, f64)> {
    let mut rng = rand::thread_rng();
    (0..100)
        .map(|_| {
            (
                rng.gen_range(1..w) as f64,
                rng.gen_range(1..h),
                rng.gen_range(1..100) as f64 / 100.,
            )
        })
        .collect()
}

fn get_terminal_size() -> (u16, u16) {
    match termion::terminal_size() {
        Ok(size) => size,
        Err(_) => {
            eprintln!("Couldn't figure out terminal size");
            std::process::exit(1);
        }
    }
}

fn start_poll_stdin(running: &Arc<AtomicBool>) {
    let running_clone = Arc::clone(running);
    thread::spawn(move || {
        let e = stdin().events().next();
        eprintln!("got event ({e:?}), quitting");
        running_clone.store(false, Ordering::Relaxed);
    });
}

fn enter_alternate_raw_or_die() -> AlternateScreen<RawTerminal<Stdout>> {
    let raw = match stdout().into_raw_mode() {
        Ok(raw) => raw,
        Err(_) => {
            eprintln!("Failed to enter raw mode");
            std::process::exit(1);
        }
    };

    match raw.into_alternate_screen() {
        Ok(term) => term,
        Err(_) => {
            eprintln!("Failed to enter alternate screen");
            std::process::exit(1);
        }
    }
}
