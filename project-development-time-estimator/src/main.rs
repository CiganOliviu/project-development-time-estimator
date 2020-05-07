/*
MIT License

Copyright (c) 2020 Cigan Oliviu David

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

mod input_stream;
mod components;

fn main() {

    println! ("The optimistic estimate");
    let _optimistic_estimate = input_stream::read_data();
    println! ("The nominal estimate");
    let _nominal_estimate = input_stream::read_data();
    println! ("The pesimistic estimate");
    let _pesimistic_estimate = input_stream::read_data();

    let comp_assign = components::Components::add_data(_optimistic_estimate, _nominal_estimate, _pesimistic_estimate);
    comp_assign.get_full_data();

    println! ("Expected duration");
    println! ("{}", comp_assign.get_expected_duration());

    println! ("Standard deviation");
    println! ("{}", comp_assign.get_standard_deviation());

    println! ("General time estimation");
    println!("{}", comp_assign.get_general_time_estimation(comp_assign.get_expected_duration(), comp_assign.get_standard_deviation() ));
}
