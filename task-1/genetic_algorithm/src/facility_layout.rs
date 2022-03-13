#[derive(Debug)]
pub struct FacilityLayout {
    pub facility_flows: Vec<FacilityFlow>,
}

#[derive(Debug)]
pub struct FacilityFlow {
    pub source: u64,
    pub dest: u64,
    pub amount: u64,
    pub cost: u64,
}
