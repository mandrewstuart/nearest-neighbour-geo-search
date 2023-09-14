# Nearest Neighbor Search Server

Carry out nearest neighbor search using JSON through a web-server application, written in Rustlang in BC, Canada.

## Uses

Great for searching natural language vectors, image vectors, or any other vectors where you want the smallest distance between vectors. It accommodates geo-coordinates which is great for services or goods that a bound to a physical location, whether it's a store or a venue. It only returns an array of ID, vector distance and geographic distance in KMs, so if there's more data than just the ID, that needs to be reconciled with the rest of the data.

## Instructions

Required file format: unexited rows of byte data: 32 bytes for the ID, then some number of f32 for the vector (512 is just the example), then 2 of f32 for the coordinate.

To start: ```cargo run --release ./words_subset.data 8989 512```

Where: ```cargo run --release path_to_data port_to_host vector_size```

Request format is JSON to /search or /search_ann:
```
{"sort_by_vec": 1 or 0,
"geoc": [float],
"vector": [float], (matching the database size)
"geo_threshold": float,
"vec_threshold": float,
"limit_results": int}
```

Response format is JSON:
``` {"items: [{"id": str, "geo_dist": float, "dist": float}, ...]} ```

## Benchmarks

Demo.py should yield results on 10k records in this output using the words_subset.data file:

```
times for exhaustive search
max time taken 0:00:00.023136
min time taken 0:00:00.013079
median time taken 0:00:00.013513
times for approximate search
max time taken 0:00:00.001968
min time taken 0:00:00.001162
median time taken 0:00:00.001391
```

## Caveats

There are commented-out lines related to geo-distance in kilometers. Please feel free to reach out to me if you need some help getting that working. My email is andrew.matte@gmail.comst

I am using the kmeans crate to cluster the vectors so that I can search the cluster centroids in the /search_ann index. That crate relies on packed_simd_2 which I am able to build locally but not on some other machines, which makes the build fail on github's workflows. I am considering changing that package out for another so that more people can use it.


## Some Details

The search runs on a single core, but if multiple queries are executing at the same time, it can use more cores thanks to Actix, the webserver.


## Credits

Made Andrew Matte, with some help from Bob Matte, and a secret third person. Mostly funded by Findhelp Information Service Inc in Toronto, Canada.
