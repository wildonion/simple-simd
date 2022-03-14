






use utils;
use log::info;
use std::env;







// NOTE - don't spawn an async task using tokio inside the rust native thread due to thread confliction nature
// NOTE - don't use info!(), println!() and chrono inside the thread::spawn() and tokio::spawn() 
//        cause the bytes order will get into a mess due to a time delay to spend inside 
//        the current thread for logging the infos and times out to the console,
//        use assert_eq!() instead.







#[tokio::main]
async fn main(){
    
    env::set_var("RUST_LOG", "trace");
    pretty_env_logger::init();

    
    let heavy_func = |chunk: u8| {
        info!("\t:) Doing some heavy operation on chunk [{:?}]", chunk);
        chunk
    };

    

    match utils::simd(3985935, heavy_func).await{
        Ok(result) => {
            assert_eq!(3985935, result); //-- it'll panic on not equal condition
            info!("::::: the result with native threads is [it might be different from the input] {:?}\n\n", result)
        },
        Err(e) => info!("::::: error in reading chunk caused by {:?}", e),
    };


    

    match utils::simd_tokio(3985935, heavy_func).await{
        Ok(result) => {
            assert_eq!(3985935, result); //-- it'll panic on not equal condition
            info!("::::: the result with tokio green threads is [it might be different from the input] {:?}", result)
        },
        Err(e) => info!("::::: error in reading chunk caused by {:?}", e),
    };



    
}
