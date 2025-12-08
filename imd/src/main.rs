struct BrowserCommand<T> {
    name: String,
    payload: T,
}
/// Rust does something called `monomorphisation`,
/// When we compile a generic functions, it is eqv to writing same function twice with different args. 
/// Rust takes a generic function and create `n concrete functions` as per requirement 
impl<T> BrowserCommand <T>{
    fn new(name:String, payload:T)->Self{
        BrowserCommand { name, payload }
    }
    fn get_payload(&self)->&T{
        &self.payload
    }
}
impl BrowserCommand <String>{
    fn print_payload(&self){
        println!("{}",self.payload);
    }
}

fn main() {
    let cmd1 = BrowserCommand {
        name: "navigate".to_owned(),
        payload: "https://nikkhil.tech".to_owned(),
    };
    cmd1.print_payload();
    let cm2 = BrowserCommand {
        name: "zoom".to_owned(),
        payload: 200,
    };
    // cm2.print_payload(); this cannot be called as the payload is implemented for concrete type. 
    let cmd3 = BrowserCommand::new("screenshot".to_owned(),"click");
    println!("{}", cmd1.name);
    let p1 = cmd1.get_payload();
    let p2 = cm2.get_payload();
    // see the types of p2 is &i32 and p1 is &String.
    serialise_payload(p1);
    serialise_payload(p2);    
}

fn serialise_payload<T>(payload:T)->String{
    "placeholder".to_owned()
}

// there is no runtime performance cost when we use generics in rust. 
