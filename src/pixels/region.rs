use std::ops::{Index, IndexMut};
use std::fmt;
use crate::Position;

// -----------------------------------------------------------------------------
//     - Region -
// -----------------------------------------------------------------------------
/// Represents a region of pixels.
/// ```
/// use nightmaregl::pixels::{Pixel, Pixels};
/// use nightmaregl::{Position, Size};
///
/// let size = Size::new(6, 6);
/// let green = Pixel { g: 255, ..Default::default() };
/// let pixels = Pixels::from_pixel(green, size);
///
/// let region = pixels.region(size.width, Position::new(2, 2), Size::new(3, 3));
///
/// // . . . . . .
/// // . . . . . .
/// // . . r r r .
/// // . . r r r .
/// // . . r r r .
/// // . . . . . .
/// ```
pub struct Region<'a, T> {
    pub(super) inner: Vec<&'a[T]>
}

impl<'a, T> Region<'a, T> {
    /// Iterator over rows of pixels
    pub fn rows(&self) -> impl Iterator<Item=&[T]> {
        self.inner.iter().cloned()
    }
}

impl<'a, T> Index<Position<usize>> for Region<'a, T> {
    type Output = T;

    fn index(&self, pos: Position<usize>) -> &Self::Output {
        &self.inner[pos.y][pos.x]
    }
}

impl<'a, T> Index<(usize, usize)> for Region<'a, T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.inner[y][x]
    }
}

impl<'a, T: fmt::Debug> fmt::Debug for Region<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.rows() {
            write!(f, "| ")?;
            for col in row {
                write!(f, "{:?} | ", col)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

impl<'a, T: fmt::Display> fmt::Display for Region<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.rows() {
            write!(f, "|")?;
            for col in row {
                write!(f, " {} |", col)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

// -----------------------------------------------------------------------------
//     - Region mut -
// -----------------------------------------------------------------------------
/// A mutable region
pub struct RegionMut<'a, T> {
    pub(super) inner: Vec<&'a mut[T]>
}

impl<'a, T> Index<Position<usize>> for RegionMut<'a, T> {
    type Output = T;

    fn index(&self, pos: Position<usize>) -> &Self::Output {
        &self.inner[pos.y][pos.x]
    }
}

impl<'a, T> Index<(usize, usize)> for RegionMut<'a, T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.inner[y][x]
    }
}

impl<'a, T> IndexMut<Position<usize>> for RegionMut<'a, T> {
    fn index_mut(&mut self, pos: Position<usize>) -> &mut Self::Output {
        &mut self.inner[pos.y][pos.x]
    }
}

impl<'a, T> IndexMut<(usize, usize)> for RegionMut<'a, T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.inner[y][x]
    }
}

#[cfg(test)]
mod test {
    use crate::Size;
    use crate::pixels::*;
    use super::*;

    macro_rules! p {
        ($r:expr) => {
            Pixel { r: $r as u8, ..Default::default() }
        }
    }

    #[test]
    fn get_region() {
        let pixels = vec![
            p!(00), p!(01), p!(02), p!(03),
            p!(04), p!(05), p!(06), p!(07),
            p!(08), p!(09), p!(10), p!(11),
            p!(12), p!(13), p!(14), p!(15),
        ];
        let mut pixels = Pixels::new(pixels);
        let row_width = 4;
        for i in 0..row_width * 4 {
            pixels[i] = Pixel { r: i as u8, ..Default::default() };
        }

        let region = pixels.region(row_width, Position::new(1, 1), Size::new(2, 2));

        let mut rows = region.rows();
        assert_eq!(rows.next().unwrap(), vec![p!(5), p!(6)].as_slice());
        assert_eq!(rows.next().unwrap(), vec![p!(9), p!(10)].as_slice());
    }

    #[test]
    fn write_region() {
        let to = vec![
            p!(0), p!(0), p!(0), p!(0),
            p!(0), p!(0), p!(0), p!(0),
            p!(0), p!(0), p!(0), p!(0),
            p!(0), p!(0), p!(0), p!(0),
        ];

        let from = vec![
            p!(0), p!(0), p!(0), p!(0),
            p!(0), p!(0), p!(0), p!(0),
            p!(1), p!(2), p!(3), p!(0),
            p!(4), p!(5), p!(6), p!(0),
        ];

        let row_width = 4;
        let mut to_pixels = Pixels::new(to);
        let from_pixels = Pixels::new(from);

        let position = Position::new(0, 2);
        let region = from_pixels.region(row_width, position, Size::new(3, 2));
        to_pixels.write_region(row_width, position, region);

        assert_eq!(from_pixels.as_bytes(), to_pixels.as_bytes());
    }
}

