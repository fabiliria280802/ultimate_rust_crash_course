pub fn print_difference(x: f32, y: f32) {
    println!("Difference between {} and {} is {}", x, y, (x - y).abs());
}

pub fn print_array(a: [f32; 2]) {
    println!("The coordinates are ({}, {})", a[0], a[1]);
}

pub fn ding(x: i32) {
    if x == 13 {
        println!("Ding, you found 13!");
    }else{
        println!("Doesn't work the pub fn ding");
    }
}

pub fn on_off(val: bool) {
    if val {
        println!("Lights are on!");
    }else{
        println!("Doesn't work the pub fn on_off");
    }
}

pub fn print_distance((x,y): (f32, f32)) {
    // Using z.0 and z.1 is not nearly as nice as using x and y.  Lucky for
    // us, Rust supports destructuring function arguments.  Try replacing "z" in
    // the parameter list above with "(x, y)" and then adjust the function
    // body to use x and y.
    /*
    
    before:

    pub fn print_distance(z: (f32, f32)) {
        println!(
            "Distance to the origin is {}",
            ( z.0.powf(2.0) + z.1.powf(2.0) ).sqrt());
    }

     */
    println!(
        "Distance to the origin is {}",
        ( x.powf(2.0) + y.powf(2.0) ).sqrt());
}

