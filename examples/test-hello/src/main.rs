// use can be used to bring a module into scope. This is useful when you want to use a module's contents without specifying the full path each time. For example, use hello::greet; would allow you to call greet() instead of hello::greet(). 

use hello::greet;

fn main() {
    hello::greet();
}
