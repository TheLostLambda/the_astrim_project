mod traits {
    // A trait for objects with mass (which are, consequently, subject to gravitation).
    trait Massive {
        fn get_mass(&self) -> u64;
        /*fn gravitation(&self) -> (f64,f64) {
            let dist = ()
        }
        // Returns net force in (X,Y) format. I can smell the trouble with f64... Make this SciNum soon.
        fn net_gravitation(&self) -> (f64,f64) {
            self.universe
                .iter()
                .fold((0.0,0.0), |(fx,fy), &x| ())
        };*/
    }
}
