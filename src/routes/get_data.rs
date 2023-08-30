use std::collections::HashMap;
use std::str;
use kmeans::*;
use std::string::String;


pub fn kmeans(input_data: HashMap<String, SearchData>) -> HashMap<String, SearchData> {
    let mut keys_map = Vec::<String>::new();
    let mut vecs_map = Vec::<f32>::new();
    let mut geo_map = Vec::<Vec::<f32>>::new();
    let mut vecs = Vec::<Vec::<f32>>::new();
    for key in input_data.keys() {
        for id in &input_data[key].ids {
            keys_map.push(id.to_owned());
        }
        for vec in &input_data[key].storage {
            vecs.push(vec.to_owned());
            for dim in vec {
                vecs_map.push(dim.to_owned());
            }
        }
        for geo in &input_data[key].geo {
            geo_map.push(geo.to_owned());
        }
    }
    let sample_dims = vecs[0].len();
    let sample_count = (vecs_map.len()/sample_dims) as usize;
    let k = (sample_count as f64).sqrt() as usize;
    let max_iter = 100;

    // Calculate kmeans, using kmean++ as initialization-method
    let kmean = KMeans::new(vecs_map, sample_count, sample_dims);
    let clusters = kmean.kmeans_lloyd(k, max_iter, KMeans::init_kmeanplusplus, &KMeansConfig::default());

    let mut output = HashMap::<String, SearchData>::new();
    let mut count = 0;
    let mut count_2 = 0;
    for assignment in clusters.assignments {
        if output.contains_key(assignment.to_string().as_str()) {
            if let Some(tmp_search_data) = output.get_mut(assignment.to_string().as_str()) {
                tmp_search_data.storage.push(vecs[count].clone());
                tmp_search_data.ids.push(keys_map[count].to_string());
                let mut new_geo = Vec::<f32>::new();
                for coord in geo_map[count].clone().into_iter() {
                    new_geo.push(coord)
                }
                tmp_search_data.geo.push(new_geo);
            }
        } else {
            let mut storage = Vec::<Vec::<f32>>::new();
            let mut ids = Vec::<String>::new();
            let mut geo = Vec::<Vec::<f32>>::new();
            storage.push(vecs[count].to_vec());
            ids.push(keys_map[count].clone());
            geo.push(geo_map[count].clone());
            let num_dims_in_vec = storage[0].len();
            let search_data = SearchData{
                storage: storage,
                ids: ids,
                geo: geo,
                // centroid: clusters.centroids[count]
                centroid: clusters.centroids[(count_2*num_dims_in_vec)..((count_2+1)*num_dims_in_vec)].to_vec()
            };
            output.insert(assignment.to_string(), search_data);
            count_2 += 1;
        }
        count += 1;
    }
    return output;
}


pub fn get_data(filename: String, vec_size: usize) -> HashMap<String, SearchData> {
    // This accepts a filename and returns search data
    // It (used to but it commented out) checks whether there is 2x RAM of the file available
    // If so, it uses th RAM
    // If not, it goes item by item, which is much slower (20x?)
    // let mut sys = sysinfo::System::new();
    // sys.refresh_all();
    // if (sys.total_memory() - sys.used_memory()) / 2 > std::fs::metadata(&filename).unwrap().len() {
    return load_bin_to_vec(&filename, vec_size);
    // } else {
    //     return read_filename_to_data(&filename);
    // }
}

// Data format for search
pub struct SearchData {
    pub storage: Vec<Vec<f32>>,
    pub ids: Vec<String>,
    pub geo: Vec<Vec<f32>>,
    pub centroid: Vec<f32>,
}

fn load_bin_to_vec(vectors_filename: &String, vec_size: usize) -> HashMap<String, SearchData> {
    // This uses 2x the RAM of the filesize temporarily
    // to read in the data to RAM
    let bytes: Vec<u8> = std::fs::read(vectors_filename).unwrap();
    let file_length = bytes.iter().count();
    let id_size = 32;
    let row_size = id_size + vec_size * 4 + 2 * 4;
    let nb_rows: usize = file_length / row_size;
    println!("{}", nb_rows);
    let mut id: &str;
    let mut storage: Vec<f32>;
    let mut geo: Vec<f32>;
    let mut row_start: usize;
    let float_size: usize = 4;
    let mut tmp_u8: [u8; 4];
    let mut vec_start: usize;
    let mut data: HashMap<String, SearchData> = HashMap::new();
    let mut partner_id: String;

    for row_index in 0..(nb_rows) {
        // Read ID
        row_start = row_index * row_size;
        id = str::from_utf8(&bytes[row_start..(row_start + id_size)]).unwrap();

        // Read vector
        storage = Vec::with_capacity(vec_size);

        row_start += id_size;
        for vec_index in 0..vec_size {
            tmp_u8 = [0, 0, 0, 0];
            vec_start = vec_index * 4;
            for u_index in 0..4 {
                tmp_u8[u_index] = bytes[row_start + vec_start + u_index];
            }
            storage.push(f32::from_ne_bytes(tmp_u8));
        }

        // Read coordinates
        row_start += vec_size * float_size;
        geo = Vec::with_capacity(2);
        for vec_index in 0..2 {
            tmp_u8 = [0; 4];
            vec_start = vec_index * 4;
            for u_index in 0..4 {
                tmp_u8[u_index] = bytes[row_start + vec_start + u_index];
            }
            geo.push(f32::from_ne_bytes(tmp_u8));
        }

        // row_start += 8;
        partner_id = "123".to_owned();
        let tmp_pid = partner_id.clone();

        if !data.contains_key(&partner_id.clone()) {
            let mut tmp_ids = Vec::<String>::new();
            let mut tmp_storage = Vec::<Vec<f32>>::new();
            let mut tmp_geo = Vec::<Vec<f32>>::new();
            let mut tmp_centroid = Vec::<f32>::new();
            let mut tmp_data = SearchData {
                ids: tmp_ids,
                storage: tmp_storage,
                geo: tmp_geo,
                centroid: tmp_centroid
            };
            data.insert(partner_id, tmp_data);
        }
        data.get_mut(&tmp_pid).unwrap().ids.push(id.to_owned());
        data.get_mut(&tmp_pid).unwrap().storage.push(storage);
        data.get_mut(&tmp_pid).unwrap().geo.push(geo);
    }

    return data;
}
