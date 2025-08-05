use macroquad::prelude::*;

#[derive(Clone)]
struct Point {
    x: f32,
    y: f32,
}

#[macroquad::main("Chaikin's Algorithm")]
async fn main() {
    let mut control_points: Vec<Point> = Vec::new();
    let mut steps: Vec<Vec<Point>> = Vec::new();
    let mut animating = false;
    let mut dragging_idx: Option<usize> = None;
    // optional warning message (text, remaining time)
    let mut warning: Option<(String, f32)> = None;
    loop {
        clear_background(WHITE);

        // handle mouse press: start dragging or add point (add only when not animating)
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            // check if clicking near existing point -> start dragging
            if let Some((i, _)) = control_points
                .iter()
                .enumerate()
                .find(|(_, p)| (p.x - mx).powi(2) + (p.y - my).powi(2) < 100.0)
            {
                dragging_idx = Some(i);
            } else if !animating {
                // add new point only when not animating
                control_points.push(Point { x: mx, y: my });
            }
        }

        // update dragged point position
        if is_mouse_button_down(MouseButton::Left) {
            if let Some(i) = dragging_idx {
                let (mx, my) = mouse_position();
                control_points[i] = Point { x: mx, y: my };
            }
        }

        // start animation on Enter
        if is_key_pressed(KeyCode::Enter) {
            // stop any dragging before animation
            dragging_idx = None;
            if control_points.len() >= 2 {
                steps.clear();
                // first step: original control points connected
                // subsequent Chaikin subdivision steps
                for _ in 0..7 {
                 // ToDo add Chaikin Algo
                }
                animating = control_points.len() > 2;
                current_step = 0;
                timer = 0.0;
            } else {
                // not enough points -> warn user
                warning = Some((
                    "Add at least two points before pressing Enter".to_string(),
                    2.0,
                ));
            }
        }

        for p in &control_points {
            draw_circle(p.x, p.y, 3.5, BLACK);
        }

        // exit on Escape
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await
    }
}
