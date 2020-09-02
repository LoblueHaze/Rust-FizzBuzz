fn main() {
  let iter = 1..101;
  let result: Vec<String> = iter.map(|x| {
    if x % 15 == 0 {
      String::from("FizzBuzz")
    } else if x % 3 == 0 {
      String::from("Fizz")
    } else if x % 5 == 0 {
      String::from("Buzz")
    } else {
      x.to_string()
    }
  }).collect();
  println!("{:?}", result)
}