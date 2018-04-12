use cores::physics::structs::Point2;

impl Point2 {
    fn dist(&self, other: &Point2) -> f64 {
        ((other.x - self.x).powi(2) + (other.y - self.y).powi(2)).sqrt()
    }
}
