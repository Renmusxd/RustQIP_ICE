use num::Complex;
use num::Float;

pub trait Precision {}

pub struct FeynmanState<P: Precision> {
    substate: FeynmanThreadSafeState<P>,
}

struct FeynmanThreadSafeState<P: Precision> {
    n: u64,
    initial_state: Vec<InitialState<P>>,
}

pub enum InitialState<P: Precision> {
    FullState(Vec<Complex<P>>),
}

impl<P: Precision> FeynmanState<P> {
    /// Calculate the amplitude of a given state.
    pub fn calculate_amplitude(&self, _m: u64) -> Complex<P> {
        todo!()
    }

    fn into_state(self) -> Vec<Complex<P>> {
        (0..1 << self.substate.n)
            .map(|m| self.calculate_amplitude(self.substate.n))
            .collect()
    }
}
