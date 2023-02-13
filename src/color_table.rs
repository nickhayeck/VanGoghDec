use std::ops::Index;
use crate::raw_image::RawImage;


#[derive(Debug, Clone, Copy)]
pub struct ColorTable([[u8 ; 3] ; 255]);

impl ColorTable {
    /// wraps an array of the proper size in this struct 
    pub fn wrap(w: [[u8 ; 3] ; 255]) -> Self {
        ColorTable(w)
    }
    /// provides the user with access to the internal structure
    pub fn inner(&self) -> [[u8 ; 3] ; 255] {
        self.0
    }
    /// given a raw image generate the optimal color table
    pub fn generate_optimal(_img: &RawImage) -> Self {
        let mut table = [[0 ; 3] ; 255];
        // constant color palette for now.
        let mut inc = 0;
        for red in [0, 64, 128, 191, 255] {
            for green in [0, 64, 128, 191, 255] {
                for blue in [0, 64, 128, 191, 255] {
                    table[inc][0] = red;
                    table[inc][1] = green;
                    table[inc][2] = blue;
                    inc += 1;
                }
            }
        }

        ColorTable(table)
    }
    pub fn find_index_nearest(&self, red: u8, green: u8, blue: u8) -> u8 {
        // cast using max(min(int(x * 6 / 255),5),0)
        // 32.0, 96.0, 159.5, 223.0
        let red_ind = f32::round(red as f32 * 4.0 / 255.0) as u8; 
        let green_ind = f32::round(green as f32 * 4.0 / 255.0) as u8;
        let blue_ind = f32::round(blue as f32 * 4.0 / 255.0) as u8;
        // convert to table index
        red_ind*5*5 + green_ind*5 + blue_ind
    }
}

impl Index<usize> for ColorTable {
    type Output = [u8 ; 3];

    fn index(&self, index: usize) -> &Self::Output {
        return &self.0[index];
    }
}