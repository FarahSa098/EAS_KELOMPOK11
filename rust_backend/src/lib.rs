mod lookup;

#[no_mangle]
pub extern "C" fn get_sin_from_lookup(angle: f64) -> f64 {
    lookup::lookup_sin(angle)
}

#[no_mangle]
pub extern "C" fn get_cos_from_lookup(angle: f64) -> f64 {
    lookup::lookup_cos(angle)
}
