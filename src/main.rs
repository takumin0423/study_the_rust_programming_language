fn main() {
    let mut s = String::new();
    let data = "initial contents";
    let s = data.to_string();
    let mut s = "initial contents".to_string();

    s.push_str("bar");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
}
