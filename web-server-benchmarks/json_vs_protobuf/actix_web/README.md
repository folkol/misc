# Apache Bench

Some warmup, after that:

~90k vs ~90k qps

$ ab -k -c100 -n 10000 http://localhost:8080/json
Total transferred:      4860000 bytes
HTML transferred:       3530000 bytes
Requests per second:    90983.53 [#/sec] (mean)
Time per request:       1.099 [ms] (mean)
Time per request:       0.011 [ms] (mean, across all concurrent requests)
Transfer rate:          43181.64 [Kbytes/sec] received

Percentage of the requests served within a certain time (ms)
  50%      1
  66%      1
  75%      1
  80%      1
  90%      2
  95%      2
  98%      2
  99%      3
 100%      5 (longest request)

$ ab -k -c100 -n 10000 http://localhost:8080/proto
Total transferred:      4770000 bytes
HTML transferred:       3400000 bytes
Requests per second:    86311.07 [#/sec] (mean)
Time per request:       1.159 [ms] (mean)
Time per request:       0.012 [ms] (mean, across all concurrent requests)
Transfer rate:          40205.45 [Kbytes/sec] received

Percentage of the requests served within a certain time (ms)
  50%      1
  66%      1
  75%      1
  80%      1
  90%      2
  95%      2
  98%      2
  99%      3
 100%      5 (longest request)
