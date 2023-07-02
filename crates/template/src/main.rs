use async_std::task::block_on;

mod app;
use app::run_app;
mod sketch;
use sketch::Model;

fn main() {
    let model = Model {};

    block_on(async {
        run_app(model).await;
    });
}
