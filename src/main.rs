fn main() {
    let string1=String::from("Hello Patika");
    let string2=String::from("World PAtika");
    let concatenated_string=concatenate_strings(&string1[0..6],&string2[0..5]); 
    println!("{}",concatenated_string);
    
}
fn concatenate_strings(slice1: &str,slice2: &str) -> String{
 let mut result: String=String::from("");
 result.push_str(slice1);
 result.push_str(slice2);
 result
}
