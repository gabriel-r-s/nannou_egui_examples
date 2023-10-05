use nannou::prelude::*;
use nannou::state::{mouse, Mouse};
use nannou_egui::{egui, Egui};

struct Model {
    egui: Egui,
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
        egui,
    }
}

fn raw_window_event(_app: &App, model: &mut Model, raw_event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(raw_event);
}

fn update(_app: &App, model: &mut Model, update: Update) {
    model.egui.set_elapsed_time(update.since_start);
    let ctx = model.egui.begin_frame();
    egui::Window::new("Settings").show(&ctx, |ui| {});
}

fn event(app: &App, model: &mut Model, window_event: WindowEvent) {
    match window_event {
        _ => {}
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.to_frame(app, &frame).expect("Video error");
    model.egui.draw_to_frame(&frame).expect("Video error");
}

fn main() {
    nannou::app(model).update(update).run();
}
