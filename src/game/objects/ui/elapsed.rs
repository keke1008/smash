use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub(super) struct Elapsed(u32);

impl Elapsed {
    pub(super) fn increment(&mut self) {
        self.0 += 1;
    }

    pub(super) fn get(&self) -> u32 {
        self.0
    }
}
