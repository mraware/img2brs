pub mod lists;
use std::io::Write;

use std::path::Path;
use std::fs::File;

use image::{
    GenericImageView,
    DynamicImage,
    Rgba
};

use brs::{
    Brick,
    Color,
    ColorMode,
    WriteData,
    User,
    uuid::Uuid,
    chrono::offset::Utc
};

pub fn convert_path(path: &String, size: (u32, u32, u32), asset_name_index: u32, vertical: bool, material_index: u32) {
  let img = get_image(path);

  let path_parse = Path::new(path);
  let file_name: String = match path_parse.file_stem() {
      Some(name) => match name.to_str() {
          Some(success) => String::from(success),
          _ => panic!("I miss null")
      },
      _ => panic!("No file name")
  };
  let mut folder_path: String = String::from(path_parse.parent().unwrap().to_str().unwrap());
  folder_path.push_str("/");
  let file_path_string = format!("{}{}{}", folder_path, file_name, ".brs");
  let file_path = Path::new(&file_path_string);

  let mut file = File::create(file_path).unwrap();

  convert(&img, &mut file, size, asset_name_index, vertical, material_index);
}

pub fn convert<T: Write>(img: &DynamicImage, writer: &mut T, size: (u32, u32, u32), asset_name_index: u32, vertical: bool, material_index: u32) {
    let dim = img.dimensions();
    let mut bricks: Vec<Brick> = Vec::with_capacity((dim.0 * dim.1) as usize);

    for x in 0..dim.0 {
        for y in 0..dim.1 {
            let px = img.get_pixel(x, y);
            if px[3] > 0 && !(material_index == 2 && px[3] < 100) {
                let brick = pixel_to_brick(x, y, size, vertical, px, dim, asset_name_index, material_index);
                bricks.push(brick);
            }
        }
    }

    write_brs(writer, bricks);
}

fn get_image(path: &str) -> DynamicImage {
    let result = image::open(path);

    let img = match result {
        Ok(img) => img,
        Err(error) => {
            panic!("Problem opening image {:?}", error);
        }
    };

    return img;
}

fn pixel_to_brick(x: u32, y: u32, size: (u32, u32, u32), vertical: bool, px: Rgba<u8>, dim: (u32, u32), asset_name_index: u32, material_index: u32) -> Brick {
    let color: Color = Color::from_rgba(px[0], px[1], px[2], px[3]);

    let x_adj = if vertical { (x * size.1 * 2 + size.1) as i32 } else { (x * size.0 * 2 + size.0) as i32  };
    let y_adj = if vertical { (y * size.0 * 2 + size.0) as i32 } else { (y * size.1 * 2 + size.1) as i32 as i32 };

    let position = if vertical { (x_adj, size.2 as i32, -y_adj+(dim.1 as i32 * size.0 as i32 * 2)) } else { (x_adj, y_adj, size.2 as i32) };

    let color_mode: ColorMode = ColorMode::Custom(color);

    let brick = Brick {
        asset_name_index: asset_name_index,
        size,
        position,
        direction: if vertical { brs::Direction::YPositive } else { brs::Direction::ZPositive },
        rotation: if vertical { brs::Rotation::Deg0 } else { brs::Rotation::Deg0 } ,
        collision: true,
        visibility: true,
        material_index: material_index,
        color: color_mode,
        owner_index: 0
    };

    return brick;
}

fn write_brs<T: Write>(mut writer: &mut T, bricks: Vec<Brick>) {
    let author = User {
        id: Uuid::parse_str("ffffffff-ffff-ffff-ffff-ffffffffffff").unwrap(),
        name: String::from("PUBLIC")
    };

    let brick_owners = vec![
        User {
            id: Uuid::parse_str("ffffffff-ffff-ffff-ffff-ffffffffffff").unwrap(),
            name: String::from("PUBLIC")
        }
    ];

    let write_data = WriteData {
        map: String::from("Plate"),
        author,
        description: String::from("Generated with img2brs."),
        save_time: Utc::now(),
        mods: vec![],
        brick_assets: lists::get_brick_assets(),
        colors: vec![],
        materials: lists::get_materials(),
        brick_owners,
        bricks
    };

    brs::write_save(&mut writer, &write_data).unwrap();
}