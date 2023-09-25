use geo_types::Coord;
use geo_types::CoordFloat;

use crate::{MapCoords, MapCoordsInPlace};

pub trait ToRadians<T: CoordFloat>: Sized + MapCoords<T, T, Output = Self> {
    fn to_radians(&self) -> Self {
        self.map_coords(|Coord { x, y }| Coord {
            x: x.to_radians(),
            y: y.to_radians(),
        })
    }
}
impl<T: CoordFloat, G: MapCoords<T, T, Output = Self>> ToRadians<T> for G {}

pub trait ToRadiansInPlace<T: CoordFloat>: MapCoordsInPlace<T> {
    fn to_radians_in_place(&mut self) {
        self.map_coords_in_place(|Coord { x, y }| Coord {
            x: x.to_radians(),
            y: y.to_radians(),
        })
    }
}
impl<T: CoordFloat, G: MapCoordsInPlace<T>> ToRadiansInPlace<T> for G {}

pub trait ToDegrees<T: CoordFloat>: Sized + MapCoords<T, T, Output = Self> {
    fn to_degrees(&self) -> Self {
        self.map_coords(|Coord { x, y }| Coord {
            x: x.to_degrees(),
            y: y.to_degrees(),
        })
    }
}
impl<T: CoordFloat, G: MapCoords<T, T, Output = Self>> ToDegrees<T> for G {}

pub trait ToDegreesInPlace<T: CoordFloat>: MapCoordsInPlace<T> {
    fn to_degrees_in_place(&mut self) {
        self.map_coords_in_place(|Coord { x, y }| Coord {
            x: x.to_degrees(),
            y: y.to_degrees(),
        })
    }
}
impl<T: CoordFloat, G: MapCoordsInPlace<T>> ToDegreesInPlace<T> for G {}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use approx::assert_relative_eq;
    use geo_types::Line;

    use super::*;

    lazy_static::lazy_static! {
        static ref LINE_DEGREES: Line = Line::new((90.0, 180.), (0., -90.));
        static ref LINE_RADIANS: Line = Line::new((PI / 2., PI), (0., -PI / 2.));
    }

    #[test]
    fn converts_to_radians() {
        assert_relative_eq!(*LINE_RADIANS, LINE_DEGREES.to_radians())
    }

    #[test]
    fn converts_to_radians_in_place() {
        let mut line = *LINE_DEGREES;
        line.to_radians_in_place();
        assert_relative_eq!(*LINE_RADIANS, line)
    }

    #[test]
    fn converts_to_degrees() {
        assert_relative_eq!(*LINE_DEGREES, LINE_RADIANS.to_degrees())
    }

    #[test]
    fn converts_to_degrees_in_place() {
        let mut line = *LINE_RADIANS;
        line.to_degrees_in_place();
        assert_relative_eq!(*LINE_DEGREES, line)
    }
}
