fn hello_world() {
    println!("Hello, world: {}!", rand::random::<char>());
}

fn main() {
    hello_world()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}