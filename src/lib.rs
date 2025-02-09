#![feature(iter_array_chunks)]
pub mod bc7_unorm;

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, fs::File, io::BufReader, path::Path};

    use bitvec::{
        order::{Lsb0, Msb0},
        view::BitView,
    };
    use ddsfile::{Caps, DataFormat, Dds, FourCC};

    use crate::bc7_unorm::{
        decode_block_mode_0, decode_block_mode_1, decode_block_mode_2, decode_block_mode_3,
        decode_block_mode_4, RGB,
    };

    #[test]
    fn test_load_art() {
        let path = "/home/adam/poe_tools_data/extract/art/textures/pet/chicken/chickenns.dds";
        let file = File::open(Path::new(path)).unwrap();
        let mut reader = BufReader::new(file);

        let dds = Dds::read(&mut reader).unwrap();
        println!("{:?}", dds.header);

        assert_eq!(
            dds.header.spf.fourcc,
            Some(FourCC(FourCC::DX10)),
            "Only DX10 files supported"
        );

        assert_eq!(
            dds.header.caps & Caps::MIPMAP,
            Caps::MIPMAP,
            "Only mipmap files supported"
        );

        println!("{:?}, {:?}", dds.get_d3d_format(), dds.get_dxgi_format());
        let image_height = dds.get_height() as usize;
        let image_width = dds.get_width() as usize;
        println!(
            "{} {} {} {:?}",
            dds.get_height(),
            dds.get_width(),
            dds.get_depth(),
            dds.get_bits_per_pixel()
        );
        println!("{:?}", dds.get_num_mipmap_levels());

        let fm = dds.get_dxgi_format().unwrap();
        let block_size = fm.get_block_size().unwrap() as usize;
        let block_height = fm.get_pitch_height() as usize;
        println!(
            "{:?}, {:?} {:?}",
            fm.get_block_size(),
            fm.requires_extension(),
            fm.get_pitch_height()
        );

        for i in 0..dds.get_num_array_layers() {
            let data = dds.get_data(i).unwrap();
            println!("{} {}", i, data.len());

            let blocks = data
                .chunks_exact(block_size)
                .map(|block| block.view_bits::<Lsb0>())
                .collect::<Vec<_>>();
            println!("num blocks: {}", blocks.len());

            let modes = blocks.iter().map(|b| b.first_one().unwrap()).fold(
                HashMap::new(),
                |mut acc, mode| {
                    acc.entry(mode)
                        .and_modify(|count: &mut usize| *count += 1)
                        .or_default();

                    acc
                },
            );
            println!("modes: {:?}", modes);

            //decode_block_mode_4(blocks[0]);
            //decode_block_mode_2(blocks[60]);
            //panic!();

            let decoded = blocks
                .into_iter()
                .filter_map(|b| {
                    let decoded = match b.first_one().unwrap() {
                        0 => decode_block_mode_0(b),
                        1 => decode_block_mode_1(b),
                        2 => decode_block_mode_2(b),
                        3 => decode_block_mode_3(b),
                        4 => decode_block_mode_4(b),
                        _ => [RGB { r: 0, g: 0, b: 0 }; 16],
                    };
                    Some(decoded)
                })
                .collect::<Vec<_>>();
            println!("done {}", decoded.len());

            // Combine blocks into single image
            let num_blocks_y = image_height / block_height;
            let num_blocks_x = image_width / block_height;
            let num_blocks = num_blocks_y * num_blocks_x;
            let biggest_image = &decoded[..num_blocks];

            let mut img = image::RgbImage::new(image_width as u32, image_height as u32);
            biggest_image.iter().enumerate().for_each(|(i, block)| {
                let block_x = i % num_blocks_x;
                let block_y = i / num_blocks_x;

                block.iter().enumerate().for_each(|(j, pixel)| {
                    let x_inner = j % 4;
                    let y_inner = j / 4;
                    let y = (block_y * block_height) + y_inner;
                    let x = (block_x * block_height) + x_inner;
                    //println!("{} {} {} {}", i, j, y, x);

                    img.put_pixel(x as u32, y as u32, image::Rgb([pixel.r, pixel.g, pixel.b]));
                });
            });

            img.save("test.png").unwrap();
        }
    }
}
