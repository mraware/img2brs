use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ImgOptions {
  pub size_x: u32,
  pub size_y: u32,
  pub size_z: u32,
  pub asset_name_index: u32,
  pub vertical: bool,
  pub material_index: u32
}

#[wasm_bindgen]
impl ImgOptions {
  pub fn new(size_x: u32, size_y: u32, size_z: u32, asset_name_index: u32, vertical: bool, material_index: u32) -> ImgOptions {
    ImgOptions { 
      size_x: size_x,
      size_y: size_y,
      size_z: size_z,
      asset_name_index: asset_name_index,
      vertical: vertical,
      material_index: material_index
    }
  }
}
