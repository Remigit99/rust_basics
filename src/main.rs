// use std::io;

// mod main2;
// mod main_struct;
// mod main_exp;
// mod main_ownership;
// mod main_vector;
mod main_string;


// enum Direction{
//     North,
//     East,
//     West,
//     South,
// }
fn main() {
    // main2::main2();
    // main_struct::struct_practice();
    // main_exp::rust_expression();
    // main_ownership::owner_ship();
    // main_vector::vector_act();
    main_string::working_with_strings();




    // println!("Hello, world!");
    // println!("This is a simple Rust program, by Aderemi Abiodun!");

    /*let last_name = "Aderemi";
    let first_name = "Oyindamola";

    println!("Her name is {} {}", last_name, first_name);*/

    /*const PI:f64 = 3.142;
    let radius: f64 = 7.25;
    let area = radius * PI;
    println!("The area of the circle of radius {} is = {} cmsq.", radius, area);*/

    //============= VOTERS REGISTRATION
    // If else
    /*println!("Hello, Welcome to the Voters Registration Center");
    println!("Please, Enter your name");
    let mut name = String::new();
    println!("Please, Enter your age(Number)");
    let mut age = String::new();

    io::stdin().read_line(&mut name).expect("Something went wrong");
    io::stdin().read_line(&mut age).expect("Something went wrong");

    let age: i32 = age.trim().parse().expect("Please enter a valid number");

    if age >=18{
        println!("Let us start the registration");
    }else {
        println!("Sorry, You are not old enough to do the voters registration");
    }*/

    /* MATCH
    let blood_group = "O+";

    match blood_group  {
        "A+" => println!("You have A+ blood group"),
        "A-" => println!("You have A- blood group"),
        "B+" => println!("You have B+ blood group"),
        "B-" => println!("You have B- blood group"),
        "AB+" => println!("You have AB+ blood group"),
        "AB-" => println!("You have AB- blood group"),
        "O+" => println!("You have O+ blood group"),
        "O-" => println!("You have O- blood group"),
        _ => println!("Invalid blood group"),
    } */

    //    let mut count = 1;

    /*loop {'
     println!("This is loop number {}", count);

     if count == 5 {
         break;
     }
     count += 1;
    }*/

    //   while count <=4 {
    //       println!("This is loop number {}", count);
    //         count += 1;
    //   }

    // let a;
    // a=5;
    // println!("{}", a);

    // fn find_sum (p:i32, q:i32) -> i32{
    //     return p + q;
    // }

    // println!("sum of 2 and 3 is {}", find_sum(2, 3));
    // println!("sum of 2 and 3 is {:?}", find_sum(2, 3));
    // println!("sum of 2 and 3 is {find_sum(2, 3):?}");
    // println!("sum of 2 and 3 is {find_sum(2, 3)}");

    //loop
    // let mut a = 0;

    // loop {
    //     if a==5 {
    //         break;
    //     }

    //     a+=1;
    //     println!("a = {}", a);
    // }

    /*let mut count = 5;

    loop {
        println!("count: {}", count );
           count -= 1;
        if count == 0 {
            break;
        }
    }
    */

    // while a !=5{
    //     println!("a = {}", a);
    //     a+=1;
    // }

    //========* MATCH ====

    // let day = "thursdays";

    /*match day {
        "monday" => println!("Today is {}", day),
        "tuesday" => println!("Today is {}", day),
        "wednesday" => println!("Today is {}", day),
        "thursday" => println!("Today is {}", day),
        "friday" => println!("Today is {}", day),
        "saturday" => println!("Today is {}", day),
        _ =>println!("Something when wrong")

    }*/

    /*
    let is_locked = false;

    match is_locked {
        true => println!("It's true"),
        false => println!("It's false")
        // _ => println!("Something went wrong")
    } */

    //=== ENUM

    // fn wind_way (dir: Direction ) {
    // let dir = Direction::West;
    //     match dir {
    //         Direction::North => println!("The Direction of the wind is North"),
    //         Direction::East => println!("The Direction of the wind is East"),
    //         Direction::West => println!("The Direction of the wind is West"),
    //         Direction::South => println!("The Direction of the wind is South"),
    //     }
    // }

    // wind_way(EAST);

    //**======= GUESS THE NUMBER GAME ======= */
    /* use std::io;

    println!("Welcome to the Guess the Number Game!");
    println!("I'm thinking of a number between 1 and 100. Can you guess it?");

    let mut guessed_number = String::new();
    io::stdin().read_line(&mut guessed_number).expect("Something Went wrong!");
    println!("You guessed: {}", guessed_number.trim()); */
}
