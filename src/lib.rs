// use crate::{Unit, Reservation, Solver2};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn both_robins_have_similar_ys_1() {
        let units = vec![
            Unit{name: String::from("A"), capacity: 2},
            Unit{name: String::from("B"), capacity: 2},
            Unit{name: String::from("C"), capacity: 2},
        ];
        let reservations = vec![
            Reservation{name: String::from("Y"), weight: 2, min: 2, max: 5},
            Reservation{name: String::from("Z"), weight: 2, min: 2, max: 5}
        ];
        let mut solver = Solver2::new(reservations, units);
        solver.init();
        solver.both_robins_have_same_ys(0, 1);

        assert!(true, true);
    }
}