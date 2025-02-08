#![feature(iter_array_chunks)]
pub mod bc7_unorm;

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, fs::File, io::BufReader, path::Path};

    use bitvec::{order::Msb0, view::BitView};
    use ddsfile::{Caps, DataFormat, Dds, FourCC};

    use crate::bc7_unorm::{decode_block_mode_0, decode_block_mode_1};

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
        println!(
            "{} {} {} {:?}",
            dds.get_height(),
            dds.get_width(),
            dds.get_depth(),
            dds.get_bits_per_pixel()
        );
        println!("{:?}", dds.get_num_mipmap_levels());

        let fm = dds.get_dxgi_format().unwrap();
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
                .chunks_exact(16)
                .map(|block| block.view_bits::<Msb0>())
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

            //decode_block_mode_0(&blocks[0]);

            let decoded = blocks
                .into_iter()
                .filter_map(|b| {
                    let decoded = match b.first_one().unwrap() {
                        0 => decode_block_mode_0(b),
                        1 => decode_block_mode_1(b),
                        _ => return None,
                    };
                    Some(decoded)
                })
                .collect::<Vec<_>>();
            println!("done {}", decoded.len());
        }
    }
}
