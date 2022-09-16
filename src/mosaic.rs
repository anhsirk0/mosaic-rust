use glob::glob;
use image::imageops::overlay;
use image::imageops::FilterType;
use image::DynamicImage;
use image::GenericImageView;
use image::Rgb;

struct Tile {
    avg_rgb: Rgb<f32>,
    img: DynamicImage,
}

pub struct Mosaic {
    img_path: String,
    output_path: String,
    tiles_dir: String,
    tiles: Vec<Tile>,
    tile_width: u32,
    tile_height: u32,
}

impl Mosaic {
    pub fn new(img_path: String, output_path: String, tiles_dir: String) -> Self {
        Self {
            img_path,
            output_path,
            tiles_dir,
            tiles: vec![],
            tile_width: 50,
            tile_height: 50,
        }
    }

    fn distance(&self, rgb1: &Rgb<f32>, rgb2: &Rgb<f32>) -> f32 {
        // returns distance between 2 rgb values
        // (like 3d coordinates, r = sqrt(x^2 + y^2 + z^2))
        let r: f32 = rgb1[0] - rgb2[0];
        let g: f32 = rgb1[1] - rgb2[1];
        let b: f32 = rgb1[2] - rgb2[2];

        let d: f32 = r.powf(2.0) + g.powf(2.0) + b.powf(2.0);
        d.sqrt()
    }

    fn get_avg_rgb(&self, img: &DynamicImage) -> Rgb<f32> {
        let mut avg_r: f32 = 0.0;
        let mut avg_g: f32 = 0.0;
        let mut avg_b: f32 = 0.0;

        let width = img.width();
        let height = img.height();
        let size = (width * height) as f32;

        for pixel in img.to_rgb32f().pixels() {
            avg_r += pixel[0] as f32 / size;
            avg_g += pixel[1] as f32 / size;
            avg_b += pixel[2] as f32 / size;
        }

        Rgb::from([avg_r, avg_g, avg_b])
    }

    fn get_closest(&self, rgb: &Rgb<f32>) -> usize {
        let mut max = f32::MAX;
        let mut index = 0;
        for (i, tile) in self.tiles.iter().enumerate() {
            let d = self.distance(rgb, &tile.avg_rgb);
            if max >= d {
                max = d;
                index = i;
            }
        }
        index
    }

    fn create_tiles(&mut self) {
        let glob_expr = self.tiles_dir.to_owned() + "/*.*g";
        for path in glob(&glob_expr).expect("Can't read Directory") {
            let img_path = path.unwrap().to_str().unwrap().to_string();
            let img = image::open(&img_path).unwrap().resize_to_fill(
                self.tile_width,
                self.tile_height,
                FilterType::Nearest,
            );
            self.tiles.push(Tile {
                avg_rgb: self.get_avg_rgb(&img),
                img: img,
            })
        }
    }

    fn create_mosaic(&mut self) {
        self.create_tiles();
        if self.tiles.len() == 0 {
            println!("Tiles directory '{}' has no images", self.tiles_dir);
            return;
        }

        println!("{} tiles loaded", self.tiles.len());

        let n: u32 = 5;
        let mut base_img = image::open(&self.img_path).unwrap().thumbnail(500, 500);
        let (width, height) = base_img.dimensions();

        let new_width: u32 = width * self.tile_width / n;
        let new_height: u32 = height * self.tile_height / n;
        let mut new_img = DynamicImage::new_rgb8(new_width, new_height);

        for w in (0..width).step_by(n as usize) {
            for h in (0..height).step_by(n as usize) {
                let piece = base_img.crop(w, h, n, n);
                let piece_rgb = self.get_avg_rgb(&piece);
                let i = self.get_closest(&piece_rgb);
                let x = w * self.tile_width / n;
                let y = h * self.tile_height / n;
                overlay(&mut new_img, &self.tiles[i].img, x.into(), y.into());
            }
        }

        match new_img.save(&self.output_path) {
            Ok(_t) => {
                println!("Mosaic saved as : {}", self.output_path)
            }
            Err(e) => {
                println!("Error occured {}", e)
            }
        }
    }

    pub fn create(&mut self) {
        self.create_mosaic();
    }
}
