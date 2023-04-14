//   Copyright (C) 2021 Jan Tschada (j.tschada@esri.de).
//   
//   This program is free software: you can redistribute it and/or modify
//   it under the terms of the GNU Lesser General Public License as published by
//   the Free Software Foundation, either version 3 of the License, or
//   (at your option) any later version.
//   
//   This program is distributed in the hope that it will be useful,
//   but WITHOUT ANY WARRANTY; without even the implied warranty of
//   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//   GNU Lesser General Public License for more details.
//   
//   You should have received a copy of the GNU Lesser General Public License
//   along with this program.  If not, see <https://www.gnu.org/licenses/>.

mod traits;

use geo::{Coord, MultiPoint, Point, Polygon, GeometryCollection};
use geo::concave_hull::ConcaveHull;
use geo::convex_hull::ConvexHull;
use geo::algorithm::is_convex::IsConvex;

pub struct GeointTool {

}

impl traits::GpTool<bool> for GeointTool {

    fn execute(&self) -> bool {
        true
    }
}

impl traits::GpTool<bool> for Polygon<f64> {

    fn execute(&self) -> bool {
        let concavity = 2.0;
        self.concave_hull(concavity);
        true
    }
}



/// Concave hull implementation
fn concave_hull_points(points: Vec<Point<f64>>) -> Polygon<f64> {
    let multi_point = MultiPoint(points);
    let concavity = 2.0;
    multi_point.concave_hull(concavity)
}

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn build_concave_hull(coordinates: Vec<(f64, f64)>) -> PyResult<Vec<(f64, f64)>> {
    let points = coordinates
        .into_iter()
        .map(|(x, y)| {
            Point::new(x, y)
        }
    ).collect();
    
    let hull = concave_hull_points(points);
    let hull_coordinates = hull
        .exterior().points()
        .map(|point| {
            (point.x(), point.y())
        }
    ).collect();
    Ok(hull_coordinates)
}

#[pymodule]
fn geoint(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(sum_as_string, module)?)?;
    module.add_function(wrap_pyfunction!(build_concave_hull, module)?)?;

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    use super::traits::*;

    #[test]
    fn it_works() {
        let tool = GeointTool {};
        let executed = tool.execute();
        assert_eq!(true, executed);
    }

    #[test]
    fn multipoint_test() {
        let points = vec![
            Point::new(-102.130629, 48.251542),
            Point::new(-102.07351, 48.3250820000001),
            Point::new(-101.20723, 46.253469),
            Point::new(-103.328183, 46.243274),
            Point::new(-101.20723, 46.253469)
        ];

        let hull = concave_hull_points(points);
        assert_eq!(true, hull.exterior().is_convex());
    }
}
