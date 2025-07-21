use raylib::prelude::*;

const WIDTH: usize = 100;
const HEIGHT: usize = 100;

fn get_color(alive: bool) -> Color {
    if alive {
        Color::WHITE
    } else {
        Color::new(10, 10, 40, 255)
    }
}

fn count_neighbors(grid: &[[bool; WIDTH]; HEIGHT], x: usize, y: usize) -> u8 {
    let mut count = 0;
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx >= 0 && nx < WIDTH as isize && ny >= 0 && ny < HEIGHT as isize {
                if grid[ny as usize][nx as usize] {
                    count += 1;
                }
            }
        }
    }
    count
}

fn place_glider(grid: &mut [[bool; WIDTH]; HEIGHT], x: usize, y: usize) {
    grid[y][x + 1] = true;
    grid[y + 1][x + 2] = true;
    grid[y + 2][x] = true;
    grid[y + 2][x + 1] = true;
    grid[y + 2][x + 2] = true;
}

fn place_lwss(grid: &mut [[bool; WIDTH]; HEIGHT], x: usize, y: usize) {
    grid[y][x + 1] = true;
    grid[y][x + 2] = true;
    grid[y][x + 3] = true;
    grid[y + 1][x] = true;
    grid[y + 1][x + 3] = true;
    grid[y + 2][x + 3] = true;
    grid[y + 3][x] = true;
    grid[y + 3][x + 2] = true;
}

fn place_blinker(grid: &mut [[bool; WIDTH]; HEIGHT], x: usize, y: usize) {
    grid[y][x] = true;
    grid[y][x + 1] = true;
    grid[y][x + 2] = true;
}

fn place_beacon(grid: &mut [[bool; WIDTH]; HEIGHT], x: usize, y: usize) {
    grid[y][x] = true;
    grid[y][x + 1] = true;
    grid[y + 1][x] = true;
    grid[y + 1][x + 1] = true;

    grid[y + 2][x + 2] = true;
    grid[y + 2][x + 3] = true;
    grid[y + 3][x + 2] = true;
    grid[y + 3][x + 3] = true;
}

fn place_block(grid: &mut [[bool; WIDTH]; HEIGHT], x: usize, y: usize) {
    grid[y][x] = true;
    grid[y][x + 1] = true;
    grid[y + 1][x] = true;
    grid[y + 1][x + 1] = true;
}

fn place_toad(grid: &mut [[bool; WIDTH]; HEIGHT], x: usize, y: usize) {
    grid[y][x + 1] = true;
    grid[y][x + 2] = true;
    grid[y][x + 3] = true;
    grid[y + 1][x] = true;
    grid[y + 1][x + 1] = true;
    grid[y + 1][x + 2] = true;
}

fn place_loaf(grid: &mut [[bool; WIDTH]; HEIGHT], x: usize, y: usize) {
    grid[y][x + 1] = true;
    grid[y][x + 2] = true;
    grid[y + 1][x] = true;
    grid[y + 1][x + 3] = true;
    grid[y + 2][x + 1] = true;
    grid[y + 2][x + 3] = true;
    grid[y + 3][x + 2] = true;
}

fn place_boat(grid: &mut [[bool; WIDTH]; HEIGHT], x: usize, y: usize) {
    grid[y][x] = true;
    grid[y][x + 1] = true;
    grid[y + 1][x] = true;
    grid[y + 1][x + 2] = true;
    grid[y + 2][x + 1] = true;
}

fn place_tub(grid: &mut [[bool; WIDTH]; HEIGHT], x: usize, y: usize) {
    grid[y][x + 1] = true;
    grid[y + 1][x] = true;
    grid[y + 1][x + 2] = true;
    grid[y + 2][x + 1] = true;
}

fn place_pulsar(grid: &mut [[bool; WIDTH]; HEIGHT], x: usize, y: usize) {
    for dx in [2, 3, 4, 8, 9, 10] {
        grid[y][x + dx] = true;
        grid[y + 5][x + dx] = true;
        grid[y + 7][x + dx] = true;
        grid[y + 12][x + dx] = true;
    }
    for dy in [2, 3, 4, 8, 9, 10] {
        grid[y + dy][x] = true;
        grid[y + dy][x + 5] = true;
        grid[y + dy][x + 7] = true;
        grid[y + dy][x + 12] = true;
    }
}

fn place_butterfly(grid: &mut [[bool; WIDTH]; HEIGHT], x: usize, y: usize) {
    place_block(grid, x + 5, y + 5);
    place_blinker(grid, x + 5, y + 8);

    place_loaf(grid, x, y);
    place_loaf(grid, x + 10, y);

    place_toad(grid, x, y + 10);
    place_toad(grid, x + 10, y + 10);
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(600, 600)
        .title("Conway's Game of Life")
        .build();

    rl.set_target_fps(10);

    let mut current = [[false; WIDTH]; HEIGHT];
    let mut next = [[false; WIDTH]; HEIGHT];

    for i in 0..3 {
        for j in 0..2 {
            let x = 10 + i * 30;
            let y = 10 + j * 30;
            place_pulsar(&mut current, x, y);
        }
    }

    place_glider(&mut current, 5, 5);
    place_beacon(&mut current, 70, 5);
    place_boat(&mut current, 5, 65);
    place_tub(&mut current, 40, 65);
    place_loaf(&mut current, 75, 65);
    place_lwss(&mut current, 40, 5);

    for i in 0..4 {
        let x = 10 + i * 20;
        let y = 85;
        place_blinker(&mut current, x, y);
    }

    while !rl.window_should_close() {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let neighbors = count_neighbors(&current, x, y);
                let alive = current[y][x];
                next[y][x] = match (alive, neighbors) {
                    (true, 2) | (true, 3) => true,
                    (false, 3) => true,
                    _ => false,
                };
            }
        }

        std::mem::swap(&mut current, &mut next);

        let mut d = rl.begin_drawing(&thread);
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let color = get_color(current[y][x]);
                d.draw_rectangle(x as i32 * 6, y as i32 * 6, 6, 6, color);
            }
        }
    }
}