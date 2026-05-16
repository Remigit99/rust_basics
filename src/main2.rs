
pub fn main2(){

    enum PrimaryColor {
        Red,
        Green,
        Blue,
    }

    let color_red = PrimaryColor::Red;
    let color_green = PrimaryColor::Green;
    let color_blue = PrimaryColor::Blue;

    fn print_color(color: PrimaryColor) {
        match color {
            PrimaryColor::Red => println!("The color is Red"),
            PrimaryColor::Green => println!("The color is Green"),
            PrimaryColor::Blue => println!("The color is Blue"),
        }
    }

    print_color(color_red);
    print_color(color_green);
    print_color(color_blue);

}