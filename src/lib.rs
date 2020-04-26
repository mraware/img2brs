mod utils;
mod convert;

use wasm_bindgen::prelude::*;
// use wasm_bindgen::JsCast;
// use wasm_bindgen_futures::JsFuture;
use js_sys::ArrayBuffer;
use js_sys::Uint8Array;
use js_sys::Array;
use web_sys::File;

use convert::structs::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32_array(a: &[u8]);
}

#[wasm_bindgen]
pub fn img2brs(image_str: ArrayBuffer, name: &str, options: ImgOptions) -> Result<File, JsValue> {
    utils::set_panic_hook();

    let image_uint8 = Uint8Array::new(&JsValue::from(&image_str));
    let mut image_uint8_vec: Vec<u8> = vec![0; image_uint8.length() as usize];
    image_uint8.copy_to(&mut image_uint8_vec);

    let result = image::load_from_memory(&image_uint8_vec);


    let img = match result {
        Ok(img) => img,
        Err(error) => {
            panic!("Problem opening image {:?}", error);
        }
    };

  
    let mut brs_vec: Vec<u8> = vec![];

    convert::convert(&img, &mut brs_vec, options);

    let uint_array = Uint8Array::new(&JsValue::from_serde(&brs_vec).unwrap());
    let array_buffer = uint_array.buffer();
    let array = Array::new();
    array.push(&JsValue::from(&array_buffer));
    let file = File::new_with_buffer_source_sequence(&array, name).unwrap();

    return Ok(file);
}