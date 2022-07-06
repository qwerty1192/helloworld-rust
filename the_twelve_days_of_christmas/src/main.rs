fn main() {
    for i in 1..=12 {
        let day = match i{
            1 => "first",
            2 => "second",
            3 => "third",
            4 => "fourth",
            5 => "fifth",
            6 => "sixth",
            7 => "seventh",
            8 => "eighth",
            9 => "ninth",
            10 => "tenth",
            11 => "eleventh",
            12 => "twelfth",
            _ => "",
        };
    
        println!("\nOn the {} day of Christmas\nmy true lov sent to me" , day);
        
        for day in 1..=i{
            let gift_text = match i + 1 - day {
                1 => "a Partridge in a Pear Tree",
                2 => "Two Turtle Doves",
                3 => "Three French Hens",
                4 => "Four Calling Birds",
                5 => "Five Golden Rings",
                6 => "Six Geese a Laying",
                7 => "Seven Swans a Swimming",
                8 => "Eight Maids a Milking",
                9 => "Nine Ladies Dancing",
                10 => "Ten Lords a Leaping",
                11 => "Eleven Pipers Piping",
                12 => "12 Drummers Drumming",
                _ => "",
            };
            if day == i && i != 1{
                println!("and {}", gift_text);
            }else{
                println!("{}", gift_text);
            }
        }
    }
}