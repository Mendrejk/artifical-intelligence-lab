struct FacilityLayout {
    facility_flows: Vec<FacilityFlow>
}

struct FacilityFlow {
    source: int,
    dest: int,
    amount: int,
    cost: int
}
