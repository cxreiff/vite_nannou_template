use nannou::{prelude::*, wgpu::Backends};
use std::cell::RefCell;

use crate::sketch::{create_window, update, Model};

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
