use png::{OutputInfo, Decoder};

// struct used for translating common image types into (and from) RGB values
pub struct RawImage {
    // image data
    pub red: Vec<u8>,
    pub green: Vec<u8>,
    pub blue: Vec<u8>,
    // image dimensions
    pub width: u32,
    pub height: u32,
}

impl RawImage {
    pub fn empty(width: u32, height: u32) -> Self {
        let buf_len = (width*height) as usize;
        RawImage { red: vec![0 ; buf_len], green: vec![0 ; buf_len], blue: vec![0 ; buf_len], width, height }
    }
    pub fn new(path: &str) -> Self {
        match path.split('.').last().unwrap() {
            "png" => RawImage::from_png(path),
            _ => unimplemented!("I haven't gotten around to making this compatible with anything other than PNG. Vince was also very particular in the materials he used. :)"),
        }
    }
    pub fn from_png(path: &str) -> Self {
        // Create a decoder
        let png_decoder = Decoder::new(std::fs::File::open(path).unwrap());
        // read some metadata
        let mut reader = png_decoder.read_info().unwrap();
        // Allocate the output buffer.
        let mut buf = vec![0; reader.output_buffer_size()];
        // Read the next frame. An APNG might contain multiple frames.
        let info = reader.next_frame(&mut buf).unwrap();

        return match info.color_type {
            png::ColorType::Rgb => {
                Self::from_png_rgb(buf, info)
            },
            png::ColorType::Rgba => {
                Self::from_png_rgba(buf, info)
            },
            png::ColorType::Grayscale => todo!(),
            png::ColorType::GrayscaleAlpha => todo!(),
            png::ColorType::Indexed => todo!(),
        };
    }

    fn from_png_rgb(buf: Vec<u8>, info: OutputInfo) -> Self {
        let rgb_buf_len = (info.width * info.height) as usize;
        let mut r_buf = Vec::with_capacity(rgb_buf_len);
        let mut g_buf = Vec::with_capacity(rgb_buf_len);
        let mut b_buf = Vec::with_capacity(rgb_buf_len);

        for index in 0..rgb_buf_len {
            r_buf.push(buf[3*index]);
            g_buf.push(buf[3*index+1]);
            b_buf.push(buf[3*index+2]);
        }

        Self { red: r_buf, green: g_buf, blue: b_buf, width: info.width, height: info.height }
    }

    fn from_png_rgba(buf: Vec<u8>, info: OutputInfo) -> Self {
        let rgb_buf_len = (info.width * info.height) as usize;
        let mut r_buf = Vec::with_capacity(rgb_buf_len);
        let mut g_buf = Vec::with_capacity(rgb_buf_len);
        let mut b_buf = Vec::with_capacity(rgb_buf_len);

        for index in 0..rgb_buf_len {
            // convert RGBA to RGB
            let new_r = buf[4*index] as u16 * buf[4*index+3] as u16 / 255;
            let new_g = buf[4*index+1] as u16 * buf[4*index+3] as u16 / 255;
            let new_b = buf[4*index+2] as u16 * buf[4*index+3] as u16 / 255;
            // append R
            r_buf.push(new_r as u8);
            // append G
            g_buf.push(new_g as u8);
            // append B
            b_buf.push(new_b as u8);
        }

        Self { red: r_buf, green: g_buf, blue: b_buf, width: info.width, height: info.height }
    }

    /// writes the RawImage to a png file at `path`
    pub fn to_png(&self, path: &str) {
        // open our file
        let file = std::fs::File::create(path).unwrap();
        // create an encoder and set the color profile
        let mut encoder = png::Encoder::new(file, self.width, self.height);
        encoder.set_color(png::ColorType::Rgb);
        // write the header and generate the bit writer
        let mut writer = encoder.write_header().unwrap();
        // convert the seperate rgb vectors into one vector
        let mut data: Vec<u8> = vec![0 ; (self.height*self.width*3) as usize];
        for i in 0..(self.height*self.width) as usize {
            data[3*i + 0] = self.red[i]; 
            data[3*i + 1] = self.green[i];
            data[3*i + 2] = self.blue[i];
        }
        writer.write_image_data(&data).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::RawImage;

    #[test]
    fn from_png() {
        let frame1 = RawImage::from_png("img/frame1.png");
        let frame2 = RawImage::from_png("img/frame2.png");
        let frame3 = RawImage::from_png("img/frame3.png");

        println!("frame1 -- w:{} h:{}", frame1.width, frame1.height);
        println!("frame2 -- w:{} h:{}", frame2.width, frame2.height);
        println!("frame3 -- w:{} h:{}", frame3.width, frame3.height);

        assert_eq!(frame1.width, 1920); assert_eq!(frame1.height, 1080);
        assert_eq!(frame2.width, 1920); assert_eq!(frame2.height, 1080);
        assert_eq!(frame3.width, 1000); assert_eq!(frame3.height, 1500);
    }

    #[test]
    fn from_arbitrary() {
        let frame1 = RawImage::new("img/frame1.png");
        let frame2 = RawImage::new("img/frame2.png");
        let frame3 = RawImage::new("img/frame3.png");

        println!("frame1 -- w:{} h:{}", frame1.width, frame1.height);
        println!("frame2 -- w:{} h:{}", frame2.width, frame2.height);
        println!("frame3 -- w:{} h:{}", frame3.width, frame3.height);

        assert_eq!(frame1.width, 1920); assert_eq!(frame1.height, 1080);
        assert_eq!(frame2.width, 1920); assert_eq!(frame2.height, 1080);
        assert_eq!(frame3.width, 1000); assert_eq!(frame3.height, 1500);
    }
}