use rand::Rng;
use crate::facility_layout::FacilityLayout;

#[derive(Debug)]
pub struct Facility {
    interior: Vec<Vec<Option<i64>>>
}

impl Facility {
    // generates a new, random facility
    pub fn generate_randomised_facility(y_dim: i64, x_dim: i64, machine_count: i64) -> Self {
        let mut interior: Vec<Vec<Option<i64>>> = vec![vec![None; x_dim as usize]; y_dim as usize];

        // TODO: do this less imperatively
        let mut generated_count = 0;

        let mut rng = rand::thread_rng();

        while generated_count < machine_count {
            let machine_y = rng.gen_range(0..y_dim) as usize;
            let machine_x = rng.gen_range(0..x_dim) as usize;

            if interior[machine_y][machine_x].is_none() {
                interior[machine_y][machine_x] = Some(generated_count.clone());
                generated_count += 1;
            }
        }

        Facility { interior }
    }

    pub fn calculate_distance(&self, from: i64, to: i64) -> i32 {
        // TODO: do this less imperatively

        // find from
        let mut from_x: Option<usize> = None;
        let mut from_y: Option<usize> = None;

        for i in 0..self.interior.len() {
            let index = self.interior[i].iter().position(|&r| r.is_some() && r.unwrap() == from);
            if index.is_some() {
                from_x = index;
                from_y = Some(i);

                break;
            }
        }

        // find to
        let mut to_x: Option<usize> = None;
        let mut to_y: Option<usize> = None;

        for i in 0..self.interior.len() {
            let index = self.interior[i].iter().position(|&r| r.is_some() && r.unwrap() == to);
            if index.is_some() {
                to_x = index;
                to_y = Some(i);

                break;
            }
        }

        (from_x.unwrap() as i32 - to_x.unwrap() as i32).abs()
            + (from_y.unwrap() as i32 - to_y.unwrap() as i32).abs()
    }

    pub fn calculate_fitness(&self, facility_layout: FacilityLayout) -> i64 {
        // TODO: do this less imperatively
        let mut total_fitness = 0;

        for facility_flow in &facility_layout.facility_flows {
            total_fitness +=
                self.calculate_distance(facility_flow.source, facility_flow.dest) as i64
                    * facility_flow.amount
                    * facility_flow.cost;
        }

        total_fitness
    }
}

