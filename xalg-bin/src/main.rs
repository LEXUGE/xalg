// Copyright 2019 LEXUGE
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

// use
use {
    clap::{load_yaml, App},
    lib_xalg::{
        formula::{NeedBrackets::False, OperatorFlag, OperatorFlag::*},
        generate,
        ErrorKind::*,
    },
    std::collections::HashSet,
};

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let depth = matches.value_of("depth").unwrap().parse::<i32>().unwrap();
    let coefficient = matches
        .value_of("coefficient")
        .unwrap()
        .parse::<i32>()
        .unwrap();
    let exponent = matches
        .value_of("exponent")
        .unwrap()
        .parse::<i32>()
        .unwrap();
    let num = matches.value_of("num").unwrap().parse::<i32>().unwrap();
    if num <= 0 {
        println!("`num` <= 0");
        return;
    } else {
    };
    // Get flags
    let mut hashset: HashSet<OperatorFlag> = HashSet::new();
    if matches.is_present("add") {
        hashset.insert(Add);
    } else {
    };
    if matches.is_present("sub") {
        hashset.insert(Sub);
    } else {
    };
    if matches.is_present("mul") {
        hashset.insert(Mul);
    } else {
    };
    if matches.is_present("div") {
        hashset.insert(Div);
    } else {
    };
    if matches.is_present("pow") {
        hashset.insert(Pow);
    } else {
    };
    // generate & export
    for _ in 1..=num {
        println!(
            "{}",
            match generate(depth, exponent, coefficient, &hashset) {
                Err(NoFlag) => {
                    println!("Please provide at least one operator flag.");
                    return;
                }
                Err(WrongNumber) => {
                    println!("Your argument(s) didn't meet the criteria.");
                    return;
                }
                Ok(s) => s,
            }
            .export(False)
        );
    }
}
