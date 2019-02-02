# labirynt-rust

Implementation of algorithm that finds the shortest path from 'S' point to 'K' point in a 10x10 randomised matrix of ones and zeroes.

```
0 0 1 0 1 0 1 1 1 0
1 1 0 0 0 1 0 0 1 1
1 1 K 1 1 0 1 0 0 1
1 0 1 1 0 0 0 1 0 0
0 1 S 1 0 1 1 1 0 0
1 1 1 0 0 0 0 1 0 0
0 1 1 1 0 0 1 1 1 1
1 1 0 1 0 0 1 1 0 0
1 1 1 0 0 0 1 0 0 0
0 0 1 0 0 0 1 0 0 1
Found 130 paths
shortest path to target has 3 nodes
(4,2)(3,2)(2,2)
0 0 1 0 1 0 1 1 1 0
1 1 0 0 0 1 0 0 1 1
1 1 K 1 1 0 1 0 0 1
1 0 1 1 0 0 0 1 0 0
0 1 S 1 0 1 1 1 0 0
1 1 1 0 0 0 0 1 0 0
0 1 1 1 0 0 1 1 1 1
1 1 0 1 0 0 1 1 0 0
1 1 1 0 0 0 1 0 0 0
0 0 1 0 0 0 1 0 0 1
```


Originally written in C++ (as this was the task requirement), but I rewrote it in Rust for practice.

The path can only use '1's ('0' are obstacles).

The algorithm recursively finds all paths ([DFS](https://en.wikipedia.org/wiki/Depth-first_search)), starting from 'S' point, and stores all paths in memory. Then it finds the shortest one that reaches the target ('K') and prints out the path with beautiful colours.


# Usage

Clone the repo and run

```bash
cargo run
```
