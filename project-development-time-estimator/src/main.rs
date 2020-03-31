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
