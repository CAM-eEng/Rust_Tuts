fn main() {
    //VARIABLES:
    // N - counts up to 12
    // M - counts down from N
    // Repeating phrase "On the {} day of Xmas, my TL gave to me..."
    // Array[]? to hold all phrases
    
    let carol_arr = [
        "A partridge in a pear tree",
        "Two turtle doves, and",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    let mut n = 1; 

    while n <= 12 {
        println!("On the {} day of xmas, my tl gave to me...", n);
        for m in (0..n).rev() {
            println!("{}", carol_arr[m]);
        }
        n += 1;
    }
}
