use macroquad::prelude::*;

#[derive(Clone)]
struct Point {
    x: f32,
    y: f32,
}

// Compute one Chaikin subdivision step, preserving endpoints
fn chaikin(points: &Vec<Point>) -> Vec<Point> {
    let n = points.len();
    if n < 2 {
        return points.clone();
    }
    let mut new_pts = Vec::new();
    // keep first
    new_pts.push(points[0].clone());
    for i in 0..n - 1 {
        let p0 = &points[i];
        let p1 = &points[i + 1];
        // Q and R points
        new_pts.push(Point {
            x: 0.75 * p0.x + 0.25 * p1.x,
            y: 0.75 * p0.y + 0.25 * p1.y,
        });
        new_pts.push(Point {
            x: 0.25 * p0.x + 0.75 * p1.x,
            y: 0.25 * p0.y + 0.75 * p1.y,
        });
    }
    // keep last
    new_pts.push(points[n - 1].clone());
    new_pts
}

#[macroquad::main("Chaikin's Algorithm")]
async fn main() {
    let mut control_points: Vec<Point> = Vec::new();
    let mut steps: Vec<Vec<Point>> = Vec::new();
    let mut animating = false;
    let mut current_step = 0;
    let mut timer = 0f32;
    const STEP_TIME: f32 = 0.5;
    let mut dragging_idx: Option<usize> = None;
    // optional warning message (text, remaining time)
    let mut warning: Option<(String, f32)> = None;
    loop {
        clear_background(WHITE);

        // display warning message if active
        if let Some((msg, time_left)) = &mut warning {
            draw_text(msg, 20.0, 30.0, 30.0, RED);
            *time_left -= get_frame_time();
            if *time_left <= 0.0 {
                warning = None;
            }
        }
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
                let mut pts = control_points.clone();
                // first step: original control points connected
                steps.push(pts.clone());
                // subsequent Chaikin subdivision steps
                for _ in 0..7 {
                    pts = chaikin(&pts);
                    steps.push(pts.clone());
                }
                animating = control_points.len() > 2;
                current_step = 0;
                timer = 0.0;
            } else {
                // not enough points -> warn user
                warning = Some(("Add at least two points before pressing Enter".to_string(), 2.0));
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
