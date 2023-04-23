#![allow(clippy::cast_possible_truncation)]
use crate::rain::{Screen, BRIGHTEST, INVISIBLE};
use itertools::Itertools;
use ncurses::{
    attroff, attron, curs_set, endwin, getch, getmaxyx, init_color, init_pair, initscr, mvaddstr,
    nodelay, noecho, raw, refresh, setlocale, start_color, stdscr, LcCategory, COLOR_PAIR,
    CURSOR_VISIBILITY,
};

const MUL: f32 = 0.65;
const COLOR_MAX: i16 = 1000;
const INTENSITY: [i16; BRIGHTEST as usize + 1] = [1, 1, 2, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 4, 7];
const WHITESPACE: u32 = ' ' as u32;

pub fn init(color: Option<(i16, i16, i16)>, background: Option<(i16, i16, i16)>) -> (usize, usize) {
    setlocale(LcCategory::all, "en_US.UTF-8");
    let w = initscr();
    noecho();
    nodelay(w, true);
    raw();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    start_color();
    ncurses::use_default_colors();

    //let (rf, gf, bf) = match color {
    //    Some(rgb) => (
    //        (MUL * f32::from(rgb.0)) as i16,
    //        (MUL * f32::from(rgb.1)) as i16,
    //        (MUL * f32::from(rgb.2)) as i16,
    //    ),
    //    None => (0, 640 / 6, 0),
    //};
    //
    let (rf, gf, bf) = color.map_or((0, 640 / 6, 0), |rgb| {
        (
            (MUL * f32::from(rgb.0)) as i16,
            (MUL * f32::from(rgb.1)) as i16,
            (MUL * f32::from(rgb.2)) as i16,
        )
    });

    //let (rb, gb, bb) = match background {
    //    Some(rgb) => (
    //        (MUL * f32::from(rgb.0)) as i16,
    //        (MUL * f32::from(rgb.1)) as i16,
    //        (MUL * f32::from(rgb.2)) as i16,
    //    ),
    //    None => (0, 0, 0),
    //};

    let (rb, gb, bb) = background.map_or((0, 0, 0), |rgb| {
        (
            (MUL * f32::from(rgb.0)) as i16,
            (MUL * f32::from(rgb.1)) as i16,
            (MUL * f32::from(rgb.2)) as i16,
        )
    });
    init_pair(1, -1, -1);

    for i in 1..8 {
        init_pair(i, i + 1, -1);
    }

    for i in 1..7 {
        init_color(
            i,
            i * rf + (7 - i) * rb,
            i * gf + (7 - i) * gb,
            i * bf + (7 - i) * bb,
        );
    }

    init_color(8, COLOR_MAX, COLOR_MAX, COLOR_MAX);

    get_xy()
}

pub fn get_xy() -> (usize, usize) {
    let (mut height, mut width) = (0, 0);
    getmaxyx(stdscr(), &mut height, &mut width);

    (height as usize, width as usize)
}

pub fn show(s: &Screen) {
    for (j, i) in (0..s.max_y).cartesian_product(0..s.max_x) {
        unsafe {
            let cell = *s.s.get_unchecked(j).get_unchecked(i);

            if cell.b <= INVISIBLE {
                continue;
            }

            let b = cell.b as usize;
            let ch_idx = if b == 0 { WHITESPACE } else { cell.c };
            let ch = char::from_u32(ch_idx).unwrap_or('□');

            let pair = *INTENSITY.get_unchecked(b);

            attron(COLOR_PAIR(pair));
            mvaddstr(j as i32, i as i32, format!("{ch}").as_ref());

            attroff(COLOR_PAIR(pair));
        }
    }
    refresh();
}

pub fn term() -> bool {
    getch() == 3
}

pub fn finish() -> Result<(), ()> {
    match endwin() {
        0 => Ok(()),
        1 => Err(()),
        _ => panic!("Wrong return code"),
    }
}
