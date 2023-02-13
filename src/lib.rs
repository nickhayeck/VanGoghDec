// Let the crate be named what we want!
#![allow(non_snake_case)]

use color_table::ColorTable;
use raw_image::RawImage;

pub mod raw_image;
pub mod color_table;

pub struct VanGoghEncoder {
    // configuration variables
    block_size: usize,
}

impl VanGoghEncoder {
    /// creates an encoder with default settings
    pub fn new() -> Self {
        Self { block_size: 2 }
    }
    /// creates an encoder with default settings and a block size of `b`
    pub fn with_block_size(block_size: usize) -> Self {
        Self { block_size }
    }
    /// block_size setter
    pub fn set_block_size(mut self, block_size: usize) -> Self {
        self.block_size = block_size;
        return self;
    }
    /// encodes an image found at `path`. can read PNGs 
    pub fn encode_path(self, path: &str) -> VanGoghImage {
        let img = RawImage::new(path);
        return self.encode(img);
    } 
    /// encodes a raw image into the Van Gogh format
    pub fn encode(self, img: RawImage) -> VanGoghImage {
        // prepare the output array
        let mut img_data: Vec<u8> = vec![0 ; (img.width*img.height) as usize];
        // generate the optimal color palette based upon this image
        let color_table = ColorTable::generate_optimal(&img);
        // create (block_size x block_size) blocks and averages the image data within these blocks
        for i in 0..(img.height as usize / self.block_size) {
            for j in 0..(img.width as usize / self.block_size) {
                // the upper left index of this block in a flat array of the rgb values
                let upper_left = i * self.block_size * img.width as usize + j * self.block_size;
                // the soon-to-be-calculated averages of all the pixels
                let mut r_avg = 0;
                let mut g_avg = 0;
                let mut b_avg = 0;
                // compute the average
                for i_block in 0..self.block_size {
                    for j_block in 0..self.block_size {
                        // the index of this pixel in a flat array of rgb values
                        let pixel_index = upper_left + i_block + j_block * img.width as usize;
                        r_avg += img.red[pixel_index] as usize;
                        g_avg += img.green[pixel_index] as usize;
                        b_avg += img.blue[pixel_index] as usize;
                    }
                }
                // the byte-int averages for this block
                let red_target_avg = (r_avg/self.block_size/self.block_size) as usize;
                let green_target_avg = (g_avg/self.block_size/self.block_size) as usize;
                let blue_target_avg = (b_avg/self.block_size/self.block_size) as usize;
                // (1) set the first pixel in the block to itself, casted into the palette
                let color_ind = color_table.find_index_nearest(img.red[upper_left], img.green[upper_left], img.blue[upper_left]);
                img_data[upper_left] = color_ind;
                // (2) set each successive pixel to the color which brings the average of the VG block closest to the original
                let mut denom: usize = 1;
                let mut red_running_avg = color_table[color_ind as usize][0] as usize;
                let mut green_running_avg = color_table[color_ind as usize][1] as usize;
                let mut blue_running_avg = color_table[color_ind as usize][2] as usize;

                for i_block in 0..self.block_size {
                    for j_block in 0..self.block_size {
                        // the index of this pixel in a flat array of rgb values
                        let pixel_index = upper_left + i_block + j_block * img.width as usize;
                        // we compute the pixel whose value would bring the running average closest to the target average
                        let red_adj = usize::saturating_sub(red_target_avg * (denom + 1), red_running_avg * denom);
                        let green_adj = usize::saturating_sub(green_target_avg * (denom + 1), green_running_avg * denom);
                        let blue_adj = usize::saturating_sub(blue_target_avg * (denom + 1), blue_running_avg * denom);
                        // find the closest color in the palette and add it to the image data
                        let color_ind = color_table.find_index_nearest(red_adj as u8, green_adj as u8, blue_adj as u8);
                        img_data[pixel_index] = color_ind;
                        // update the running averages
                        let new_red = color_table[color_ind as usize][0] as usize;
                        let new_green = color_table[color_ind as usize][1] as usize;
                        let new_blue = color_table[color_ind as usize][2] as usize;

                        red_running_avg = red_running_avg * denom / (denom + 1) + new_red / (denom + 1) ;
                        green_running_avg = green_running_avg * denom / (denom + 1) + new_green / (denom + 1);
                        blue_running_avg = blue_running_avg * denom / (denom + 1) + new_blue / (denom + 1);
                        // increment the denominator of the running averages
                        denom += 1;
                    }
                }
            }
        }

        VanGoghImage { color_table, width: img.width, height: img.height, data: img_data }
    }
}

pub struct VanGoghImage {
    // encoded data
    color_table: color_table::ColorTable,
    width: u32,
    height: u32,
    data: Vec<u8>,
}
impl VanGoghImage {
    /// writes the file to the given path
    pub fn write_to_path(&self, path: &str) -> std::io::Result<()> {
        let file = std::fs::File::create(path).unwrap();
        self.write_to(file)
    }
    /// write the file using anything that implements the `std::io::Write` trait
    pub fn write_to<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        // flatten the color table to a one-dimesional vector
        let color_table_buf: Vec<u8> = self.color_table.inner().to_owned().into_iter().flatten().collect();
        // write the color_table to the file
        writer.write(&color_table_buf)?;
        // write the width and height to the file (big endian format)
        writer.write(&self.width.to_be_bytes())?;
        writer.write(&self.height.to_be_bytes())?;
        
        // write the image data to the file
        let size = (self.width*self.height) as usize;
        assert!(size*3 != self.data.len(), "Image data size (len:{}*3) does not match dimensions of image struct (w:{} h:{})", self.data.len(), self.width, self.height);
        writer.write(&self.data[..size])?;

        Ok(())
    }
    /// read an image at a given path
    pub fn read_from_path(path: &str) -> std::io::Result<Self> {
        if path.split('.').last().unwrap().ne("vg") {
            println!("[WARN] The file at this path ({}) does not have a .vg extension.", path);
        }

        let file = std::fs::File::open(path).unwrap();
        VanGoghImage::read_from(file)
    }
    /// read an image from a struct that implements the `std::io::Read` trait
    pub fn read_from<R: std::io::Read>(mut reader: R) -> std::io::Result<Self> {
        // read color_table in
        let mut color_table_buf = [0u8 ; 255*3];
        if reader.read(&mut color_table_buf[..])? != 255*3 {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Could not read the correct number of bytes from the file. Corrupted!"));
        }

        let color_table_arr: [[u8 ; 3] ; 255];
        unsafe { color_table_arr = std::mem::transmute(color_table_buf); }
        let color_table = ColorTable::wrap(color_table_arr);
        
        // read width and height of the image
        let mut width_buf = [0u8 ; 4];
        let mut height_buf = [0u8 ; 4];
        
        reader.read(&mut width_buf[..])?;
        reader.read(&mut height_buf[..])?;
        
        let width = u32::from_be_bytes(width_buf);
        let height = u32::from_be_bytes(height_buf);
        
        // read the image data
        let size = (width*height) as usize;
        let mut img_data = vec![0u8 ; size];

        reader.read(&mut img_data)?;

        Ok(VanGoghImage { color_table, width, height, data: img_data})
    }

}

pub struct VanGoghDecoder;

impl VanGoghDecoder {
    /// decodes an image found at `path`. can write to PNGs 
    pub fn decode_path(path: &str) -> RawImage {
        let img = VanGoghImage::read_from_path(path).unwrap();
        return Self::decode(img);
    } 
    /// decodes a raw image into the Van Gogh format
    pub fn decode(img: VanGoghImage) -> RawImage {
        let mut red = vec![0 ; (img.width * img.height) as usize];
        let mut green = vec![0 ; (img.width * img.height) as usize];
        let mut blue = vec![0 ; (img.width * img.height) as usize];

        for i in 0..(img.width * img.height) as usize {
            let color = img.data[i];
            let r = img.color_table[color as usize][0];
            let g = img.color_table[color as usize][1];
            let b = img.color_table[color as usize][2];
            red[i] = r; green[i] = g; blue[i] = b;
        }

        RawImage { red, green, blue, width: img.width, height: img.height }
    }
}

#[cfg(test)]
mod tests {
    use super::VanGoghEncoder;
    use super::VanGoghDecoder;

    #[test]
    fn codec() {
        let frame1_enc = VanGoghEncoder::new().encode_path("img/frame1.png");
        let frame2_enc = VanGoghEncoder::new().encode_path("img/frame2.png");
        let frame3_enc = VanGoghEncoder::new().encode_path("img/frame3.png");
        let frame4_enc = VanGoghEncoder::new().encode_path("img/frame4.png");
        // Encode to a file
        frame1_enc.write_to_path("img/test/frame1.vg").unwrap();
        frame2_enc.write_to_path("img/test/frame2.vg").unwrap();
        frame3_enc.write_to_path("img/test/frame3.vg").unwrap();
        frame4_enc.write_to_path("img/test/frame4.vg").unwrap();

        let frame1_dec = VanGoghDecoder::decode_path("img/test/frame1.vg");
        let frame2_dec = VanGoghDecoder::decode_path("img/test/frame2.vg");
        let frame3_dec = VanGoghDecoder::decode_path("img/test/frame3.vg");
        let frame4_dec = VanGoghDecoder::decode_path("img/test/frame4.vg");
        // Encode to a file
        frame1_dec.to_png("img/test/output_frame1.png");
        frame2_dec.to_png("img/test/output_frame2.png");
        frame3_dec.to_png("img/test/output_frame3.png");
        frame4_dec.to_png("img/test/output_frame4.png");
    }
}

