mod lifetimes;
mod traits_and_shared_behaviour;

fn main() {
    let l_i32 = vec![1, 2, 5, 8, 99, 6];
    let l_char = vec!['a', 'b', 'z', 'f'];

    let largest_i = largest_element(&l_i32);
    let largest_c = largest_element(&l_char);
    println!(
        "The largest number is {} and the largest char is {}",
        largest_i, largest_c
    );

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    traits_and_shared_behaviour::summarize_stuff();
    lifetimes::lifetimes();
}

fn largest_element<T: PartialOrd>(l: &[T]) -> &T {
    let mut largest = &l[0];
    for elem in l {
        if elem > largest {
            largest = elem;
        }
    }
    largest
}

struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
