let average (xs: []f64) = reduce (+) 0.0 xs / r64 (length xs)

entry avg = average
