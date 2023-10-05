use nannou::prelude::*;
use nannou::state::Mouse;
use nannou_egui::{egui, Egui};

#[derive(Clone, Copy)]
enum State {
    Gui,
    PlacePoints,
    MovePoints(Option<usize>),
    ConnectPoints(Option<usize>),
}

const MAX_SELECT_DISTANCE: f32 = 20.0;

struct Model {
    points: Vec<Point2>,
    lines: Vec<(usize, usize)>,
    state: State,
    egui: Egui,
}

impl Model {
    fn find_nearest_point(&self, point: Point2, max_dist: f32) -> Option<usize> {
        self.points.iter()
            .enumerate()
            .map(|(i, p)| (i, p, point.distance(*p)))
            .filter(|(_, _, d)| *d <= max_dist)
            .min_by(|l, r| if l.2 < r.2 { std::cmp::Ordering::Less } else { std::cmp::Ordering::Greater })
            .map(|(i, _, _)| i)
    }
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .size(800, 600)
        .view(view)
        .raw_event(raw_window_event)
        .event(event)
        .build()
        .expect("Error creating window");
    
    let window = app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);

    Model {
        points: Vec::new(),
        lines: Vec::new(),
        state: State::Gui,
        egui,
    }
}

fn raw_window_event(_app: &App, model: &mut Model, raw_event: &nannou::winit::event::WindowEvent) {
    if let State::Gui = model.state {
        model.egui.handle_raw_event(raw_event);
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
    model.egui.set_elapsed_time(update.since_start);
    let ctx = model.egui.begin_frame();
    egui::Window::new("Menu").show(&ctx, |ui| {
        if let State::Gui = model.state {
            if ui.button("Place Points").clicked() {
                model.state = State::PlacePoints;
            }
            if ui.button("Move Points").clicked() {
                model.state = State::MovePoints(None);
            }
            if ui.button("Connect Points").clicked() {
                model.state = State::ConnectPoints(None);
            }
            if ui.button("Fullscreen (F11)").clicked() {
                let window = app.main_window();
                window.set_fullscreen(!window.is_fullscreen());
            }
            if ui.button("Quit (Esc)").clicked() {
                app.quit();
            }
        } else {
            ui.label("Press ENTER to finish");
        }
    });
}

fn event(app: &App, model: &mut Model, window_event: WindowEvent) {
    let state = model.state;
    match (state, window_event) {
        (State::PlacePoints, MousePressed(MouseButton::Left)) => {
            let Mouse { x, y, .. } = app.mouse;
            model.points.push(Point2::new(x, y));
        }

        (State::MovePoints(None), MousePressed(MouseButton::Left)) => {
            let Mouse { x, y, .. } = app.mouse;
            if let Some(selected) = model.find_nearest_point(Point2::new(x, y), MAX_SELECT_DISTANCE) {
                model.state = State::MovePoints(Some(selected));
            }
        }
        (State::MovePoints(Some(selected)), MouseMoved(new_pos)) => {
            model.points[selected] = new_pos;
        }
        (State::MovePoints(Some(_)), MouseReleased(MouseButton::Left)) => {
            model.state = State::MovePoints(None);
        }

        (State::ConnectPoints(None), MousePressed(MouseButton::Left)) => {
            let Mouse { x, y, ..} = app.mouse;
            if let Some(point) = model.find_nearest_point(Point2::new(x, y), MAX_SELECT_DISTANCE) {
                model.state = State::ConnectPoints(Some(point));
            }
        }
        (State::ConnectPoints(Some(start)), MousePressed(MouseButton::Left)) => {
            let Mouse { x, y, ..} = app.mouse;
            if let Some(end) = model.find_nearest_point(Point2::new(x, y), MAX_SELECT_DISTANCE) {
                model.lines.push((start, end));
                model.state = State::ConnectPoints(None);
            }
        }
        (_, KeyPressed(Key::Return)) => {
            model.state = State::Gui;
        }
        _ => {}
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(PURPLE);
    for &point in &model.points {
        draw.ellipse().color(WHITE).radius(2.0).xy(point);
    }
    for &(start, end) in &model.lines {
        let start = model.points[start];
        let end = model.points[end];
        draw.line().points(start, end).color(WHITE);
    }
    match model.state {
        State::ConnectPoints(Some(start)) => {
            let start = model.points[start];
            let Mouse { x, y, .. } = app.mouse;
            let end = Point2::new(x, y);
            if let Some(point) = model.find_nearest_point(end, MAX_SELECT_DISTANCE) {
                let end = model.points[point];
                draw.line().points(start, end).color(GRAY);
                let [x, y] = end.to_array();
                let square_size = 5.0;
                let points = [
                    Point2::new(x - square_size, y - square_size),
                    Point2::new(x - square_size, y + square_size),
                    Point2::new(x + square_size, y + square_size),
                    Point2::new(x + square_size, y - square_size),
                    Point2::new(x - square_size, y - square_size),
                ];
                draw.polyline().color(GRAY).points(points);
            } else {
                draw.line().points(start, end).color(GRAY);
            }
        }
        State::PlacePoints => {
            let Mouse { x, y, .. } = app.mouse;
            draw.ellipse().color(GRAY).radius(2.0).xy(Point2::new(x, y));
        }
        State::MovePoints(None) | State::ConnectPoints(_) => {
            let Mouse { x, y, .. } = app.mouse;
            if let Some(point) = model.find_nearest_point(
                Point2::new(x, y), MAX_SELECT_DISTANCE) {
                let [x, y] =  model.points[point].to_array();
                let square_size = 5.0;
                let points = [
                    Point2::new(x - square_size, y - square_size),
                    Point2::new(x - square_size, y + square_size),
                    Point2::new(x + square_size, y + square_size),
                    Point2::new(x + square_size, y - square_size),
                    Point2::new(x - square_size, y - square_size),
                ];
                draw.polyline().color(GRAY).points(points);
            }
        }
        State::Gui | State::MovePoints(_) => {}
    }
    draw.to_frame(app, &frame).expect("Video error");
    model.egui.draw_to_frame(&frame).expect("Video error");
}

fn main() {
    nannou::app(model).update(update).run();
}
