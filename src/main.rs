use std::cell::{RefCell};

struct Reservation {
    weight: i32
}

struct Unit {
    capacity: i32
}

trait Sameable {
    fn same(&self, another: &Self) -> bool;
}

impl Sameable for Reservation {
    fn same(&self, another: &Self) -> bool {
        self.weight == another.weight
    }
}

impl Sameable for Unit {
    fn same(&self, another: &Self) -> bool {
        self.capacity == another.capacity
    }
}

trait Fittable<T> {
    fn fits_in(&self, container: T) -> bool;
}

impl Fittable<Unit> for Reservation {
    fn fits_in(&self, container: Unit) -> bool {
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

    fn next(&self) -> Option<usize> {
        let new_cursor = if let Some(mut cursor) = *self.cursor.borrow() {
            cursor += 1;
            if cursor >= self.max {
                None
            } else {
                Some(cursor)
            }
        } else {
            Some(0)
        };
        *self.cursor.borrow_mut() = new_cursor.clone();
        new_cursor
    }

    fn jump(&self, to: Option<usize>) -> Option<usize> {
        let new_cursor = if let Some(to) = to {
            if to >= self.max {
                None
            } else {
                Some(to)
            }
        } else {
            None
        };

        *self.cursor.borrow_mut() = new_cursor.clone();
        new_cursor
    }

    fn skip(&self, hops: usize) -> Option<usize> {
        let new_cursor = if let Some(cursor) = *self.cursor.borrow() {
            cursor as i32 + hops as i32
        } else {
            hops as i32 - 1
        } as usize;
        let new_cursor = if new_cursor >= self.max {
            None
        } else {
            Some(new_cursor)
        };

        *self.cursor.borrow_mut() = new_cursor.clone();
        new_cursor
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
struct Solver2<A: Sameable + Fittable<B>, B: Sameable> {
    xs: Vec<A>,
    ys: Vec<B>,
}

impl<A: Sameable + Fittable<B>, B: Sameable> Solver2<A, B> {
    fn new(xs: Vec<A>, ys: Vec<B>) -> Self {
        Self {xs: xs, ys: ys}
    }


}

fn main() {
    println!("Hello, world!");

    let units = vec![Unit{capacity: 2}, Unit{capacity: 3}, Unit{capacity: 3}, Unit{capacity: 4}];
    let reservations = vec![Reservation{weight: 2}, Reservation{weight: 3}, Reservation{weight: 3}, Reservation{weight: 4}];
    let solver = Solver2::new(reservations, units);

    let rr = RoundRobin::new(3);
    // rr.next();
    rr.skip(5);
    println!("rr state: {:?}", rr.state())
}
