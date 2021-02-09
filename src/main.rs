use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Reservation {
    weight: i32
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Unit {
    capacity: i32
}

trait Similarable {
    fn similar(&self, another: &Self) -> bool;
}

impl Similarable for Reservation {
    fn similar(&self, another: &Self) -> bool {
        self.weight == another.weight
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
        self.weight == another.weight
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

struct RoundRobin {
    max: usize,
    cursor: RefCell<Option<usize>>
}

impl RoundRobin {
    fn new(max: usize) -> Self {
        Self { max: max, cursor: RefCell::new(None)}
    }

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
struct Solver2<X: Similarable + Fittable<Y> + Overlappable + Eq + Ord + PartialEq + PartialOrd, Y: Similarable + Eq + Ord + PartialEq + PartialOrd> {
    xs: Vec<X>,
    ys: Vec<Rc<Y>>,
    cursor: usize,
    max: usize,
    y_links: Vec<Vec<Rc<Y>>>,
    robins: Vec<RoundRobin>,
    similar: Vec<Vec<usize>>,
    overlapping: Vec<Vec<usize>>,
}

impl<X: Similarable + Fittable<Y> + Overlappable + Eq + Ord + PartialEq + PartialOrd, Y: Similarable + Eq + Ord + PartialEq + PartialOrd> Solver2<X, Y> {
    fn new(mut xs: Vec<X>, mut ys: Vec<Y>) -> Self {  
        let max = xs.len();
        xs.sort();
        ys.sort();

        let ys = ys.into_iter().map(|y|Rc::new(y)).collect::<Vec<Rc<Y>>>();

        // let (y_links, robins) = Self::produce_links_and_robins(&xs, &ys);

        Self {xs: xs, ys: ys, cursor: 0, max: max, y_links: vec![], robins: vec![], similar: vec![], overlapping: vec![]}
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

    fn fill_similar_and_overlapping_reservations(&mut self) {
        for (i, x) in self.xs.iter().enumerate() {
            let mut similar_to_this_x = vec![];
            let mut overlapping_this_x = vec![];
            for (ii, x_f) in self.xs.iter().enumerate().skip(i+1) {
                if x_f.similar(x) {
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
    fn get_y_at_ith_x(&self, i: usize) {

    }


}

fn main() {
    println!("Hello, world!");

    let units = vec![Unit{capacity: 2}, Unit{capacity: 3}, Unit{capacity: 3}, Unit{capacity: 4}];
    let reservations = vec![Reservation{weight: 2}, Reservation{weight: 3}, Reservation{weight: 3}, Reservation{weight: 4}];
    let mut solver = Solver2::new(reservations, units);
    solver.fill_links_and_robins();
    solver.fill_similar_and_overlapping_reservations();





    if false {
        let rr = RoundRobin::new(3);
        // rr.next();
        rr.skip(5);
        println!("rr state: {:?}", rr.state())
    }
    
    
    
    
}
