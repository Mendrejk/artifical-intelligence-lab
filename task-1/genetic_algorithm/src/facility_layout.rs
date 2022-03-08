#[derive(Debug)]
pub struct FacilityLayout {
    pub facility_flows: Vec<FacilityFlow>
}

#[derive(Debug)]
pub struct FacilityFlow {
    pub source: i64,
    pub dest: i64,
    pub amount: i64,
    pub cost: i64
}
