use nannou::prelude::*;
use nannou::state::{mouse, Mouse};
use nannou::winit::event::WindowEvent as RawEvent;
use nannou_egui::{egui, Egui};

const _INTERVAL: std::time::Duration = std::time::Duration::from_micros(1_000_000_000 / 60);

struct Settings {
    bg_color: [f32; 3],
    line_color: [f32; 3],
    _string: String,
    line_draw: bool,
}

struct Model {
    focused: Option<(Point2, Point2)>,
    lines: Vec<(Point2, Point2)>,
    egui: Egui,
    settings: Settings,
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
        focused: None,
        lines: Vec::new(),
        egui,
        settings: Settings {
            bg_color: [0.0, 0.0, 0.0],
            line_color: [1.0, 1.0, 1.0],
            _string: String::new(),
            line_draw: false,
        }
    }
}

fn raw_window_event(_app: &App, model: &mut Model, event: &RawEvent) {
    if model.settings.line_draw {
        return;
    }
    model.egui.handle_raw_event(event);
}

fn update(_app: &App, model: &mut Model, update: Update) {
    model.egui.set_elapsed_time(update.since_start);
    let ctx = model.egui.begin_frame();
    egui::Window::new("Settings").show(&ctx, |ui| {
        if model.settings.line_draw {
            ui.label("Press ENTER to finish");
        } else {
            ui.label("background color");
            ui.add(egui::Slider::new(&mut model.settings.bg_color[0], 0.0..=1.0));
            ui.add(egui::Slider::new(&mut model.settings.bg_color[1], 0.0..=1.0));
            ui.add(egui::Slider::new(&mut model.settings.bg_color[2], 0.0..=1.0));
            ui.label("line color");
            ui.add(egui::Slider::new(&mut model.settings.line_color[0], 0.0..=1.0));
            ui.add(egui::Slider::new(&mut model.settings.line_color[1], 0.0..=1.0));
            ui.add(egui::Slider::new(&mut model.settings.line_color[2], 0.0..=1.0));
            ui.label("mode");
            if ui.button("line drawing").clicked() {
                model.settings.line_draw = true;
            };
            ui.label("Esc - Quit");
            ui.label("F11 - Toggle Fullscreen");
        }
    });
}

fn event(app: &App, model: &mut Model, window_event: WindowEvent) {
    if !model.settings.line_draw {
        return;
    }
    match window_event {
        MousePressed(mouse::Button::Left) => {
            let Mouse { x, y, ..} = app.mouse;
            model.focused = Some((Point2::new(x, y), Point2::new(x, y)));
        }
        MouseReleased(mouse::Button::Left) => {
            if let Some(line) = model.focused.take() {
                model.lines.push(line);
            }
        }
        MouseMoved(pos) => {
            if let Some((_start, end)) = model.focused.as_mut() {
                *end = pos;
            }
        }
        KeyPressed(Key::Return | Key::NumpadEnter) => {
            model.settings.line_draw = false;
        }
        _ => {}
    }
}


fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let [r, g, b] = model.settings.bg_color;
    draw.background().rgb(r, g, b);
    let [r, g, b] = model.settings.line_color;
    for &(start, end) in &model.lines {
        draw.line().rgb(r, g, b).points(start, end);
    }
    if let Some((start, end)) = model.focused {
        let scale = |s: f32| s*0.33;  
        draw.line().rgb(scale(r), scale(g), scale(b)).points(start, end);
    }
    draw.to_frame(app, &frame).expect("Video error");
    model.egui.draw_to_frame(&frame).expect("Video error");
}

fn main() {
    nannou::app(model).update(update).run();
}
