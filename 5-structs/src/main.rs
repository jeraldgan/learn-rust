use std::io::Read;

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    users();
    tuple_structs();
    area()
}

fn users() {
    let mut user1 = User {
        email: String::from("testing@gmail.com"),
        username: String::from("testing"),
        active: true,
        sign_in_count: 1,
    };

    let name = user1.username;
    user1.username = String::from("testing2");

    let user2 = build_user(String::from("testing3@gmail.com"), String::from("testing3"));

    // Struct update syntax
    let user3 = User {
        email: String::from("testing4@gmail.com"),
        ..user2
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn tuple_structs(){
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    // associative function don't get passed self
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn area() {
    let rect = Rectangle {
        width: 30,
        height: 50
    };
    print!("rect is {:#?}", rect);

    let rect1 = Rectangle {
        width: 20,
        height: 30
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    println!(
        "Can rect hold rect1? {}",
        rect.can_hold(&rect1)
    );

    let rect3 = Rectangle::square(5);
}

fn calculate_area(dimensions: &Rectangle) -> u32 {
    dimensions.width * dimensions.height
}
