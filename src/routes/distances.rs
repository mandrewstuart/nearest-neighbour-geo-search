pub fn dist(vec1: &Vec<f32>, vec2: &Vec<f32>) -> f32 {
    // Euclidean distance
    let mut fl_dist: f32 = 0.0;
    for vec_index in 0..(vec1.len()) {
        fl_dist += (&vec1[vec_index] - vec2[vec_index]).powf(2.0);
    }
    return fl_dist;
}

// pub fn manhattan(vec1: &Vec<f32>, vec2: &Vec<f32>) -> f32 {
//     // Manhattan distance
//     let mut fl_dist: f32 = 0.0;
//     for vec_index in 0..(vec1.len()) {
//         fl_dist += (&vec1[vec_index] - vec2[vec_index]).abs();
//     }
//     return fl_dist;
// }

pub fn haversine(geo1: &Vec<f32>, geo2: &Vec<f32>) -> f32 {
    // geo distance in KMs
    let earth_radius: f32 = 6372.8;
    let conversion_factor: f32 = std::f32::consts::PI / 180.0;
    let lat1: f32 = geo1[0] * conversion_factor;
    let lat2: f32 = geo2[0] * conversion_factor;
    let lon1: f32 = geo1[1] * conversion_factor;
    let lon2: f32 = geo2[1] * conversion_factor;
    let d_lat: f32 = lat2 - lat1;
    let d_lon: f32 = lon2 - lon1;

    let a: f32 = ((d_lat / 2.0) * (d_lat / 2.0)).sin()
        + lat1.cos() * lat2.cos() * ((d_lon / 2.0) * d_lon / 2.0).sin();
    let c: f32 = 2.0 * a.sqrt().asin();
    let output = earth_radius * c;
    return output;
}
