pub struct Image {
    pixel_data: Vec<u8>, // BGR888
    width: u16,
    height: u16,
}

impl Image {
    pub fn new(width: u16, height: u16) -> Self {
        Image {
            pixel_data: blank_pixel_data(width, height),
            width,
            height,
        }
    }

    pub fn fill_with(&mut self, r: u8, g: u8, b: u8) -> &mut Self {
        for y in 0..self.height as usize {
            for x in 0..self.width as usize {
                self.pixel_data[3 * x + 3 * self.width as usize * y] = b;
                self.pixel_data[1 + 3 * x + 3 * self.width as usize * y] = g;
                self.pixel_data[2 + 3 * x + 3 * self.width as usize * y] = r;
            }
        }
        return self;
    }
}

fn blank_pixel_data(width: u16, height: u16) -> Vec<u8> {
    let mut v = Vec::with_capacity(width as usize * height as usize * 3);
    for _ in 0..height {
        for _ in 0..width {
            v.push(255);
            v.push(255);
            v.push(255);
        }
    }
    return v;
}

pub struct BMPFileHeader {
    signature: [u8; 2],
    size: u32,
    _reserved: u32,
    offset_to_pixels: u32,
}

impl BMPFileHeader {
    pub fn new(size: u32) -> BMPFileHeader {
        BMPFileHeader {
            signature: [0x42, 0x4D], // 'BM'
            size,
            _reserved: 0,
            offset_to_pixels: 26, // 14 (this header) + 12 (the other header)
        }
    }

    pub fn as_bytes(&self) -> [u8; 14] {
        let mut a = [0; 14];
        a[0] = self.signature[0];
        a[1] = self.signature[1];
        a[2..6].copy_from_slice(u32_to_bytes_little_endian(self.size).as_slice());
        a[6..10].fill(0); // reserved
        a[10..14].copy_from_slice(u32_to_bytes_little_endian(self.offset_to_pixels).as_slice());
        return a;
    }
}

// BITMAPCOREHEADER aka OS21XBITMAPHEADER
pub struct BMPCoreHeader {
    size: u32, // always 12 (bytes)
    width: u16,
    height: u16,
    num_color_planes: u16, // must be 1
    num_bbp: u16,
}

impl BMPCoreHeader {
    pub fn new(width: u16, height: u16) -> BMPCoreHeader {
        BMPCoreHeader {
            size: 12,
            width,
            height,
            num_color_planes: 1,
            num_bbp: 24,
        }
    }

    pub fn as_bytes(&self) -> [u8; 12] {
        let mut a = [0; 12];
        a[0..4].copy_from_slice(u32_to_bytes_little_endian(self.size).as_slice());
        a[4..6].copy_from_slice(u16_to_bytes_little_endian(self.width).as_slice());
        a[6..8].copy_from_slice(u16_to_bytes_little_endian(self.height).as_slice());
        a[8..10].copy_from_slice(u16_to_bytes_little_endian(self.num_color_planes).as_slice());
        a[10..12].copy_from_slice(u16_to_bytes_little_endian(self.num_bbp).as_slice());
        return a;
    }
}

pub struct BMPImage {
    file_header: BMPFileHeader,
    bmp_core_header: BMPCoreHeader,
    pixel_array: Vec<u8>,
}

impl From<Image> for BMPImage {
    fn from(image: Image) -> Self {
        let file_size = bmp_size_for_image(&image);
        let file_header = BMPFileHeader::new(file_size);
        let bmp_core_header: BMPCoreHeader = BMPCoreHeader::new(image.width, image.height);

        let row_size = bmp_rowsize(image.width);
        let pixel_array_size = bmp_pixel_array_size(row_size, image.height) as usize;
        let mut bmp_image = BMPImage {
            file_header,
            bmp_core_header,
            pixel_array: Vec::with_capacity(bmp_pixel_array_size(row_size, image.height) as usize),
        };

        let pixel_array = &mut bmp_image.pixel_array;
        for _ in 0..pixel_array_size {
            pixel_array.push(0);
        }

        for y in 0..image.height {
            for x in 0..image.width {
                let x_i: usize = x as usize * 3;
                let y: usize = y.into();
                let r = image.pixel_data
                    [x_i + 3 * (image.height as usize - 1 - y) * image.width as usize];
                let g = image.pixel_data
                    [x_i + 3 * (image.height as usize - 1 - y) * image.width as usize + 1];
                let b = image.pixel_data
                    [x_i + 3 * (image.height as usize - 1 - y) * image.width as usize + 2];

                pixel_array[x_i + y * row_size as usize] = r;
                pixel_array[x_i + y * row_size as usize + 1] = g;
                pixel_array[x_i + y * row_size as usize + 2] = b;
            }
        }

        bmp_image.pixel_array[0] = 0;

        return bmp_image;
    }
}

impl BMPImage {
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut output = vec![];
        output.extend_from_slice(self.file_header.as_bytes().as_slice());
        output.extend_from_slice(self.bmp_core_header.as_bytes().as_slice());
        output.extend(self.pixel_array.iter());
        return output;
    }
}

// bpp is always 24
fn bmp_rowsize(image_width: u16) -> u16 {
    let bpp = 24 as f64;
    let image_width = image_width as f64;
    return ((bpp * image_width / 32.0).ceil() as u16) * 4;
}

fn bmp_pixel_array_size(rowsize: u16, image_height: u16) -> u32 {
    rowsize as u32 * image_height as u32
}

fn bmp_size_for_image(image: &Image) -> u32 {
    let headers_size: u32 = 26;
    let pixel_array_size = bmp_pixel_array_size(bmp_rowsize(image.width), image.height);
    return headers_size + pixel_array_size;
}

fn u16_to_bytes_little_endian(value: u16) -> [u8; 2] {
    let mut a: [u8; 2] = [0; 2];
    let mut value = value;
    for i in 0..2 {
        a[i] = (value & 0xFF) as u8;
        value = value >> 8
    }

    return a;
}

fn u32_to_bytes_little_endian(value: u32) -> [u8; 4] {
    let mut a: [u8; 4] = [0; 4];
    let mut value = value;
    for i in 0..4 {
        a[i] = (value & 0xFF) as u8;
        value = value >> 8
    }

    return a;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u16_to_bytes() {
        let array = u16_to_bytes_little_endian(0x1234);
        assert_eq!(array[0], 0x34);
        assert_eq!(array[1], 0x12);
    }

    #[test]
    fn test_u32_to_bytes() {
        let array = u32_to_bytes_little_endian(0x12345678);
        assert_eq!(array[0], 0x78);
        assert_eq!(array[1], 0x56);
        assert_eq!(array[2], 0x34);
        assert_eq!(array[3], 0x12);
    }
}
