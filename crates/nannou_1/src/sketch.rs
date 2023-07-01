use nannou::prelude::*;

use nannou::wgpu::{Backends, DeviceDescriptor, Limits};
use std::cell::RefCell;

pub struct Model;

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
	let win = app.window_rect();
	let t = app.time;
	let draw = app.draw();

	draw.background().color(PURPLE);

	let radius = win.w().min(win.h()) * 0.25;
	let n_points = 12;
	let points_colored = (0..n_points).map(|i| {
		let fract = i as f32 / n_points as f32;
		let phase = fract;
		let x = radius * (TAU * phase).cos();
		let y = radius * (TAU * phase).sin();
		let r = fract;
		let g = 1.0 - fract;
		let b = (0.5 + fract) % 1.0;
		(pt2(x, y), rgb(r, g, b))
	});
	draw.polygon()
		.x(0.0)
		.rotate(t * 0.2)
		.points_colored(points_colored);

	draw.to_frame(app, &frame).unwrap();
}

pub async fn run_app(model: Model) {
	thread_local!(static MODEL: RefCell<Option<Model>> = Default::default());

	MODEL.with(|m| m.borrow_mut().replace(model));

	app::Builder::new_async(|app| {
		Box::new(async move {
			create_window(app).await;
			MODEL.with(|m| m.borrow_mut().take().unwrap())
		})
	})
	.backends(Backends::PRIMARY | Backends::GL)
	.update(update)
	.run_async()
	.await;
}

async fn create_window(app: &App) {
	let device_desc = DeviceDescriptor {
		limits: Limits {
			max_texture_dimension_2d: 8192,
			..Limits::downlevel_webgl2_defaults()
		},
		..Default::default()
	};

	app.new_window()
		.device_descriptor(device_desc)
		.title("nannou sketch")
		.view(view)
		.build_async()
		.await
		.unwrap();
}
