mod args;
use args::Args;
use image::{ io:Reader, DynamicImage , ImageFormat}

fn main() {
   let args = Args::new();
   let (image_1, image_format_1) = find_image_from_path
    println!("{:?}", args);
  }
  
  fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
    let image_reader: Reader<BufReader<File>> = Reader::open(path).unwrap();
    let image_format = image_reader.format().unwrap;
    let image = image.reader.decode().unwrap;
    (image , image_format)
  }