pub fn vector_act(){
 
    println!("Welcome to activities on Vector");

    /*struct Items{
        num1: i32,
        num2: i32,
        str1: String,
        num3: i32
    }

    let new_item = Items{
        num1:10,
        num2: 20,
        str1: "thirty",
        num3: 40
    };

    let items_vector = vec![new_item.num1, new_item.num2, new_item.str1, new_item.num3];

    for items in items_vector{
        println!("Items: {}", items)
    }
    */

    let my_numbers = vec![10,20,30,40];

    for num in &my_numbers  {

        match num {
            30 => println!("Number is: Thirty"),
            _ => println!("Number is: {}", num)
        }
        
    }
    println!("The length of my_numbers = {}", my_numbers.len())

}