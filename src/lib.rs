pub mod bc7_unorm;

#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader, path::Path};

    use bitvec::{
        field::BitField,
        order::{Lsb0, Msb0},
        store::BitStore,
        view::BitView,
    };
    use ddsfile::{Caps, DataFormat, Dds, FourCC};

    use crate::bc7_unorm::{extract_mode, PARTITION_TABLE};

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
            println!("{}", blocks[0]);

            let mode = blocks[0].first_one().unwrap();
            println!("mode: {}", mode);
            assert_eq!(mode, 0);

            let partition = blocks[0][1..5].load::<usize>();
            let partition_table = PARTITION_TABLE[partition];
            println!("partition: {} {:?}", partition, partition_table);

            let ps = &blocks[0][77..83];

            // Rx6 Gx6 Bx6
            let mut rgbs = blocks[0][5..77]
                .chunks_exact(4)
                .map(|c| c.load::<u8>())
                .enumerate()
                .map(|(i, rgb)| if ps[i % 4] { rgb << 1 } else { rgb })
                .collect::<Vec<_>>();
            println!("rgbs: {:?}", rgbs);

            let indices = &blocks[0][83..]
                .chunks_exact(3)
                .map(|c| c.load::<u8>())
                .collect::<Vec<_>>();

            println!("indices: {:?}", indices);

            //let mode = extract_mode(blocks[0]);
            //println!("mode: {}", mode);

            //let partition = extract_partition(&blocks[0].try_into().unwrap(), mode);
            //println!("partition: {}", partition);
        }
    }
}
