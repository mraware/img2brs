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
    chrono::DateTime
};

pub fn convert(path: &String) {
    let img = get_image(path);
    let dim = img.dimensions();
    let size = (5, 5, 2);
    let vertical = false;
    let mut bricks: Vec<Brick> = vec![];


    for x in 0..dim.0 {
        for y in 0..dim.1 {
            let px = img.get_pixel(x, y);
            let brick = pixel_to_brick(x, y, size, vertical, px, dim);
            bricks.push(brick);
        }
    }

    write_brs(path, bricks);
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

fn pixel_to_brick(x: u32, y: u32, size: (u32, u32, u32), vertical: bool, px: Rgba<u8>, dim: (u32, u32)) -> Brick {
    let color: Color = Color::from_rgba(px[0], px[1], px[2], px[3]);

    let x_adj = (x * size.0 * 2 + size.0) as i32;
    let y_adj = (y * size.1 * 2 + size.1) as i32;

    let position = if vertical { (x_adj, size.2 as i32, -y_adj+(dim.1 as i32)) } else { (x_adj, y_adj, size.2 as i32) };

    let color_mode: ColorMode = ColorMode::Custom(color);

    let brick = Brick {
        asset_name_index: 1,
        size,
        position,
        direction: if vertical { brs::Direction::YPositive } else { brs::Direction::ZPositive },
        rotation: if vertical { brs::Rotation::Deg0 } else { brs::Rotation::Deg0 } ,
        collision: true,
        visibility: true,
        material_index: 0,
        color: color_mode,
        owner_index: 0
    };

    return brick;
}

fn write_brs(path: &String, bricks: Vec<Brick>) {
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

    let author = User {
        id: Uuid::parse_str("fa577b9e-f2be-493f-a30a-3789b02ba70b").unwrap(),
        name: String::from("Aware")
    };

    let brick_owners = vec![
        User {
            id: Uuid::parse_str("fa577b9e-f2be-493f-a30a-3789b02ba70b").unwrap(),
            name: String::from("Aware")
        },
        User {
            id: Uuid::parse_str("ffffffff-ffff-ffff-ffff-ffffffffffff").unwrap(),
            name: String::from("PUBLIC")
        }
    ];

    let brick_assets = vec![
        String::from("PB_DefaultBrick"),
        String::from("PB_DefaultTile")
    ];
    let materials = vec![
        String::from("BMC_Plastic"),
        String::from("BMC_Glow"),
        String::from("BMC_Metallic"),
        String::from("BMC_Hologram")
    ];

    let write_data = WriteData {
        map: String::from("Plate"),
        author,
        description: String::from("Generated with img2brs."),
        save_time: DateTime::from(std::time::SystemTime::now()),
        mods: vec![],
        brick_assets,
        colors: vec![],
        materials,
        brick_owners,
        bricks
    };

    let mut file = File::create(file_path).unwrap();

    brs::write_save(&mut file, &write_data).unwrap();
}