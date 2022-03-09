use serde_json::Value;

use crate::facility_layout::{FacilityFlow, FacilityLayout};

use std::fs;
use std::iter::zip;

pub fn parse_flows(flow_file_path: &str, cost_file_path: &str) -> FacilityLayout {
    // read the JSON files into strings
    let flow_contents = fs::read_to_string(flow_file_path).unwrap();
    let cost_contents = fs::read_to_string(cost_file_path).unwrap();

    // parse the JSONs
    let flows: Vec<Value> = serde_json::from_str(&flow_contents).unwrap();
    let costs: Vec<Value> = serde_json::from_str(&cost_contents).unwrap();

    // map the JSONs to FacilityFlows
    // TODO use map() instead of doing this imperatively, somehow?
    let mut facility_flows: Vec<FacilityFlow> = Vec::new();

    for i in zip(&flows, &costs) {
        let (flow, cost) = i;
        facility_flows.push(parse_facility_flow(flow, cost));
    }

    FacilityLayout { facility_flows }
}

fn parse_facility_flow(flow: &Value, cost: &Value) -> FacilityFlow {
    FacilityFlow {
        source: flow["source"].as_i64().unwrap(),
        dest: flow["dest"].as_i64().unwrap(),
        amount: flow["amount"].as_i64().unwrap(),
        cost: cost["cost"].as_i64().unwrap(),
    }
}
