trait Park {
    fn park(&self); // we can have both custom implementation
    fn dont_park(&self){ // and a default implentation
        println!("please don't park here");
    }
}
struct VehicleInfo {
    make:String,
    model:String,
    year:u16
}
struct Car {
    info:VehicleInfo
}
struct Truck {
    info:VehicleInfo
}

impl Truck {
    fn unload(&self){
        println!("Unloading truck");
    }
}

impl Park for Car {
    fn park(&self){
        println!("Car parked here");
    }
}
impl Park for Truck{
    fn park(&self){
        println!("truck parked here");
    }
    fn dont_park(&self) {
        println!("new impl of don't park here");
    }
}


fn main(){
    let truck = Truck{
        info:VehicleInfo { make: ("hello".to_owned()), model: ("new".to_owned()), year: (12) }
    };
    println!("{}",truck.info.year);
    truck.dont_park();
    truck.park();
}