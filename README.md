# Ackermann-function

## Table of Contents

- [Getting Started](#getting_started)
- [Installing](#installing)
- [Usage](#usage)

## Getting Started <a name = "getting_started"></a>

Clone the repo and run it.

```sh
make start
```

### Prerequisites

Have rust installed and cargo

### Installing <a name = "installing"></a>

Clone the repo

```
git clone github.com/mozart409/ackermann-function
```

## Usage <a name = "usage"></a>

```sh
make start
```

## Output

```sh
    Finished release [optimized] target(s) in 0.44s
./target/release/ackermann-function
Hello, world!
0 0 1 0ms
0 1 2 0ms
0 2 3 0ms
0 3 4 0ms
0 4 5 0ms
1 0 2 0ms
1 1 3 0ms
1 2 4 0ms
1 3 5 0ms
1 4 6 0ms
2 0 3 0ms
2 1 5 0ms
2 2 7 0ms
2 3 9 0ms
2 4 11 0ms
3 0 5 0ms
3 1 13 0ms
3 2 29 0ms
3 3 61 0ms
3 4 125 0ms
4 0 13 0ms
4 1 65533 2822ms
```

The next calculation is 4 2, which takes

```sh
2^65533 * 2822ms = 2^65533 * 2.822s = 7.06745182951015090521367772012880215970470436507229813145 × 10^19727s = 2.24107427369043344279987243788965060873436845670734973727 × 10^19720 years
```

The universe will be [crunched](https://en.wikipedia.org/wiki/Big_Crunch) before the calculation is finished.
