//5q
//number type must be choosen by considering the range of type

fn main() {
   let v1 = 251_u16 + 8;
   let v2 = u16::checked_add(251, 8).unwrap();
   println!("{},{}",v1,v2);
}
