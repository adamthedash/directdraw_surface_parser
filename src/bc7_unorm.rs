use bitvec::{field::BitField, order::Msb0, slice::BitSlice};

/// https://learn.microsoft.com/en-us/windows/win32/direct3d11/bc7-format-mode-reference
/// https://github.com/microsoft/DirectXTex/blob/main/DirectXTex/BC6HBC7.cpp
pub const PARTITION_TABLE: [[usize; 16]; 64] = [
    // BC7 Partition Set for 3 Subsets
    [0, 0, 1, 1, 0, 0, 1, 1, 0, 2, 2, 1, 2, 2, 2, 2], // Shape 0
    [0, 0, 0, 1, 0, 0, 1, 1, 2, 2, 1, 1, 2, 2, 2, 1], // Shape 1
    [0, 0, 0, 0, 2, 0, 0, 1, 2, 2, 1, 1, 2, 2, 1, 1], // Shape 2
    [0, 2, 2, 2, 0, 0, 2, 2, 0, 0, 1, 1, 0, 1, 1, 1], // Shape 3
    [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 2, 2, 1, 1, 2, 2], // Shape 4
    [0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 2, 2, 0, 0, 2, 2], // Shape 5
    [0, 0, 2, 2, 0, 0, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1], // Shape 6
    [0, 0, 1, 1, 0, 0, 1, 1, 2, 2, 1, 1, 2, 2, 1, 1], // Shape 7
    [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2], // Shape 8
    [0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2], // Shape 9
    [0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2], // Shape 10
    [0, 0, 1, 2, 0, 0, 1, 2, 0, 0, 1, 2, 0, 0, 1, 2], // Shape 11
    [0, 1, 1, 2, 0, 1, 1, 2, 0, 1, 1, 2, 0, 1, 1, 2], // Shape 12
    [0, 1, 2, 2, 0, 1, 2, 2, 0, 1, 2, 2, 0, 1, 2, 2], // Shape 13
    [0, 0, 1, 1, 0, 1, 1, 2, 1, 1, 2, 2, 1, 2, 2, 2], // Shape 14
    [0, 0, 1, 1, 2, 0, 0, 1, 2, 2, 0, 0, 2, 2, 2, 0], // Shape 15
    [0, 0, 0, 1, 0, 0, 1, 1, 0, 1, 1, 2, 1, 1, 2, 2], // Shape 16
    [0, 1, 1, 1, 0, 0, 1, 1, 2, 0, 0, 1, 2, 2, 0, 0], // Shape 17
    [0, 0, 0, 0, 1, 1, 2, 2, 1, 1, 2, 2, 1, 1, 2, 2], // Shape 18
    [0, 0, 2, 2, 0, 0, 2, 2, 0, 0, 2, 2, 1, 1, 1, 1], // Shape 19
    [0, 1, 1, 1, 0, 1, 1, 1, 0, 2, 2, 2, 0, 2, 2, 2], // Shape 20
    [0, 0, 0, 1, 0, 0, 0, 1, 2, 2, 2, 1, 2, 2, 2, 1], // Shape 21
    [0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 2, 2, 0, 1, 2, 2], // Shape 22
    [0, 0, 0, 0, 1, 1, 0, 0, 2, 2, 1, 0, 2, 2, 1, 0], // Shape 23
    [0, 1, 2, 2, 0, 1, 2, 2, 0, 0, 1, 1, 0, 0, 0, 0], // Shape 24
    [0, 0, 1, 2, 0, 0, 1, 2, 1, 1, 2, 2, 2, 2, 2, 2], // Shape 25
    [0, 1, 1, 0, 1, 2, 2, 1, 1, 2, 2, 1, 0, 1, 1, 0], // Shape 26
    [0, 0, 0, 0, 0, 1, 1, 0, 1, 2, 2, 1, 1, 2, 2, 1], // Shape 27
    [0, 0, 2, 2, 1, 1, 0, 2, 1, 1, 0, 2, 0, 0, 2, 2], // Shape 28
    [0, 1, 1, 0, 0, 1, 1, 0, 2, 0, 0, 2, 2, 2, 2, 2], // Shape 29
    [0, 0, 1, 1, 0, 1, 2, 2, 0, 1, 2, 2, 0, 0, 1, 1], // Shape 30
    [0, 0, 0, 0, 2, 0, 0, 0, 2, 2, 1, 1, 2, 2, 2, 1], // Shape 31
    [0, 0, 0, 0, 0, 0, 0, 2, 1, 1, 2, 2, 1, 2, 2, 2], // Shape 32
    [0, 2, 2, 2, 0, 0, 2, 2, 0, 0, 1, 2, 0, 0, 1, 1], // Shape 33
    [0, 0, 1, 1, 0, 0, 1, 2, 0, 0, 2, 2, 0, 2, 2, 2], // Shape 34
    [0, 1, 2, 0, 0, 1, 2, 0, 0, 1, 2, 0, 0, 1, 2, 0], // Shape 35
    [0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 0, 0, 0, 0], // Shape 36
    [0, 1, 2, 0, 1, 2, 0, 1, 2, 0, 1, 2, 0, 1, 2, 0], // Shape 37
    [0, 1, 2, 0, 2, 0, 1, 2, 1, 2, 0, 1, 0, 1, 2, 0], // Shape 38
    [0, 0, 1, 1, 2, 2, 0, 0, 1, 1, 2, 2, 0, 0, 1, 1], // Shape 39
    [0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 0, 0, 0, 0, 1, 1], // Shape 40
    [0, 1, 0, 1, 0, 1, 0, 1, 2, 2, 2, 2, 2, 2, 2, 2], // Shape 41
    [0, 0, 0, 0, 0, 0, 0, 0, 2, 1, 2, 1, 2, 1, 2, 1], // Shape 42
    [0, 0, 2, 2, 1, 1, 2, 2, 0, 0, 2, 2, 1, 1, 2, 2], // Shape 43
    [0, 0, 2, 2, 0, 0, 1, 1, 0, 0, 2, 2, 0, 0, 1, 1], // Shape 44
    [0, 2, 2, 0, 1, 2, 2, 1, 0, 2, 2, 0, 1, 2, 2, 1], // Shape 45
    [0, 1, 0, 1, 2, 2, 2, 2, 2, 2, 2, 2, 0, 1, 0, 1], // Shape 46
    [0, 0, 0, 0, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1], // Shape 47
    [0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 2, 2, 2, 2], // Shape 48
    [0, 2, 2, 2, 0, 1, 1, 1, 0, 2, 2, 2, 0, 1, 1, 1], // Shape 49
    [0, 0, 0, 2, 1, 1, 1, 2, 0, 0, 0, 2, 1, 1, 1, 2], // Shape 50
    [0, 0, 0, 0, 2, 1, 1, 2, 2, 1, 1, 2, 2, 1, 1, 2], // Shape 51
    [0, 2, 2, 2, 0, 1, 1, 1, 0, 1, 1, 1, 0, 2, 2, 2], // Shape 52
    [0, 0, 0, 2, 1, 1, 1, 2, 1, 1, 1, 2, 0, 0, 0, 2], // Shape 53
    [0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 2, 2, 2, 2], // Shape 54
    [0, 0, 0, 0, 0, 0, 0, 0, 2, 1, 1, 2, 2, 1, 1, 2], // Shape 55
    [0, 1, 1, 0, 0, 1, 1, 0, 2, 2, 2, 2, 2, 2, 2, 2], // Shape 56
    [0, 0, 2, 2, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 2, 2], // Shape 57
    [0, 0, 2, 2, 1, 1, 2, 2, 1, 1, 2, 2, 0, 0, 2, 2], // Shape 58
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 1, 1, 2], // Shape 59
    [0, 0, 0, 2, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 1], // Shape 60
    [0, 2, 2, 2, 1, 2, 2, 2, 0, 2, 2, 2, 1, 2, 2, 2], // Shape 61
    [0, 1, 0, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2], // Shape 62
    [0, 1, 1, 1, 2, 0, 1, 1, 2, 2, 0, 1, 2, 2, 2, 0], // Shape 63
];

const FIXUP_TABLE: [[usize; 3]; 64] = [
    // BC7 Partition Set Fixups for 3 Subsets
    [0, 3, 15],
    [0, 3, 8],
    [0, 15, 8],
    [0, 15, 3],
    [0, 8, 15],
    [0, 3, 15],
    [0, 15, 3],
    [0, 15, 8],
    [0, 8, 15],
    [0, 8, 15],
    [0, 6, 15],
    [0, 6, 15],
    [0, 6, 15],
    [0, 5, 15],
    [0, 3, 15],
    [0, 3, 8],
    [0, 3, 15],
    [0, 3, 8],
    [0, 8, 15],
    [0, 15, 3],
    [0, 3, 15],
    [0, 3, 8],
    [0, 6, 15],
    [0, 10, 8],
    [0, 5, 3],
    [0, 8, 15],
    [0, 8, 6],
    [0, 6, 10],
    [0, 8, 15],
    [0, 5, 15],
    [0, 15, 10],
    [0, 15, 8],
    [0, 8, 15],
    [0, 15, 3],
    [0, 3, 15],
    [0, 5, 10],
    [0, 6, 10],
    [0, 10, 8],
    [0, 8, 9],
    [0, 15, 10],
    [0, 15, 6],
    [0, 3, 15],
    [0, 15, 8],
    [0, 5, 15],
    [0, 15, 3],
    [0, 15, 6],
    [0, 15, 6],
    [0, 15, 8],
    [0, 3, 15],
    [0, 15, 3],
    [0, 5, 15],
    [0, 5, 15],
    [0, 5, 15],
    [0, 8, 15],
    [0, 5, 15],
    [0, 10, 15],
    [0, 5, 15],
    [0, 10, 15],
    [0, 8, 15],
    [0, 13, 15],
    [0, 15, 3],
    [0, 12, 15],
    [0, 3, 15],
    [0, 3, 8],
];

#[derive(Debug)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug)]
pub struct RGBInterval {
    pub min: RGB,
    pub max: RGB,
}

impl RGBInterval {
    /// Sample within the interval
    /// x should represent where to sample in the 0-255 range
    pub fn lerp(&self, t: u8) -> RGB {
        RGB {
            r: interpolate_u8(self.min.r, self.max.r, t),
            g: interpolate_u8(self.min.g, self.max.g, t),
            b: interpolate_u8(self.min.b, self.max.b, t),
        }
    }
}

/// Fast integer interpolation
fn interpolate_u8(min: u8, max: u8, t: u8) -> u8 {
    let (low, high) = if min < max { (min, max) } else { (max, min) };
    low + ((high - low) as u16 * t as u16 / 255) as u8
}

pub fn decode_block_mode_0(block: &BitSlice<u8, Msb0>) -> [RGB; 16] {
    assert_eq!(block.len(), 128);

    let mode = block.first_one().expect("No valid BC7 mode found");
    assert_eq!(mode, 0);

    // =========================== Step 1: Go from raw bits to raw native dtypes

    // Table selection
    let partition = block[1..5].load::<usize>();
    let partition_table = PARTITION_TABLE[partition];
    //println!("partition: {} {:?}", partition, partition_table);
    let fixup_table = FIXUP_TABLE[partition];
    //println!("fixup {:?}", fixup_table);

    // Rx6 Gx6 Bx6
    let ps = &block[77..83];
    let rgbs = block[5..77]
        .chunks_exact(4)
        .map(|c| c.load::<u8>())
        .enumerate()
        // Shift by common offset
        .map(|(i, rgb)| if ps[i % 4] { rgb << 1 } else { rgb })
        // Shift from 5 bit to 8 bit range
        .map(|rgb| rgb << 3)
        .collect::<Vec<_>>();
    //println!("rgbs: {:?}", rgbs);

    // Indices into intervals
    let mut indices = [0; 16];
    let index_block = &block[83..];
    let mut offset = 0;
    indices.iter_mut().enumerate().for_each(|(i, x)| {
        let width = if fixup_table.contains(&i) { 2 } else { 3 };
        *x = index_block[offset..offset + width].load::<u8>();
        offset += width;
    });
    //println!("indices: {:?}", indices);

    // ========================= Step 2: Prepare RGB lookup structs
    let rgb_intervals: [RGBInterval; 3] = (0..6)
        .map(|i| RGB {
            r: rgbs[i],
            g: rgbs[i + 6],
            b: rgbs[i + 12],
        })
        .array_chunks::<2>()
        .map(|[min, max]| RGBInterval { min, max })
        .collect::<Vec<_>>()
        // No way to directly collect into an array :(
        .try_into()
        .unwrap();

    //println!("parsed rgbs: {:?}", rgb_intervals);

    // ======================== Step 3: Lerp em!
    let pixels = partition_table
        .into_iter()
        .zip(indices)
        .map(|(table, index)| rgb_intervals[table].lerp(index))
        .collect::<Vec<RGB>>()
        // No way to directly collect into an array :(
        .try_into()
        .unwrap();
    //println!("Decoded: {:?}", pixels);

    pixels
}
