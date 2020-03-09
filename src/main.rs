// Various Spaced Repetition algorithms
mod sm2;

#[cfg(test)]
mod tests {
    use crate::sm2::*; 

    #[test]
    fn sm2_update() {
        let old_ease = 1.8;
        let old_interval = 6;
        let quality = Quality::Three;
        let old_repetitions = 3;
        let (new_interval, new_repetitions, new_ease_factor) = sm2(quality, old_ease, old_repetitions, old_interval);
        // check that sm2 works against hardcoded values
        assert_eq!(new_interval, 11);
        assert_eq!(new_repetitions, 4);
        assert_eq!(new_ease_factor, 1.66);
    }
}

mod simulator {
    use crate::sm2::*;
    // simulates new facts entered into an SRS in at some rate
    // tested and recalled at some rate, specified by the algorithm in question
    // outputs the results as snapshots, which can be displayed or analyzed
   pub fn simulate_sm2 () {
       let mut f = Fact::default(String::from("use the mod, Luke"));
       for i in 1..10 {
           let q = Quality::weighted_random();
           f = Fact::update(f, q);
           println!("{}: quality ({:?}), {:?}", i, q, f);
       }
       println!("{:?}", f);
   }
}

fn main() {
    simulator::simulate_sm2();
}
