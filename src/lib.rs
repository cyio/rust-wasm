use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn increase_brightness(data: &mut [u8], width: u32, height: u32, amount: u8) {
    for pixel in data.chunks_mut(4) {
        for color in &mut pixel[0..3] {
            *color = color.saturating_add(amount);
        }
    }
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    Ok(())
}