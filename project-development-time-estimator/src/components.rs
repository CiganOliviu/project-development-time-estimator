 /* ** Components ** */

pub struct Components {

    _optimistic_estimate : f64,
    _nominal_estimate : f64,
    _pesimistic_estimate : f64,
}

impl Components {

    pub fn add_data (op_estimation: f64, no_estimation: f64, pe_estimation: f64) -> Components {
        Components {
            _optimistic_estimate : op_estimation,
            _nominal_estimate : no_estimation,
            _pesimistic_estimate : pe_estimation
        }
    }

    pub fn get_full_data (&self) {
        println!("{} {} {}", self._optimistic_estimate, self._nominal_estimate, self._pesimistic_estimate)
    }

    pub fn get_expected_duration (&self) -> f64 {

        let expect_duration = ( self._optimistic_estimate + (4.00 * self._nominal_estimate) + self._pesimistic_estimate ) / 6.00;

        return expect_duration;
    }

    pub fn get_standard_deviation (&self) -> f64 {

        let standard_deviation = (self._pesimistic_estimate - self._optimistic_estimate) / 6.00;

        return standard_deviation;
    }

    pub fn get_general_time_estimation (&self, expected: f64, deviation: f64) -> f64 {

        let general_time_estimation = expected + deviation;

        return general_time_estimation;
    }
}
