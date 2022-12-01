use std::fmt::Display;


fn main() {
    let number_list = vec![34,50,25,100,65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y','m','a','q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    let p1 = Point { x:5, y:10.4};
    let p2 = Point {x:"Hello",y:'c'};

    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "Lorem ipsum blah blah"
        ),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list{
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
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

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pait<T> {
    fn new(x: T, y:T) -> Self {
        Self { x,y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest memver is x = {}", self.x);
        } else {
            println!("The largest memeber is y = {}", self.y);
        }
    }
}

