
pub fn rust_expression(){
    println!("Welcome to rust expression");

    fn compare_num (numb:i32){
        // let numb = 105;
        let numb2: bool = numb > 100;
         match numb2 {
            true => println!("Its big"),
            false => println!("Its small")
        };

        
    }

    compare_num(108);
}