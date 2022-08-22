# Tentley
A toy linear algebra library not nearly as good as something like
[nalgebra](https://nalgebra.org). 

## Motivation
I have written this library with a strong focus on compile time guarantees
of proper sizes. My previous library had the issue of constant `Result`
usage because of dynamic shapes. I would like this library to be
efficient on large, non-sparse matrices.

## Usage
Add the following line to the dependencies of your Cargo.toml. 

```toml
tentley = { git = "https://github.com/HazelTheWitch/tentley" }
```

To create a matrix simply use the `mat![]` command.

```rust
use tentley::prelude::*;

let a = mat![f64;  // Ensures the type is f32, uses into behind the scenes
    -1, 3.0 / 2.0;
     1,  -1
];
```

Or create a vector / row vector with the `vector` / `row_vector` macros.

```rust
use tentley::prelude::*;

let col_v = vector![1.0, 0.25, 0.125];
let row_v = row_vector![1.0, 2.0, 4.0];
```

You can do most basic operations on the matrix, for example.

```rust
use tentley::prelude::*;

let a = mat![f32;
    -1, 3.0 / 2.0;
     1,  -1
];

let b = a.transpose() * a + a;
```

## Comparisons to Other Linear Algebra Libraries
***Worse.***
