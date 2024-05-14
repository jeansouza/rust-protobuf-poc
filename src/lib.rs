use test::Who;

pub mod test {
  tonic::include_proto!("test");
}

pub fn greet(message: &str, who: Who) {
    println!("{} {}!", message, match who {
        Who::World => "World",
        Who::You => "You",
    });
}
