






use utils;




fn main() {
    
    
    
    let heavy_func = |chunk: u8| {
        println!(":) Doing some heavy operation on chunk .... {:?}", chunk);
        chunk
    };

    

    let result = utils::simd(3985935, heavy_func);
    println!("the result is {:?}", result);



    
}
