fn main() {
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eigth", "ninth", "tenth", "eleventh", "twelfth"];
    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves, and",
        "Three French hens,",
        "Four calling birds,",
        "Five golden rings,",
        "Six geese a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-milking,",
        "Nine ladies dancing,",
        "Ten lords a-leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,",
    ];

    for n in 0..days.len() {
        println!("On the {} day of Christmas,\nmy true love gave to me",
                 days[n]);
        for x in (0..n+1).rev() {
            print!("{}", gifts[x]);
            if x != 0 {
                print!("\n");
            }
        }
        if n == days.len() - 1 {
            println!("!");
        } else {
            println!(".\n");
        }
    }
}
