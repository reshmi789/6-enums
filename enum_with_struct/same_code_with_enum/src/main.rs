fn main() {
    
    enum IpAddr {
        V4(String),
        V6(String),
    }

    /*
    We attach data to each variant of the enum directly, 
    so there is no need for an extra struct.
    the name of each enum variant that we define also becomes a 
    function that constructs an instance of the enum.

    IpAddr::V4() is a function call that takes a String argument 
    and returns an instance of the IpAddr type. 
    We automatically get this constructor function defined as a result 
    of defining the enum
    */

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));
    
    println!("Hello, world!");
}
