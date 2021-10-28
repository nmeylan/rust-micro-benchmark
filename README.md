# Some micro benchmark

## hashmap_get_vs_vec_find
```
vec 10 find             time:   [3.4933 ns 3.5009 ns 3.5095 ns]
hashmap 10 get          time:   [9.5789 ns 9.5944 ns 9.6115 ns]

vec 100 find            time:   [46.300 ns 46.419 ns 46.553 ns]
hashmap 100 get         time:   [9.5790 ns 9.5981 ns 9.6190 ns]

vec 1000 find           time:   [369.25 ns 373.79 ns 378.33 ns]
hashmap 1000 get        time:   [9.5862 ns 9.6049 ns 9.6261 ns]
```
## rwlock_vs_mutex_vs_regular
```
rwlock                  time:   [10.342 ns 10.382 ns 10.444 ns]
mutex                   time:   [10.319 ns 10.335 ns 10.352 ns]
raw                     time:   [206.56 ps 206.84 ps 207.15 ps]
```