#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use wasm_bindgen::prelude::wasm_bindgen;

mod sketch;
use sketch::{run_app, Model};
use async_std::task::block_on;

#[wasm_bindgen]
pub async fn main_web() {
	#[cfg(debug_assertions)]
	console_error_panic_hook::set_once();

	let model = Model {};
	block_on(async {
		run_app(model).await;
	});
}
