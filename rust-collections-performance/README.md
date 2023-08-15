# cargo bench

add/vec_capacity/100                thrpt:  [542.11 Melem/s 546.23 Melem/s 550.16 Melem/s]
add/vec/100                         thrpt:  [126.47 Melem/s 128.35 Melem/s 129.97 Melem/s]
add/heap/100                        thrpt:  [79.842 Melem/s 80.872 Melem/s 81.945 Melem/s]
add/btree/100                       thrpt:  [31.217 Melem/s 31.488 Melem/s 31.754 Melem/s]
add/hashset/100                     thrpt:  [19.853 Melem/s 19.977 Melem/s 20.094 Melem/s]
add/linked list/100                 thrpt:  [18.288 Melem/s 18.414 Melem/s 18.536 Melem/s]

add/vec_capacity/10000              thrpt:  [815.63 Melem/s 823.46 Melem/s 831.27 Melem/s]
add/vec/10000                       thrpt:  [524.09 Melem/s 528.05 Melem/s 532.08 Melem/s]
add/heap/10000                      thrpt:  [113.78 Melem/s 114.55 Melem/s 115.28 Melem/s]
add/hashset/10000                   thrpt:  [20.690 Melem/s 20.800 Melem/s 20.906 Melem/s]
add/linked list/10000               thrpt:  [18.438 Melem/s 18.555 Melem/s 18.676 Melem/s]
add/btree/10000                     thrpt:  [15.622 Melem/s 15.698 Melem/s 15.773 Melem/s]

add/vec_capacity/1000000            thrpt:  [513.58 Melem/s 516.50 Melem/s 519.40 Melem/s]
add/vec/1000000                     thrpt:  [195.22 Melem/s 196.21 Melem/s 197.18 Melem/s]
add/heap/1000000                    thrpt:  [46.063 Melem/s 46.291 Melem/s 46.517 Melem/s]
add/linked list/1000000             thrpt:  [16.854 Melem/s 16.942 Melem/s 17.026 Melem/s]
add/hashset/1000000                 thrpt:  [13.508 Melem/s 13.621 Melem/s 13.733 Melem/s]
add/btree/1000000                   thrpt:  [10.984 Melem/s 11.037 Melem/s 11.088 Melem/s]

> contains '1337', mind that it's early in the list

contains vec/hashset/100            thrpt:  [1.0178 Gelem/s 1.0298 Gelem/s 1.0406 Gelem/s]
contains vec/heap/100               thrpt:  [884.30 Melem/s 891.89 Melem/s 899.86 Melem/s]
contains vec/vec/100                thrpt:  [831.68 Melem/s 838.15 Melem/s 844.91 Melem/s]
contains vec/vec_capacity/100       thrpt:  [775.09 Melem/s 781.38 Melem/s 786.96 Melem/s]
contains vec/btree/100              thrpt:  [58.857 Melem/s 59.304 Melem/s 59.724 Melem/s]
contains vec/linked_list/100        thrpt:  [14.271 Melem/s 14.387 Melem/s 14.497 Melem/s]

contains vec/hashset/10000          thrpt:  [17.473 Gelem/s 17.746 Gelem/s 18.002 Gelem/s]
contains vec/vec/10000              thrpt:  [8.5758 Gelem/s 8.6550 Gelem/s 8.7335 Gelem/s]
contains vec/vec_capacity/10000     thrpt:  [8.7819 Gelem/s 8.8516 Gelem/s 8.9194 Gelem/s]
contains vec/heap/10000             thrpt:  [5.9037 Gelem/s 5.9930 Gelem/s 6.0880 Gelem/s]
contains vec/btree/10000            thrpt:  [56.038 Melem/s 56.391 Melem/s 56.738 Melem/s]
contains vec/linked_list/10000      thrpt:  [14.428 Melem/s 14.630 Melem/s 14.786 Melem/s]

contains vec/vec/1000000            thrpt:  [5.3094 Gelem/s 5.3488 Gelem/s 5.3880 Gelem/s]
contains vec/vec_capacity/1000000   thrpt:  [2.8139 Gelem/s 2.8350 Gelem/s 2.8555 Gelem/s]
contains vec/heap/1000000           thrpt:  [813.59 Melem/s 827.17 Melem/s 840.71 Melem/s]
contains vec/hashset/1000000        thrpt:  [704.94 Melem/s 719.40 Melem/s 732.46 Melem/s]
contains vec/btree/1000000          thrpt:  [27.685 Melem/s 28.098 Melem/s 28.502 Melem/s]
contains vec/linked_list/1000000    thrpt:  [11.085 Melem/s 11.208 Melem/s 11.332 Melem/s]

