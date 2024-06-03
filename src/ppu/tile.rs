pub const TILE_WIDTH: usize = 8;
pub const TILE_HEIGHT: usize = 8;
pub struct Tile {
    data: Vec<u8>,
}

impl Tile {
    pub fn new(data: &[u8]) -> Tile {
        assert!(data.len() == 16, "Tile data must be 16 bytes");
        Tile {
            data: data.to_vec(),
        }
    }

    pub fn get_pixel_index(&self, x: u8, y: u8) -> u8 {
        let low = self.data[y as usize];
        let high = self.data[y as usize + 8];
        let shift = 7 - x;
        let low_bit = (low >> shift) & 1;
        let high_bit = (high >> shift) & 1;
        (high_bit << 1) | low_bit
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_pixel_index() {
        let tile = Tile::new(&[
            0b01010000, 0b01010000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
            0b00000000, 0b00110000, 0b00110000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
            0b00000000, 0b00000000,
        ]);
        assert_eq!(tile.get_pixel_index(0, 0), 0);
        assert_eq!(tile.get_pixel_index(1, 0), 1);
        assert_eq!(tile.get_pixel_index(2, 0), 2);
        assert_eq!(tile.get_pixel_index(3, 0), 3);
        assert_eq!(tile.get_pixel_index(0, 1), 0);
        assert_eq!(tile.get_pixel_index(1, 1), 1);
        assert_eq!(tile.get_pixel_index(2, 1), 2);
        assert_eq!(tile.get_pixel_index(3, 1), 3);
    }
}
