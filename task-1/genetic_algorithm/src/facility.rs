use crate::facility_layout::FacilityLayout;

use rand::Rng;

#[derive(Debug)]
pub struct Facility {
    interior: Vec<Vec<Option<i64>>>,
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
                interior[machine_y][machine_x] = Some(generated_count);
                generated_count += 1;
            }
        }

        Facility { interior }
    }

    pub fn calculate_distance(&self, from: i64, to: i64) -> Option<i64> {
        // TODO: do this less imperatively

        // find from
        let mut from_x: Option<usize> = None;
        let mut from_y: Option<usize> = None;

        for i in 0..self.interior.len() {
            let index = self.interior[i]
                .iter()
                .position(|&r| r.is_some() && r.unwrap() == from);
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
            let index = self.interior[i]
                .iter()
                .position(|&r| r.is_some() && r.unwrap() == to);
            if index.is_some() {
                to_x = index;
                to_y = Some(i);

                break;
            }
        }

        match (from_x, from_y, to_x, to_y) {
            (Some(from_x), Some(from_y), Some(to_x), Some(to_y)) => Some(
                ((from_x as isize - to_x as isize).abs() + (from_y as isize - to_y as isize).abs())
                    as i64,
            ),
            _ => None,
        }
    }

    pub fn calculate_fitness(&self, facility_layout: FacilityLayout) -> i64 {
        // TODO: do this less imperatively
        let mut total_fitness: i64 = 0;

        for facility_flow in &facility_layout.facility_flows {
            total_fitness += self
                .calculate_distance(facility_flow.source, facility_flow.dest)
                .unwrap_or(0)
                * facility_flow.amount
                * facility_flow.cost;
        }

        total_fitness
    }
}

#[derive(Debug)]
struct FacilityInterior<T> {
    interior: Vec<Option<T>>,
    height: u32,
    width: u32,
}

impl<T> FacilityInterior<T> {
    pub fn new<P>(interior: Vec<Option<T>>, height: u32, width: u32) -> Self {
        FacilityInterior {
            interior,
            height,
            width,
        }
    }

    pub fn position<P>(&self, predicate: P) -> Option<(u32, u32)>
    where
        P: FnMut(&Option<T>) -> bool,
    {
        self.interior
            .iter()
            .position(predicate)
            .map(|i| (i as u32 / self.width, i as u32 % self.width))
    }
}
