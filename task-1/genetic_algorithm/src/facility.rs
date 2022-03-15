use crate::facility_layout::FacilityLayout;

use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::facility_configuration::Dimensions;
use std::iter::zip;

#[derive(Debug)]
pub struct Facility {
    interior: FacilityInterior<u64>,
}

impl Facility {
    // generates a new facility with random machine arrangement
    pub fn generate_randomised_facility(dimensions: &Dimensions) -> Self {
        let interior_size = dimensions.height * dimensions.width;

        if (interior_size as usize) < dimensions.machines.len() {
            panic!("The interior must be able to fit all of the machines!")
        }

        let mut rng = thread_rng();
        let mut shuffled_facility_indices: Vec<u64> = (0..interior_size).collect();
        shuffled_facility_indices.shuffle(&mut rng);

        let interior = zip(shuffled_facility_indices, &dimensions.machines).fold(
            vec![None; interior_size as usize],
            |mut acc_vec: Vec<Option<u64>>, indices| {
                let (facility_index, machine) = indices;

                acc_vec[facility_index as usize] = Some(*machine);
                acc_vec
            },
        );

        Facility {
            interior: FacilityInterior::new(interior, dimensions.height),
        }
    }

    pub fn calculate_distance(&self, from: u64, to: u64) -> Option<u64> {
        let (from_x, from_y) = self.interior.position(|&machine| machine == Some(from))?;
        let (to_x, to_y) = self.interior.position(|&machine| machine == Some(to))?;

        Some(
            ((from_x as isize - to_x as isize).abs() + (from_y as isize - to_y as isize).abs())
                as u64,
        )
    }

    pub fn calculate_fitness(&self, facility_layout: &FacilityLayout) -> u64 {
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
