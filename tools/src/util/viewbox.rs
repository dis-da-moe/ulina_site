use std::str::FromStr;

use crate::debug;
use glam::Vec2;

#[derive(Clone, Default)]
pub struct Viewbox {
    pub pos: Vec2,
    pub dim: Vec2,
}

impl FromStr for Viewbox {
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let numbers: Vec<f32> = string
            .split(" ")
            .map(|x| x.parse())
            .collect::<Result<Vec<f32>, _>>()
            .map_err(debug!())?;

        if numbers.len() != 4 {
            return Err(format!("Expected 4 numbers but got {}", numbers.len()));
        }

        let numbers = numbers.as_slice();

        Ok(Viewbox {
            pos: (numbers[0], numbers[1]).into(),
            dim: (numbers[2], numbers[3]).into(),
        })
    }
}

impl ToString for Viewbox {
    fn to_string(&self) -> String {
        format!(
            "{} {} {} {}",
            self.pos.x, self.pos.y, self.dim.x, self.dim.y
        )
    }
}
