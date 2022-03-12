use crate::facility::Facility;
use crate::flow_parser::parse_flows;

mod facility;
mod facility_layout;
mod flow_parser;

fn main() {
    let facility_flow = parse_flows("data/easy_flow.json", "data/easy_cost.json");
    let facility = Facility::generate_randomised_facility(3, 3, (0..9).collect());
    println!("{:?}", facility.calculate_fitness(facility_flow));
}