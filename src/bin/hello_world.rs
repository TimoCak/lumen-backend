fn main() {
    let mut x = 5; //stack value
    let s0 = String::from("Hello"); //heap value
    let s1 = "lol";
    let s2 = s1;
    x = 3;
    print!("{} {}",s0,x);
}