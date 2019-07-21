A library for generating random formulas.
# Features
- Export to LaTeX.
- Control the operators which are involved in generating process.
# Getting Started
```
use {
   lib_xalg::{
       formula::{NeedBrackets::False, OperatorFlag, OperatorFlag::*},
       generate,
   },
   std::collections::HashSet,
};
let hashset = [Add, Sub, Mul, Div, Pow].iter().copied().collect::<HashSet<OperatorFlag>>();
generate(5, 3, 3, &hashset).unwrap().export(False);
```
