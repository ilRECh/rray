pub struct Texture {
    walls: WallsTexture
}

use ggez::{
    graphics,
    winit::dpi::PhysicalSize,
    Context
};

use crate::raycasting::Line;

type WallsTexture = Vec<Vec<u8>>;

const TEXTURE_WIDTH: u32 = 64;
const TEXTURE_HEIGHT: u32 = 64;

// const WALL_TEXTURE_BLACK_CROSS: i32 = 1;
// const WALL_TEXTURE_SLOPED_GREYSCALE: i32 = 2;
// const WALL_TEXTURE_SLOPED_YELLOW_GRADIENT: i32 = 3;
// const WALL_TEXTURE_XOR_GREYSCALE: i32 = 4;
// const WALL_TEXTURE_XOR_GREEN: i32 = 5;
// const WALL_TEXTURE_RED_BRICKS: i32 = 6;
// const WALL_TEXTURE_RED_GRADIENT: i32 = 7;
const WALL_TEXTURE_FLAT_GREY: i32 = 8;

impl Texture {
    pub fn new(ctx: &mut Context) -> Self {
        let mut walls = Vec::with_capacity(8);

        let bluestone = graphics::Image::from_path(ctx, "/bluestone.png").unwrap();
        walls.push(bluestone.to_pixels(ctx).unwrap());

        let colorstone = graphics::Image::from_path(ctx, "/colorstone.png").unwrap();
        walls.push(colorstone.to_pixels(ctx).unwrap());

        let eagle = graphics::Image::from_path(ctx, "/eagle.png").unwrap();
        walls.push(eagle.to_pixels(ctx).unwrap());

        let greystone = graphics::Image::from_path(ctx, "/greystone.png").unwrap();
        walls.push(greystone.to_pixels(ctx).unwrap());

        let mossy = graphics::Image::from_path(ctx, "/mossy.png").unwrap();
        walls.push(mossy.to_pixels(ctx).unwrap());

        let purplestone = graphics::Image::from_path(ctx, "/purplestone.png").unwrap();
        walls.push(purplestone.to_pixels(ctx).unwrap());

        let redbrick = graphics::Image::from_path(ctx, "/redbrick.png").unwrap();
        walls.push(redbrick.to_pixels(ctx).unwrap());

        let wood = graphics::Image::from_path(ctx, "/wood.png").unwrap();
        walls.push(wood.to_pixels(ctx).unwrap());

        Self {
            walls
        }
    }

    pub fn convert_dda_to_pixels(
        &self,
        screen_size: &PhysicalSize<u32>,
        lines: Vec<Line>,
        pixels: &mut [u8]
    ) {
        for line in lines.iter() {
            let wall = self.code_to_texture(line.wall_code);

            // i can modify the wall brigtness however i want. this time i'm just dividing it
            let dimm = if line.wall_side > 0 { 4 } else { 1 };
            let mut texture_x = (line.wall_x * TEXTURE_WIDTH as f32) as u32;

            if (line.wall_side == 0 && line.ray_dir_x > 0.0) || (line.wall_side == 1 && line.ray_dir_y < 0.0) {
                texture_x = TEXTURE_WIDTH - texture_x - 1;
            }

            let step = TEXTURE_HEIGHT as f32 / (line.y_end - line.y_start);
            let mut texture_pos = (line.y_start - screen_size.height as f32 / 2.0 + (line.y_end - line.y_start) / 2.0) * step;

            let start = if line.y_start < 0.0 {
                texture_pos += -1.0 * line.y_start * step;
                0
            } else {
                line.y_start as u32
            };

            let end = if line.y_end >= screen_size.height as f32 {
                screen_size.height - 1
            } else {
                line.y_end as u32
            };

            for y in start..end {
                let texture_y = texture_pos as u32 & (TEXTURE_HEIGHT - 1);
                texture_pos += step;
                let index_pixel = (y * screen_size.width + line.screen_x as u32) as usize * 4;
                let index_texture = (TEXTURE_HEIGHT * texture_y + texture_x) as usize * 4;

                pixels[index_pixel + 0] = wall[index_texture + 0];
                pixels[index_pixel + 1] = wall[index_texture + 1];
                pixels[index_pixel + 2] = wall[index_texture + 2];
                pixels[index_pixel + 3] = wall[index_texture + 3] / dimm;
            }
        }
    }

    fn code_to_texture(&self, code: i32) -> &Vec<u8> {
        if code < 0 || code > WALL_TEXTURE_FLAT_GREY {
            panic!("No such code is associated with any texture!");
        }

        &self.walls[code as usize]
    }
}
