mod args;
use args::Args;
use image::{imageops::FilterType::Triangle, io::Reader, DynamicImage, ImageFormat};

#[derive(Debug)]
enum ImageDataErrors {
    DifferentImageFormats,
    BuffferTooSmall,
}

struct FloatingImage {
    width: u32,
    height: u32,
    data: Vec<u8>,
    name: String,
}

impl FloatingImage {
    fn new(width: u32, height: u32, name: String) -> Self {
        let buffersize = width * height * 4;
        let data = Vec::with_capacity(buffersize.try_into().unwrap());
        FloatingImage {
            width,
            height,
            data,
            name,
        }
    }

    fn set_data(&mut self, data: Vec<u8>) -> Result<(), ImageDataErrors> {
        if data.len() > self.data.capacity(){
            return Err(ImageDataErrors::BuffferTooSmall);
        }

        self.data = data;
        Ok(())
    }
 
}

fn main() -> Result<(), ImageDataErrors> {
    let mut _args = Args::new();
    let (_image_1, _image_1_format) = find_image_from_path(_args.image_1);
    let (_image_2, _image_2_format) = find_image_from_path(_args.image_2);

    if _image_1_format != _image_2_format {
        return Err(ImageDataErrors::DifferentImageFormats);
    }
    let (_image_1, _image_2) = standardize_size(_image_1, _image_2);
   
    let mut output = FloatingImage::new(_image_2.width(), _image_2.height(), _args.output);
    let combined_data = combine_images(_image_1, _image_2);
    
    output.set_data(combined_data)?;
   
    image::save_buffer_with_format(output.name, &output.data, output.width, output.height, image::ColorType::Rgba8, _image_1_format).unwrap();
    return Ok(());
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
    // generate image object and image from path using the image crate

    let image_reader = Reader::open(path).unwrap();
    let image_format: ImageFormat = image_reader.format().unwrap();
    let image: DynamicImage = image_reader.decode().unwrap();

    return (image, image_format);
}

fn get_smallest_dimension(dim1: (u32, u32), dim2: (u32, u32)) -> (u32, u32) {
    // This will find the image with the smaller dimension

    let pix1 = dim1.0 * dim1.1;
    let pix2 = dim2.0 * dim2.1;
    return if pix1 > pix2 { dim2 } else { dim1 };
}

fn standardize_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
    let dim = get_smallest_dimension(
        (image_1.width(), image_1.height()),
        (image_2.width(), image_2.height()),
    );
    println!("Dimension is : {:?}", dim);

    if dim == (image_1.width(), image_1.height()) {
        // resize image 2
        (image_1, image_2.resize(dim.0, dim.1, Triangle))
    } else {
        // resize image 1
        (image_1.resize(dim.0, dim.1, Triangle), image_2)
    }
}

fn combine_images(image1: DynamicImage, image2: DynamicImage) -> Vec<u8> {
    let vec1 = image1.to_rgba8().into_vec();
    let vec2 = image2.to_rgba8().into_vec();

    return alternate_pixels(vec1, vec2);
}

fn alternate_pixels(v1: Vec<u8>, v2: Vec<u8>) -> Vec<u8> {
    let sample: u8 = 0;
    let mut size: usize = 0;
    
    if v1.len() < v2.len() {
        size = v1.len();
     } else {
        size = v2.len();
    }
    
    let mut combined_data = vec![sample; size];
    let mut i = 0;
  
    while i < size {
        if i % 8 == 0 {
            combined_data.splice(i..=i + 3, set_rgba(&v1, i, i + 3));
        } else {
            combined_data.splice(i..=i + 3, set_rgba(&v2, i, i + 3));
        }
        i += 4;
    }

    combined_data
}

fn set_rgba(vec: &Vec<u8>, start: usize, end: usize) -> Vec<u8> {
    let mut rgba = Vec::new();
    for i in start..=end {
        let val = match vec.get(i) {
            Some(d) => *d,
            None => panic!("Index is out of bound"),
        };
        rgba.push(val);
    }
    rgba
}
