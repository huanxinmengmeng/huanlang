// Copyright (c) 2026 幻心梦梦（huanxinmengmeng）
// icon_generator.rs - 将 logo.jpg 转换为 icon.ico 的工具
//
// 使用方法: rustc --edition 2021 icon_generator.rs && ./icon_generator

use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

fn main() {
    let jpg_path = Path::new("images").join("logo.jpg");
    let ico_path = Path::new("resources").join("icon.ico");

    println!("Converting {} to {}", jpg_path.display(), ico_path.display());

    if !jpg_path.exists() {
        eprintln!("Error: {} not found!", jpg_path.display());
        std::process::exit(1);
    }

    // 确保 resources 目录存在
    if let Some(parent) = ico_path.parent() {
        std::fs::create_dir_all(parent).expect("Failed to create resources directory");
    }

    match convert_jpg_to_ico(&jpg_path, &ico_path) {
        Ok(_) => println!("Successfully converted to {}", ico_path.display()),
        Err(e) => {
            eprintln!("Error converting image: {}", e);
            std::process::exit(1);
        }
    }
}

fn convert_jpg_to_ico(jpg_path: &Path, ico_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // 读取 JPEG 文件
    let file = File::open(jpg_path)?;
    let mut bufreader = BufReader::new(file);
    let mut jpg_data = Vec::new();
    bufreader.read_to_end(&mut jpg_data)?;

    // 解码 JPEG
    let img = image::load_from_memory_with_format(&jpg_data, image::ImageFormat::Jpeg)?;

    // 调整为多个常用尺寸
    let sizes = [256, 128, 64, 48, 32, 16];
    let mut icons: Vec<IconData> = Vec::new();

    for size in sizes.iter() {
        let resized = img.resize_exact(*size as u32, *size as u32, image::imageops::FilterType::Lanczos3);
        let rgba = resized.to_rgba8();

        // 创建 PNG 数据
        let mut png_data = Vec::new();
        let mut cursor = std::io::Cursor::new(&mut png_data);
        rgba.write_to(&mut cursor, image::ImageFormat::Png)?;

        icons.push(IconData {
            width: *size as u8,
            height: *size as u8,
            png_data,
        });
    }

    // 写入 ICO 文件
    let file = File::create(ico_path)?;
    let mut writer = BufWriter::new(file);
    write_ico_file(&mut writer, &icons)?;

    Ok(())
}

struct IconData {
    width: u8,
    height: u8,
    png_data: Vec<u8>,
}

fn write_ico_file<W: Write>(writer: &mut W, icons: &[IconData]) -> Result<(), Box<dyn std::error::Error>> {
    // ICO 文件头
    // 0-1: 保留 (0)
    writer.write_all(&[0, 0])?;
    // 2-3: 类型 (1 = ICO)
    writer.write_all(&[1, 0])?;
    // 4-5: 图片数量
    let num_images = icons.len() as u16;
    writer.write_all(&num_images.to_le_bytes())?;

    // 计算数据偏移量
    // 头: 6 bytes
    // 目录项: 16 bytes * num_images
    let data_offset = 6 + (16 * icons.len() as u32);

    // 写入目录项和数据
    let mut current_offset = data_offset;

    for icon in icons {
        // 目录项 (16 bytes)
        // 0: 宽度 (0 = 256)
        writer.write_all(&[if icon.width == 0 { 0 } else { icon.width }])?;
        // 1: 高度 (0 = 256)
        writer.write_all(&[if icon.height == 0 { 0 } else { icon.height }])?;
        // 2: 颜色调色板 (0 = 无调色板)
        writer.write_all(&[0])?;
        // 3: 保留 (0)
        writer.write_all(&[0])?;
        // 4-5: 颜色平面 (1)
        writer.write_all(&[1, 0])?;
        // 6-7: 每像素位数 (32)
        writer.write_all(&[32, 0])?;
        // 8-11: 数据大小
        let data_size = icon.png_data.len() as u32;
        writer.write_all(&data_size.to_le_bytes())?;
        // 12-15: 数据偏移量
        writer.write_all(&current_offset.to_le_bytes())?;

        current_offset += data_size;
    }

    // 写入实际的 PNG 数据
    for icon in icons {
        writer.write_all(&icon.png_data)?;
    }

    Ok(())
}
