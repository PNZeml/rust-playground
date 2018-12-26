extern crate colored;
extern crate image;
extern crate rayon;
extern crate glob;

pub mod input;
pub mod weight;
pub mod neural_node;
pub mod image_class;

pub const X_IMG_SIZE: usize = 32;
pub const Y_IMG_SIZE: usize = 32;

pub fn get_img_size() -> usize {
    X_IMG_SIZE * Y_IMG_SIZE
}