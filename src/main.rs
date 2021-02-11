use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone)]
struct Reservation {
    name: String,
    min: i32,
    max: i32,
    weight: i32,
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone)]
struct Unit {
    name: String,
    capacity: i32
}

trait Similarable {
    fn similar(&self, another: &Self) -> bool;
}

impl Similarable for Reservation {
    fn similar(&self, another: &Self) -> bool {
        self.min == another.min && self.max == another.max && self.weight == another.weight
    }
}

impl Similarable for Unit {
    fn similar(&self, another: &Self) -> bool {
        self.capacity == another.capacity
    }
}


trait Overlappable {
    fn overlaps(&self, another: &Self) -> bool;
}

impl Overlappable for Reservation {
    fn overlaps(&self, another: &Self) -> bool {
        self.min <= another.min && self.max >= another.min ||
        self.max >= another.max && self.min <= another.max ||
        self.min >= another.min && self.max <= another.max ||
        self.min <= another.min && self.max >= another.max
    }
}


trait Fittable<T> {
    fn fits_in(&self, container: &T) -> bool;
}

impl Fittable<Unit> for Reservation {
    fn fits_in(&self, container: &Unit) -> bool {
        self.weight <= container.capacity
    }
}

#[derive(Debug)]
struct RoundRobin {
    max: usize,
    cursor: RefCell<Option<usize>>
}

impl RoundRobin {
    fn new(max: usize) -> Self {
        Self { max: max, cursor: RefCell::new(None)}
    }

    /// Turns a robin and returns true if made a full round
    fn next(&self) -> bool {
        let mut turned = false;
        let new_cursor = if let Some(mut cursor) = *self.cursor.borrow() {
            cursor += 1;
            if cursor >= self.max {
                turned = true;
                None
            } else {
                Some(cursor)
            }
        } else {
            Some(0)
        };
        *self.cursor.borrow_mut() = new_cursor.clone();
        turned
    }

    fn jump(&self, to: Option<usize>) -> bool {
        let mut turned = false;
        let new_cursor = if let Some(to) = to {
            if to >= self.max {
                turned = true;
                None
            } else {
                Some(to)
            }
        } else {
            turned = true;
            None
        };

        *self.cursor.borrow_mut() = new_cursor.clone();
        turned
    }

    fn skip(&self, hops: usize) -> bool {
        let mut turned = false;
        let new_cursor = if let Some(cursor) = *self.cursor.borrow() {
            cursor as i32 + hops as i32
        } else {
            hops as i32 - 1
        } as usize;
        let new_cursor = if new_cursor >= self.max {
            turned = true;
            None
        } else {
            Some(new_cursor)
        };

        *self.cursor.borrow_mut() = new_cursor.clone();
        turned
    }

    fn state(&self) -> Option<usize> {
        *self.cursor.borrow()
    }
}

// xs are basically reservations any ys are rooms
//y yy
//yyyy yy
//xxxxxxx
// y  yyy
//    y
#[derive(Debug)]
struct Solver2<X: Similarable + Fittable<Y> + Overlappable + Eq + Ord + PartialEq + PartialOrd + Clone + std::fmt::Debug, Y: Similarable + Eq + Ord + PartialEq + PartialOrd + Clone + std::fmt::Debug> {
    xs: Vec<X>,
    ys: Vec<Rc<Y>>,
    cursor: usize,
    max: usize,
    y_links: Vec<Vec<Rc<Y>>>,
    robins: Vec<RoundRobin>,
    similar: Vec<Vec<usize>>,
    overlapping: Vec<Vec<usize>>,
    shortcuts: Vec<Vec<Option<Option<usize>>>>,
}

impl<X: Similarable + Fittable<Y> + Overlappable + Eq + Ord + PartialEq + PartialOrd + Clone + std::fmt::Debug, Y: Similarable + Eq + Ord + PartialEq + PartialOrd + Clone + std::fmt::Debug> Solver2<X, Y> {
    fn new(mut xs: Vec<X>, mut ys: Vec<Y>) -> Self {  
        let max = xs.len();
        xs.sort();
        ys.sort();

        let ys = ys.into_iter().map(|y|Rc::new(y)).collect::<Vec<Rc<Y>>>();

        Self {xs: xs, ys: ys, cursor: 0, max: max, y_links: vec![], robins: vec![], similar: vec![], overlapping: vec![], shortcuts: vec![]}
    }

    fn init(&mut self) {
        self.fill_links_and_robins();
        self.fill_similar_and_overlapping_xs();
        self.fill_similar_ys_shortcuts();
    }

    fn fill_links_and_robins(&mut self) {
        for x in self.xs.iter() {
            let mut links = vec![];
            for y in self.ys.iter(){
                if x.fits_in(y) {
                    links.push(y.clone())
                }
            }
            self.robins.push(RoundRobin::new(links.len()));
            self.y_links.push(links);
        }
    }

    fn both_robins_have_same_ys(&self, i: usize, ii: usize) -> bool {
        return self.y_links[i] == self.y_links[ii]
    }

    fn fill_similar_and_overlapping_xs(&mut self) {
        for (i, x) in self.xs.iter().enumerate() {
            let mut similar_to_this_x = vec![];
            let mut overlapping_this_x = vec![];
            for (ii, x_f) in self.xs.iter().enumerate().skip(i+1) {
                if x_f.similar(x) || x_f.overlaps(x) && self.both_robins_have_same_ys(i, ii) { // Questionable construction (everything after ||)! This construction makes reservations similar if they are overlapping and have same reservation - meaning they are so identical, that can have Rule 2 have applied 
                    similar_to_this_x.push(ii)
                }
                if x_f.overlaps(x) {
                    overlapping_this_x.push(ii)
                }
            }
            self.similar.push(similar_to_this_x);
            self.overlapping.push(overlapping_this_x);
        }
        
    }


    // Gets current y(room) at ith x(reservation) link. Used to find whether current y at current x 
    fn fill_similar_ys_shortcuts(&mut self) {
        for x in self.y_links.iter() {
            let mut xith_shortcuts: Vec<Option<Option<usize>>> = x.iter().map(|_|None).collect(); // Fisrt option if the next ones are the same, second is actually address, where None is empty element, meaning that the ith y has rest of the rooms the same
            if x.len() > 0 {
                let mut last_not_same_i: Option<usize> = None;
                for (yi, y_link) in x.iter().enumerate().rev().skip(1) {
                    // check if yi'th is same as yi+1'th
                    // if so: keep last_not_same_i (as None in the first round) and put this last_not_same_i into yith sortcut
                    // else: put last_not_same_i = Some()
                    if y_link.similar(&x[yi+1])  {
                        xith_shortcuts[yi] = Some(last_not_same_i)
                    } else {
                        last_not_same_i = Some(yi+1);
                    }
                }
            }
            self.shortcuts.push(xith_shortcuts)
        }
    }

    /// Return true if one of the further overlapping robins have same room
    fn further_overlapping_have_same(&self) -> bool {
        let current_state_at_x = self.robins[self.cursor].state();

        if let Some(current_state_at_x) = current_state_at_x {
            for overlapping_i in self.overlapping[self.cursor].iter() {
                if let Some(further_overlapping_state) = self.robins[*overlapping_i].state() {
                    if current_state_at_x == further_overlapping_state {
                        return true
                    }
                }
            }
        }
        false
    }


    /// Returns true if all further robins have 
    fn any_further_similar_xs_state_is_higher_than_current(&self) -> bool {
        let current_state_at_x = self.robins[self.cursor].state();
        if let Some(current_state_at_x) = current_state_at_x {
            for further_similar_i in self.similar[self.cursor].iter() {
                if let Some(further_overlapping_state) = self.robins[*further_similar_i].state() {
                    if further_overlapping_state >= current_state_at_x {
                        return true
                    }
                }
            }
        }
        // else { // drop this else if want earlier similar reservations to be None. But usually u dont want it
        //     for further_similar_i in self.similar[self.cursor].iter() {
        //         if self.robins[*further_similar_i].state().is_some() {}
        //     }
        // }
        false
    }

    fn next(&mut self) -> bool {
        if let Some(robin_s_state) = self.robins[self.cursor].state() {
            if let Some(possible_shortcut) = self.shortcuts[self.cursor][robin_s_state] {
                return self.robins[self.cursor].jump(possible_shortcut)
            } else {
                return self.robins[self.cursor].next()
            }
        } else {
            return self.robins[self.cursor].next()
        }
    }

    /// Returns finish of the algo. Ie when all robins made a turn at least once
    fn step(&mut self) -> bool {
        loop {
            // While we turn a robin under cursor and get a full round turn, basically while it tunes we go next. Search Rule 3 here
            while self.next() {
                self.cursor += 1;
                if self.cursor >= self.robins.len() {
                    return true
                }
            }

            // Rule 1
            if self.further_overlapping_have_same() {
                // let current_combination = self.current_combination();
                // println!("c: {:?}", current_combination.iter().map(|x| x.as_ref().map(|y| format!("{:?}", y)).unwrap_or(String::from("-"))).collect::<Vec<String>>());
                continue
            }


            // Rule 2
            if self.any_further_similar_xs_state_is_higher_than_current() {
                continue
            }

             

            self.cursor = 0;

            return false
        }

        
    }

    fn current_combination(&self) -> Vec<Option<Y>> {
        let mut result = vec![];
        for robin in self.robins.iter() {
            let state = robin.state();

            let chosen_y = if let Some(ii) = state {
                Some((*self.ys[ii]).clone())
            } else {
                None
            };

            result.push(chosen_y);
        }
        result
    }

    fn produce_combinations(&mut self) -> Vec<Vec<Option<Y>>> {
        let mut combinations = vec![];
        while !self.step() {
            combinations.push(self.current_combination())
        }
        return combinations
    }
}

fn main() {
    println!("Hello, world!");

    // let units = vec![
    //     Unit{name: String::from("AA"), capacity: 2},
    //     Unit{name: String::from("BB"), capacity: 3},
    //     Unit{name: String::from("CC"), capacity: 3},
    //     Unit{name: String::from("DD"), capacity: 3},
    //     Unit{name: String::from("EE"), capacity: 4}
    // ];
    // let reservations = vec![
    //     Reservation{name: String::from("QQ"), weight: 2, min: 2, max: 5},
    //     Reservation{name: String::from("WW"), weight: 3, min: 2, max: 5},
    //     Reservation{name: String::from("XX"), weight: 3, min: 2, max: 5},
    //     Reservation{name: String::from("YY"), weight: 3, min: 2, max: 5},
    //     Reservation{name: String::from("ZZ"), weight: 4, min: 6, max: 8}];
    let units = vec![
        Unit{name: String::from("A"), capacity: 2},
        Unit{name: String::from("B"), capacity: 2},
        Unit{name: String::from("C"), capacity: 2},
    ];
    let reservations = vec![
        Reservation{name: String::from("Y"), weight: 2, min: 2, max: 4},
        Reservation{name: String::from("Z"), weight: 2, min: 3, max: 5}
    ];
    let mut solver = Solver2::new(reservations, units);
    solver.init();

    let combinations = solver.produce_combinations();


    println!("{:#?}", solver);

    for combination in combinations.iter() {
        println!("--> {:?}", combination.iter().map(|x| x.as_ref().map(|y| y.name.clone()).unwrap_or(String::from("-"))).collect::<Vec<String>>());
    }

    println!("len combinations: {}", combinations.len());
}
