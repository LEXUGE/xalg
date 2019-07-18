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

// use(s)
use {
    lib_xalg::polynomial::{NeedBrackets::False, Polynomial},
    criterion::{criterion_group, criterion_main, Criterion},
};

fn bench_generate(c: &mut Criterion) {
    c.bench_function("generate", |b| {
        b.iter(|| {
            Polynomial::generate(10, 10, 20);
        })
    });
}

fn bench_export(c: &mut Criterion) {
    c.bench_function("export", |b| {
        b.iter(|| {
            Polynomial::generate(10, 10, 20).export(False);
        })
    });
}

criterion_group!(benches, bench_generate, bench_export);
criterion_main!(benches);
