//@Descritption:
//  This file exercises structs in Rust.

// ////v1 -- Original concept
// fn main() {
//     let w1 = 30;
//     let h1 = 50;

//     println!(
//         "The area of the rectangel is {} square pixels.",
//         area(w1, h1)
//     );
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// ////v2 -- Refactor with Tuple
// fn main() {
//     let rect1 = (30, 50)

//     println!(
//         "The area of the rectangel is {} square pixels.",
//         area(rect1)
//     );
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// ////v3 -- Refactor with struct
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle { width: 30, height: 50 };

//     println!(
//         "The area of the rectangel is {} square pixels.",
//         area(&rect1)
//     );
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     Rectangle.width * Rectangle.height
// }

// ////v4 -- Add debugging trait
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle { width: 30, height: 50 };

//     println!("rect1 is {:?}", rect1);

//     println!(
//         "The area of the rectangel is {} square pixels.",
//         area(&rect1)
//     );
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     Rectangle.width * Rectangle.height
// }

// ////v5 -- Refactor with struct method for 'area'
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

// fn main() {
//     let rect1 = Rectangle { width: 30, height: 50 };

//     println!("rect1 is {:#?}", rect1);

//     println!(
//         "The area of the rectangel is {} square pixels.",
//         rect1.area()
//     );
// }

////v6 -- Add 'can_hold' method to struct
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

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("rect1 is {:#?}", rect1);
    println!("rect1 is {:#?}", rect2);
    println!("rect1 is {:#?}\n", rect3);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}\n", rect1.can_hold(&rect3));

    println!("The area of the rectangle is {} square pixels.", rect1.area());
    println!("The area of the rectangle is {} square pixels.", rect2.area());
    println!("The area of the rectangle is {} square pixels.", rect3.area());

}
