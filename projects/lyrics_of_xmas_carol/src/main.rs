fn main() {
    let days = [
        "first", 
        "second", 
        "third", 
        "fourth", 
        "fifth", 
        "sixth", 
        "seventh", 
        "eighth", 
        "nineth", 
        "tenth", 
        "eleventh", 
        "twelfth"
    ];
    let amounts = [
        "a", 
        "two", 
        "three", 
        "four", 
        "five", 
        "six", 
        "seven", 
        "eight", 
        "nine", 
        "ten", 
        "eleven", 
        "twelve"
    ];
    let things = [
        "partridge in a pear tree", 
        "turtle doves", 
        "French hens",
        "calling birds",
        "golden rings",
        "geese-a-laying",
        "swans-a-swimming",
        "maids-a-milking",
        "ladies dancing",
        "lords-a-leaping",
        "pipers piping",
        "drummers drumming",
    ];

    for num in 1..=12 {
        println! ("\nOn the {} day of Christmas,\nMy true love gave to me:", 
            days[num-1]);
        for tmp in (0..num).rev() {
            if tmp == 0 && num == 1 {
                println! ("{} {}.", amounts[tmp], things[tmp]);
            }
            if tmp == 0 && num != 1 {
                println! ("and {} {}.", amounts[tmp], things[tmp]);
            }
            if tmp != 0 {
                println! ("{} {},", amounts[tmp], things[tmp]);
            }
        }
    }
}
