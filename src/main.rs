/*
enum value can only be one of its variants.
*/
enum IpAddrKind {
    V4,
    V6,
}
// instances of each of the two variants of IpAddrKind
let four  = IpAddrKind::v4;
let six = IpAddrKind::v6;


// define a function that takes any IpAddrKind
fn route(ip_kind: IpAddrKind) {}

route(IpAddrKind::V4);
route(IpAddrKind::V6);
// call this function with either variant

/*
Advantages - 
- enum is more concise
- the name of each enum variant that we define also becomes a 
function that constructs an instance of the enum.
- each variant can have different types and amounts of associated data. 
Ex - If we wanted to store V4 addresses as four u8 values but still express V6 addresses as one String value, we wouldn’t be able to with a struct. Enums handle this case with ease

*/

fn main() {
    println!("Hello, world!");
}
