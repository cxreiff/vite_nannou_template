use nannou::prelude::*;

use nannou::wgpu::{DeviceDescriptor, Limits};

pub struct Model;

pub fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let win = app.window_rect();
    let t = app.time;
    let draw = app.draw();

    draw.background().color(BLACK);

    let radius = win.w().min(win.h()) * 0.25;
    let n_points = 12;
    let points_colored = (0..n_points).map(|i| {
        let fract = i as f32 / n_points as f32;
        let phase = fract;
        let x = radius * (TAU * phase).cos();
        let y = radius * (TAU * phase).sin();
        let r = 1.0 - fract;
        let g = (0.5 + fract) % 1.0;
        let b = fract;
        (pt2(x, y), rgb(r, g, b))
    });
    draw.polygon()
        .x(0.0)
        .rotate(t * 0.2)
        .points_colored(points_colored);

    draw.to_frame(app, &frame).unwrap();
}

pub async fn create_window(app: &App) {
    app.new_window()
        .device_descriptor(DeviceDescriptor {
            limits: Limits {
                max_texture_dimension_2d: 8192,
                ..Limits::downlevel_webgl2_defaults()
            },
            ..Default::default()
        })
        .title("nannou sketch")
        .view(view)
        .build_async()
        .await
        .unwrap();
}
