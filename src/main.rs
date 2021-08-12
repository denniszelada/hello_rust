fn greet_world() {
    println!("Hello, world!");
    let southern_germany = "Grus Gott!";
    let guatemala = "Hola, mundo!";
    let regions = [southern_germany, guatemala];
    for region in regions.iter()
    {
        println!("{}", &region);
    }
}

fn main() {
    greet_world();
}
