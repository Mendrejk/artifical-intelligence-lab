use crate::Facility;
use rand::seq::SliceRandom;

pub struct Specimen {
    pub facility: Facility,
    pub fitness: u64,
}

pub struct Population {
    pub specimens: Vec<Specimen>,
}

impl Population {
    pub fn select_by_tournament(&self, tournament_size: u64) -> Result<&Specimen, &'static str> {
        if tournament_size as usize >= self.specimens.len() {
            return Err("The tournament size must be less than or equal the total specimen count.");
        }

        let rng = &mut rand::thread_rng();

        self.specimens
            .choose_multiple(rng, tournament_size as usize)
            .min_by(|first, second| first.fitness.cmp(&second.fitness))
            .ok_or("There are no specimens to choose from.")
    }
}
