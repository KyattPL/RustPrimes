use std::fs::File;
use std::io::prelude::*;

fn main() {
    
    let mut f = File::open("liczby.txt").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let numbers: Vec<i32> = contents.split_whitespace().filter_map(|w| w.parse().ok()).collect();

    let mut nr_liczby: i32 = 1;
    let mut trzy_czynniki: i32 = 0;

    for number in numbers.iter(){
        let mut rozklady = 0;
        let mut rozklad = 3;
        let mut temp_liczba = *number;

        println!("Liczba nr: {}", nr_liczby);

        while (rozklad < number/2) && (number % 2 != 0) && (temp_liczba > 1){
            if rozklad > (*number as f32).sqrt() as i32 && rozklady == 0{
                break;
            }
            if temp_liczba % rozklad == 0{
                rozklady += 1;
                while temp_liczba % rozklad == 0{
                    temp_liczba = temp_liczba / rozklad;
                }
                if rozklady > 3{
                    break;
                }
                rozklad = 1;
            }
            rozklad += 2;
        }
        nr_liczby += 1;

        if rozklady == 3{
            trzy_czynniki += 1;
        }
    }

    println!("Tyle liczb ma trzy czynniki: {}", trzy_czynniki);

}