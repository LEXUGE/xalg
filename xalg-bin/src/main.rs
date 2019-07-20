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
use clap::{load_yaml, App};
use lib_xalg::{generate, formula::NeedBrackets::False};

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
    for _ in 1..=num {
        println!(
            "{}",
            generate::<i32>(depth, exponent, coefficient)
                .unwrap()
                .export(False)
        );
    }
}
