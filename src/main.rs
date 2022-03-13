






use utils;




fn main() {
    
    
    
    let multiple = |chunk: u8| {
        println!(":) Doing some heavy operation on chunk .... {:?}", chunk);
        chunk
    };

    

    let result = utils::simd(3985935, multiple);
    println!("the result is {:?}", result);



    
}
