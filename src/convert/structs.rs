use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ImgOptions {
  pub size_x: u32,
  pub size_y: u32,
  pub size_z: u32,
  pub asset_name_index: u32,
  pub vertical: bool,
  pub material_index: u32,
  pub remove_alpha: bool,
  pub offset_x: i32,
  pub offset_y: i32,
  pub offset_z: i32
}

#[wasm_bindgen]
impl ImgOptions {
  pub fn new(size_x: u32, size_y: u32, size_z: u32, asset_name_index: u32, vertical: bool, material_index: u32, remove_alpha: bool, offset_x: i32, offset_y: i32, offset_z: i32) -> ImgOptions {
    ImgOptions { 
      size_x: size_x,
      size_y: size_y,
      size_z: size_z,
      asset_name_index: asset_name_index,
      vertical: vertical,
      material_index: material_index,
      remove_alpha: remove_alpha,
      offset_x: offset_x,
      offset_y: offset_y,
      offset_z: offset_z
    }
  }
}
