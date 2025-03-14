pub fn say_hello<S>(name: S)
where
    S: std::fmt::Display,
{
    println!("Hello {name}!");
}
