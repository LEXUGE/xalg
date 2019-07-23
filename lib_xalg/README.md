A library for generating random formulas.
# Features
- Export to LaTeX.
- Control the operators which are involved in generating process.
# Getting Started
```
use {
   lib_xalg::{
       formula::{OperatorFlag, OperatorFlag::*},
       generate,
   },
   std::collections::HashSet,
};
let hashset = [Add, Sub, Mul, Div, Pow].iter().copied().collect::<HashSet<OperatorFlag>>();
println!("{}", generate(5, 3, 3, &hashset).unwrap());
```
