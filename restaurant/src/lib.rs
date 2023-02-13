// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() { println!("add_to_wishlist"); }
        // pub fn seat_at_table() { println!("seat_at_table"); }
    }

    mod serving {
        // fn take_order() { println!("take_order"); }
        // fn serve_order() { println!("serve_order"); }
        // fn take_payment() { println!("take_payment"); }
    }
}

pub fn eat_at_restaurant() {
    //Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    //Relative path
    front_of_house::hosting::add_to_waitlist();
}
