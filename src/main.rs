use macroquad::prelude::*;
mod stimulus;

const ROWS: i16 = 12;
const COLUMNS: i16 = 12;

fn draw_focus(center: &stimulus::Point) {
    draw_circle(
        center.0,
        center.1,
        5.,
        LIGHTGRAY,
    );
}

fn draw_crosshairs(center: &stimulus::Point, length: &f32) {
    draw_line(
        center.0 - length/4.,
        center.1,
        center.0 + length/4.,
        center.1,
        3.,
        RED
    );

    draw_line(
        center.0,
        center.1 - length/4.,
        center.0,
        center.1 + length/4.,
        3.,
        RED
    );
}

#[macroquad::main("SWAP Visual Evoked Potential")]
async fn main() {

    let mut speed = 0.25;
    let mut last_update = get_time();
    let mut shifting = false;

    let window_size = screen_width().min(screen_height());
    let offset_x = (screen_width() - window_size) / 2. + 10.;
    let offset_y = (screen_height() - window_size) / 2. + 10.;
    let starting_point = (offset_x, offset_y);

    let width = (screen_height() - offset_y * 2.) / (ROWS as f32);
    let height = (screen_height() - offset_y * 2.) / (COLUMNS as f32);
    let patch_size = (width, height);
    let center = (
        (starting_point.0 + (COLUMNS as f32 + 2.) * (patch_size.0 / 2.)),
        (starting_point.1 + (ROWS as f32 + 2.) * (patch_size.1 / 2.))
    );
    let mut pattern = stimulus::Pattern{
        patches: Vec::<stimulus::Patch>::new(), 
        rows: ROWS, 
        columns: COLUMNS
    };
    pattern.init(&starting_point, &patch_size);

    loop {
        if shifting {
            clear_background(LIGHTGRAY);
            if get_time() - last_update >= speed {
                last_update = get_time();
                pattern.invert();
                draw_crosshairs(&center, &patch_size.0);
                //draw_focus(&center);
            }
            else {
                pattern.draw();
                draw_crosshairs(&center, &patch_size.0);
                //draw_focus(&center);
            }
            if is_key_down(KeyCode::Backspace) {
                shifting = false;
            }
            if is_key_down(KeyCode::Up) || is_key_down(KeyCode::Right) {
                speed = speed * 0.95;
            }
            if is_key_down(KeyCode::Down) || is_key_down(KeyCode::Left) {
                speed = speed / 0.95;
            }
        }
        else {
            clear_background(WHITE);
            let text = "Press [enter] to start sVEP.";
            let font_size = 30.;
            let text_size = measure_text(text, None, font_size as _, 1.0);

            draw_text(
                text,
                screen_width() / 2. - text_size.width / 2.,
                screen_height() / 2. + text_size.height / 2.,
                font_size,
                DARKGRAY,
            );

            if is_key_down(KeyCode::Enter) {
                shifting = true;
            }
        }
        next_frame().await;
    }
}