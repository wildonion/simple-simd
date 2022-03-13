






/*
        SAMPLE OUTPUT
        ----------------
        chunk 0 -> 0
        chunk 1 -> 60
                sending updated chunk 0 -channel-> ...
        chunk 2 -> 210
        chunk 3 -> 15
                sending updated chunk 1 -channel-> ...
                sending updated chunk 2 -channel-> ...
                sending updated chunk 3 -channel-> ...
        the result is 3985935
*/




use utils;




fn main() {
    
    
    
    let multiple = |chunk: u8| {
        println!(":) Doing some heavy operation on chunk .... {:?}", chunk);
        chunk
    };

    

    let result = utils::simd(3985935, multiple);
    println!("the result is {:?}", result);



    
}
