mod calc {
    const G: f64 = 6.67408e-11;

    fn newton_grav_field(m: u64, r: u64) -> u64 {
        ((G*(m as f64)) as u64)/r.pow(2)
    }
}
