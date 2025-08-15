fn main() {
    let cat = ("Furry McFurson", 3.5);

    // FIXED: Destructure the `cat` tuple in one statement so that the println works.
    // let /* your pattern here */ = cat;
    let (name, age) = cat;

    println!("{name} is {age} years old");
}
