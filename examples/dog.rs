use tlapyen::tlapyen;

tlapyen!("Make a dog type with a name and age. Give it a method called \
          'something_interesting' that returns a string with a fun, \
          creative fact about the dog based on its name and age.");

fn main() {
    let dog = Dog {
        name: "Rover".to_string(),
        age: 5,
    };
    println!("{}", dog.something_interesting());
}
