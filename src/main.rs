fn main() {
    twelve_days_lyrics();
}
fn twelve_days_lyrics() {
    let days: [&str; 12] = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eight", "ninth", "tenth", "eleventh", "twelfth"];
    let gifts: [&str; 12] = ["A partridge in a pear tree", "Two turtle doves",
     "Three French hens", "Four calling birds",
     "Five golden rings", "Six geese a-laying", "Seven swans a-swimming",
     "Eight maids a-milking", "Nine ladies dancing", "Ten lords a-leaping",
     "Eleven pipers piping", "Twelve drummers drumming"];

    for i in 0..days.len() {
        println!("On the {} day of Christmas, My true love gave to me,", days[i]);

        for j in (0..=i).rev() {
            if j == 0 && i != 0 {
                println!("And {}", gifts[0]);
            } else {
                println!("{}", gifts[j]);
            }
        }
        println!();
    }
}