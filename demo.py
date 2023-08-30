from datetime import datetime
from random import random
import requests
import subprocess
import time


outfile = open("outfile_" + str(datetime.now()), "w")

proc = subprocess.Popen(
    ["cargo", "run", "--release", "./words_subset.data", "8989", "512"],
    stdout=outfile,
    stderr=outfile,
)

# here we are doing a bare sleep because we dont' know how long it's going to take to start the webserver.
print("Building application. If it breaks here, build with `cargo build --release` before running this demo.")
time.sleep(10)

VECTOR_DIMENSIONS = 512

# geo-coordinate data, currently not used
geoc = [0.5, 0.75]
geo_threshold = 9999999

# this next line means search by vector instead of by geo-coordinate
sort_by_vec = "1"
vector = [random() for _ in range(VECTOR_DIMENSIONS)]
times = []
# we carry out the search 100 times to test its capacity
data = {
    "sort_by_vec": sort_by_vec,
    "geoc": geoc,
    "vector": vector,
    "geo_threshold": geo_threshold,
    "vec_threshold": 10000000,
    "limit_results": 50,
}
print("Running exhaustive seach 100 times")
for i in range(100):
    start = datetime.now()
    response = requests.post(
        "http://localhost:8989/search",
        json=data,
    )
    end = datetime.now()
    times.append(end - start)

output = response.json()
print("times for exhaustive search")
times.sort(reverse=True)
print("max time taken", times[0])
print("min time taken", times[-1])
print("median time taken", times[int(len(times) / 2)])


print("Running approximate seach 1000 times")
times = []
# we carry out the search 100 times to test its capacity
for i in range(1000):
    start = datetime.now()
    response = requests.post("http://localhost:8989/search_ann", json=data)
    end = datetime.now()
    times.append(end - start)

output = response.json()
print("times for approximate search")
times.sort(reverse=True)
print("max time taken", times[0])
print("min time taken", times[-1])
print("median time taken", times[int(len(times) / 2)])

proc.kill()
