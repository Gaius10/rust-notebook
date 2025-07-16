fn day(number: &str) -> String {
    match number {
        "One" => String::from("first"),
        "Two" => String::from("second"),
        "Three" => String::from("third"),
        "Four" => String::from("fourth"),
        "Five" => String::from("fifth"),
        "Six" => String::from("sixth"),
        "Seven" => String::from("seventh"),
        "Eight" => String::from("eighth"),
        "Nine" => String::from("ninth"),
        "Ten" => String::from("tenth"),
        "Eleven" => String::from("eleventh"),
        "Twelve" => String::from("twelfth"),
        _ => String::new(),
    }
}

fn main() {
    let numbers = [
        "One",
        "Two",
        "Three",
        "Four",
        "Five",
        "Six",
        "Seven",
        "Eight",
        "Nine",
        "Ten",
        "Eleven",
        "Twelve"
    ];

    let gifts = [
        "partridge in a pear tree", 
        "turtle doves", 
        "French hens", 
        "calling birds", 
        "gold rings", 
        "geese a-laying", 
        "swans a-swimming",
        "maids a-milking",
        "ladies dancing",
        "lords a-leaping",
        "pipers piping",
        "drummers drumming"
    ];

    let mut counter = 1;
    for number in numbers {
        let day = day(number);
        println!("On the {day} day of Christmas,");
        println!("my true love sent to me");

        let mut gift_counter = counter - 1;
        for gift in (&gifts[0..counter]).into_iter().rev() {
            if gift_counter == 0 {
                println!("A {gift}.");
                break;
            }

            println!("{} {gift},", numbers[gift_counter]);
            gift_counter -= 1;
        }

        println!();
        counter += 1;
    }    
}

