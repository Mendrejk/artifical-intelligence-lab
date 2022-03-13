use crate::facility::Facility;
use crate::facility_configuration::FacilityConfig;
use crate::flow_parser::parse_flows;

mod facility;
mod facility_configuration;
mod facility_layout;
mod flow_parser;

fn main() {
    let easy_config = FacilityConfig::get_easy_config().unwrap();

    let facility_flow = parse_flows(easy_config.get_flow_path(), easy_config.get_cost_path());
    let facility = Facility::generate_randomised_facility(&easy_config.dimensions);
    println!("{:?}", facility.calculate_fitness(facility_flow));
}
