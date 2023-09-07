8080: actix
7000: Jetty
8081: Undertow
8080: GlassFish

$ time -p ab -k -c 20 -n 1000000 localhost:8080/ping
Concurrency Level:      20
Requests per second:    128982.26 [#/sec] (mean)
 100%      4 (longest request)
Concurrency Level:      4
Requests per second:    66980.80 [#/sec] (mean)
 100%      1 (longest request)
$ time -p ab -k -c 20 -n 1000000 localhost:7000/ping
Concurrency Level:      20
Requests per second:    58779.08 [#/sec] (mean)
 100%     30 (longest request)
Concurrency Level:      4
Requests per second:    40908.77 [#/sec] (mean)
 100%      3 (longest request)
$ time -p ab -k -c 20 -n 1000000 localhost:8081/ping
Concurrency Level:      20
Requests per second:    64163.31 [#/sec] (mean)
 100%     38 (longest request)
$ time -p ab -k -c 4 -n 1000000 localhost:8081/ping
Concurrency Level:      4
Requests per second:    36838.04 [#/sec] (mean)
 100%      3 (longest request)
$ time -p ab -k -c 20 -n 1000000 localhost:8080/ping
Concurrency Level:      20
Requests per second:    42413.68 [#/sec] (mean)
 100%    106 (longest request)
$ time -p ab -k -c 4 -n 1000000 localhost:8080/ping
Concurrency Level:      4
Requests per second:    25318.80 [#/sec] (mean)
 100%     10 (longest request)
