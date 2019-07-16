# Benchmarks

*Notice: `palantir` is not completed yet, and the benchmarks will probably change over time*.

## Performance comparison between palantir reverse proxy and NGINX reverse proxy

### Prerequisites

First, install `nginx` if needed:

```shell
sudo apt update
sudo apt install nginx
```

Then, setup an `echo server` and a `reverse proxy` with NGINX. Use [default](../benches/default) for example.

```shell
sudo nano /etc/nginx/sites-available/default
```

Also, modify the [nginx.conf](../benches/nginx.conf) as we did to gain **high
performance with 16 `worker processes` and 1024 `worker connections`**.

```shell
sudo nano /etc/nginx/nginx.conf
```

For benchmark, you are going to need to install [wrk](<https://github.com/wg/wrk/wiki/Installing-Wrk-on-Linux>).

Finally, you are needed to pull the [palantir](<https://github.com/AASAAM/palantir>)
 repository and use [config.toml](../config.toml)
as it is provided to proxy the nginx upstream echo sever.

### Results

The results of *palantir* reverse proxy running on a single Intel(R) Core(TM)
i5-3230M CPU @ 2.60GHz paired with 3.68 GiB RAM:

```shell
$ wrk --latency -t4 -c500 -d30s http://localhost:8080/
Running 30s test @ http://localhost:8080/
  4 threads and 500 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    32.55ms   49.01ms 669.02ms   98.32%
    Req/Sec     4.63k   480.89    13.47k    87.08%
  Latency Distribution
     50%   26.45ms
     75%   29.54ms
     90%   33.44ms
     99%  311.95ms
  543716 requests in 30.10s, 56.00MB read
  Non-2xx or 3xx responses: 1
Requests/sec:  18063.99
Transfer/sec:      1.86MB

```

The results of *nginx* reverse proxy running on the same platform as above:

```shell
$ wrk --latency -t4 -c500 -d30s http://localhost:9065/
Running 30s test @ http://localhost:9065/
  4 threads and 500 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   120.43ms  243.18ms   1.31s    87.54%
    Req/Sec     4.30k   672.91     6.64k    82.26%
  Latency Distribution
     50%   24.40ms
     75%   56.03ms
     90%  504.30ms
     99%    1.02s
  513179 requests in 30.10s, 92.47MB read
  Socket errors: connect 0, read 0, write 0, timeout 6
Requests/sec:  17050.49
Transfer/sec:      3.07MB
```

*palantir* handled ~18.064 HTTP RPS (Requests Per Second), which is a bit higher
than a high-performance *nginx* configuration (17051 RPS).

### Memory

palantir has a small RAM footprint (~0.2-1.1% which is ~28MB). 

To check for **real-time memory usage** by palantir when **`wrk`** benchmark is running:

```shell
ps aux | grep -i palantir
```

Then use the *`target/release/palantir`* **$PID** for:

```shell
grep VmHWM /proc/$PID/status
```

which resulted in:

```shell
VmHWM:       41176 kB
```
