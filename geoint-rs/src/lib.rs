mod traits;

use geo::{Coordinate, Point, Polygon, GeometryCollection};
use geo::concave_hull::ConcaveHull;
use geo::convex_hull::ConvexHull;

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
}
