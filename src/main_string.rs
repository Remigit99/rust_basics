// enum Color{
//     Blue,
//     Green
// }

pub fn working_with_strings() {
    struct Person {
        name: String,
        age: i32,
        color: String,
    }

    let persons = vec![
        Person {
            name: "Remi".to_owned(),
            age: 34,
            color: "Green".to_owned(),
        },
        Person {
            name: "Oyin".to_owned(),
            age: 19,
            color: "Pink".to_owned(),
        },
        Person {
            name: "Yemisi".to_owned(),
            age: 28,
            color: "Blue".to_owned(),
        },
        Person {
            name: String::from("Bose"),
            age: 10,
            color: "Blue".to_owned(),
        },
        Person {
            name: String::from("Samson"),
            age: 5,
            color: "Grey".to_owned(),
        },
    ];

    fn print_name_age (pers:&Person){
        println!("This is {}, favorite color:{}", pers.name, pers.color)
    }

    for person in &persons  {

        if person.age >=10{
            // println!("{} is less than or equal to 10 years old", person.name)
            print_name_age(&person);
        }
        
    }
}
