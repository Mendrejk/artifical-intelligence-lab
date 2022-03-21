use crate::facility_layout::FacilityLayout;
use std::collections::HashSet;

use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

use crate::facility_configuration::Dimensions;
use std::iter::zip;

#[derive(Debug)]
pub struct Facility {
    interior: Vec<Option<u64>>,
    width: u64,
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
            interior,
            width: dimensions.width,
        }
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

    pub fn crossover(&self, other: &Facility) -> Facility {
        // TODO - check if both interiors are of same dimensions

        let mut rng = rand::thread_rng();
        // crossover takes place on this row, and all to the bottom of it
        // exclude the 0th row, so that crossover always takes place
        let crossover_row = rng.gen_range(1..self.get_height());
        let mut crossover_facility = self.create_crossover(other, crossover_row);

        // normalize the crossover
        // TODO this should probably be in another function?
        let mut uniques_in_parent = self.get_uniques();
        let uniques_in_crossover = crossover_facility.get_uniques();

        uniques_in_parent.retain(|x| !uniques_in_crossover.contains(x));
        uniques_in_parent.shuffle(&mut rng);

        crossover_facility.remove_duplicates(uniques_in_parent);

        crossover_facility
    }

    fn calculate_distance(&self, from: u64, to: u64) -> Option<u64> {
        let (from_x, from_y) = self.position(|&machine| machine == Some(from))?;
        let (to_x, to_y) = self.position(|&machine| machine == Some(to))?;

        Some(
            ((from_x as isize - to_x as isize).abs() + (from_y as isize - to_y as isize).abs())
                as u64,
        )
    }

    fn position<P>(&self, predicate: P) -> Option<(u64, u64)>
    where
        P: FnMut(&Option<u64>) -> bool,
    {
        self.interior
            .iter()
            .position(predicate)
            .map(|i| (i as u64 / self.width, i as u64 % self.width))
    }

    fn get_height(&self) -> u64 {
        (self.interior.len() as u64) / self.width
    }

    fn create_crossover(&self, other: &Facility, crossover_row: u64) -> Facility {
        // TODO less imperatively?
        let mut crossover = Vec::new();

        for i in 0..(crossover_row * self.width) {
            crossover.push(self.interior[(i as usize)]);
        }

        // FIXME usize -> u64 cast potentially unsafe!
        for i in (crossover_row * self.width)..(self.interior.len() as u64) {
            crossover.push(other.interior[(i as usize)]);
        }

        Facility {
            interior: crossover,
            width: self.width,
        }
    }

    fn get_uniques(&self) -> Vec<u64> {
        let mut uniques: Vec<u64> = Vec::new();
        for val in &self.interior {
            if !uniques.contains(&val.unwrap()) {
                uniques.push(val.unwrap());
            }
        }

        uniques
    }

    fn remove_duplicates(&mut self, mut free_machines: Vec<u64>) {
        let mut visited_elems: HashSet<u64> = HashSet::new();

        for i in 0..self.interior.len() {
            let elem = self.interior[i];

            if visited_elems.contains(&elem.unwrap()) {
                self.interior[i] = Some(free_machines[0]);
                free_machines.swap_remove(0);
            }

            visited_elems.insert(elem.unwrap());
        }
    }
}
