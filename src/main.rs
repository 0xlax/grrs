use std::env::args();


fn main() {
    let pattern = args().nth(1).expect("No Pattern Given");
    let path = args().nth(2).expect("No path Given");
    
}