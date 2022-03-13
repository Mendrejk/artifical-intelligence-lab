use crate::facility_layout::FacilityLayout;

use rand::seq::SliceRandom;
use rand::thread_rng;

use std::iter::zip;

#[derive(Debug)]
pub struct Facility {
    interior: FacilityInterior<i64>,
}

impl Facility {
    // generates a new facility with random machine arrangement
    pub fn generate_randomised_facility(y_dim: u64, x_dim: u64, machines: &[i64]) -> Self {
        let interior_size = y_dim * x_dim;

        if (interior_size as usize) < machines.len() {
            panic!("The interior must be able to fit all of the machines!")
        }

        let mut rng = thread_rng();
        let mut shuffled_facility_indices: Vec<u64> = (0..interior_size).collect();
        shuffled_facility_indices.shuffle(&mut rng);

        let interior = zip(shuffled_facility_indices, machines).fold(
            vec![None; interior_size as usize],
            |mut acc_vec: Vec<Option<i64>>, indices| {
                let (facility_index, machine) = indices;

                acc_vec[facility_index as usize] = Some(*machine);
                acc_vec
            },
        );

        Facility {
            interior: FacilityInterior::new(interior, y_dim),
        }
    }

    pub fn calculate_distance(&self, from: i64, to: i64) -> Option<i64> {
        let (from_x, from_y) = self.interior.position(|&machine| machine == Some(from))?;
        let (to_x, to_y) = self.interior.position(|&machine| machine == Some(to))?;

        Some(
            ((from_x as isize - to_x as isize).abs() + (from_y as isize - to_y as isize).abs())
                as i64,
        )
    }

    pub fn calculate_fitness(&self, facility_layout: FacilityLayout) -> i64 {
        facility_layout
            .facility_flows
            .iter()
            .fold(0, |total_fitness, facility_flow| {
                total_fitness
                    + self
                        .calculate_distance(facility_flow.source, facility_flow.dest)
                        .unwrap_or(0)
                        * facility_flow.amount
                        * facility_flow.cost
            })
    }
}

#[derive(Debug)]
struct FacilityInterior<T> {
    interior: Vec<Option<T>>,
    width: u64,
}

impl<T> FacilityInterior<T> {
    pub fn new(interior: Vec<Option<T>>, width: u64) -> Self {
        FacilityInterior { interior, width }
    }

    pub fn position<P>(&self, predicate: P) -> Option<(u64, u64)>
    where
        P: FnMut(&Option<T>) -> bool,
    {
        self.interior
            .iter()
            .position(predicate)
            .map(|i| (i as u64 / self.width, i as u64 % self.width))
    }
}
