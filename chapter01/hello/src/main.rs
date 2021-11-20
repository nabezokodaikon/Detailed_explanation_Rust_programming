fn greet_world() {
    println!("Hello, world!");
    let southern_germany = "GGGGG";
    let japan = "ハロー・ワールド";
    let regions = [southern_germany, japan];
    for region in regions.iter() {
        println!("{}", &region);
    }
}

fn main() {
    println!("Hello, world!");
}
