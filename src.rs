// runs nearest neighbor search and geo search from coordinates
// search file of rows of UUIDv4 ID, a 512-dim vec of f64s, and 2-dim vec of f64s
// nearest neighbour on the 512-dim vec
// haversine geo distance on the 2-dim vec
// expects 5000 rows per file. could be made more flexible
// currently only outputs nearest ID by vector distance


// [dependencies]
// rand = "*"
// permutation = "0.4.1"

use rand::prelude::*;
use std::io::{self, Write};


fn main() {
    // Make a fake input vector
    let mut input_vec = Vec::<f64>::new();
    let vec_size: usize = 512;
    let mut rng = rand::thread_rng();
    for _ in 0..vec_size {
        input_vec.push(rng.gen());
    }

    // Make a fake geo coordinate
    let mut ge0c: Vec<f64> = Vec::<f64>::new();
    let lat: f64 = rng.gen();
    ge0c.push(lat * 44.0);
    let lon: f64 = rng.gen();
    ge0c.push(lon * 80.0);

    // Read binary data into struct
    let nb_rows = 5000;
    let data = load_bin_to_vec(nb_rows);

    // Calculate distances
    let mut vector_distances = Vec::<i32>::new();
    for index in 0..nb_rows {
        vector_distances.push(euclidean(&(data.storage[index]), &input_vec));
    }
    let mut geo_distances = Vec::<f64>::new();
    for index in 0..nb_rows {
        geo_distances.push(haversine(&ge0c, &(data.storage[index])));
    }

    // Sorting based on smallest distance
    let permutation = permutation::sort(&vector_distances);
    let permuted = permutation.apply_inv_slice(data.ids);

    io::stdout().write(permuted[0].as_bytes());
}

struct Data {
    storage: Vec<Vec<f64>>,
    ids: Vec<String>,
    geo: Vec<Vec<f64>>,
}

fn load_bin_to_vec(nb_rows: usize) -> Data {
    let bytes: Vec<u8> =
        std::fs::read("./data_geo.bin").unwrap();
        // std::fs::read("/home/andrew/Agency/211/vector_search_function/data_geo.bin").unwrap();
    let file_length = bytes.len();

    let mut ids: Vec<String> = Vec::with_capacity(nb_rows);
    let mut storage: Vec<Vec<f64>> = Vec::with_capacity(nb_rows);
    let mut geo: Vec<Vec<f64>> = Vec::with_capacity(nb_rows);

    let num_size = 8;
    // id + vector + geo
    let row_size = 128 + 512 * num_size + 2 * num_size;
    let nb_rows = file_length / row_size;
    for index in 0..nb_rows {
        // Read ID
        ids.push(
            String::from_utf8(bytes[(index * row_size)..(128 + index * row_size)].to_vec())
                .unwrap(),
        );

        // Read vector
        let mut tmp_vec: Vec<f64> = Vec::with_capacity(512);
        for vec_index in 0..512 {
            let mut tmp_u8: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
            for u_index in 0..8 {
                tmp_u8[u_index] = bytes[128 + index * row_size + vec_index * num_size + u_index];
            }
            let val = u64::from_be_bytes(tmp_u8) as f64;
            tmp_vec.push(val)
        }
        storage.push(tmp_vec);

        // Read coordinates
        let mut tmp_vec: Vec<f64> = Vec::with_capacity(2);
        for vec_index in 0..2 {
            let mut tmp_u8: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
            for u_index in 0..8 {
                tmp_u8[u_index] = bytes[128 + index * row_size + vec_index * num_size + u_index];
            }
            let val = u64::from_be_bytes(tmp_u8) as f64;
            tmp_vec.push(val)
        }
        geo.push(tmp_vec);
    }
    return Data {
        storage: storage,
        ids: ids,
        geo: geo,
    };
}

fn euclidean(vec1: &Vec<f64>, vec2: &Vec<f64>) -> i32 {
    let mut total = 0.0;
    for vec_index in 0..512 {
        let tmp_val1 = vec1[vec_index];
        let tmp_val2 = vec2[vec_index];
        total += (tmp_val1 - tmp_val2) * (tmp_val1 - tmp_val2);
    }
    return (total.sqrt() * 1000000.0) as i32;
}

fn haversine(geo1: &Vec<f64>, geo2: &Vec<f64>) -> f64 {
    let earth_radius = 6372.8;
    let conversion_factor = std::f64::consts::PI / 180.0;
    let lat1 = geo1[0] * conversion_factor;
    let lat2 = geo2[0] * conversion_factor;
    let lon1 = geo1[1] * conversion_factor;
    let lon2 = geo2[1] * conversion_factor;
    let d_lat = lat2 - lat1;
    let d_lon = lon2 - lon1;

    let a = ((d_lat / 2.0) * (d_lat / 2.0)).sin()
        + lat1.cos() * lat2.cos() * ((d_lon / 2.0) * d_lon / 2.0).sin();
    let c = 2.0 * a.sqrt().asin();
    return earth_radius * c;
}
~              
