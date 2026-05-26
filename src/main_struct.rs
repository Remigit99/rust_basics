
pub fn struct_practice(){

 /*    struct ShippingBox{
        depth: f32,
        height: f32,
        width: f32,
        // color: &str
    }

        let new_box  =  ShippingBox{
        depth: 83.25,
        height: 74.12,
        width: 92.43,
        // color: "Brown"
    };

    let box_height = new_box.height;
    println!("The height of the shopping box is {}", box_height);
*/

    // Enum for drink flavours
    enum Flavour {
        Sparkling,
        Sweet,
        Fruity
    }

    // Struct for drink
    struct Drinks {
        flavour: Flavour,
        fluid_ounces: f64
    }

    let my_drink = Drinks{
        flavour: Flavour::Sweet,
        fluid_ounces: 4.5
    } ;

    

 fn print_drink(picked_drink:Drinks){
    match picked_drink.flavour {
       Flavour::Sparkling => println!("The flavour of the drink is sparkling"),
        Flavour::Sweet => println!("The flavour of the drink is sweet"),
        Flavour::Fruity => println!("The flavour of the drink is fruity")
        
    }

    }

    

    print_drink(my_drink);


}