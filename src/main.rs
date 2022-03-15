






use utils;
use log::info;
use std::env;
use std::time::{Duration, Instant};






// NOTE - don't spawn an async task using tokio inside the rust native thread due to threads confliction nature








#[tokio::main]
async fn main(){
    
    env::set_var("RUST_LOG", "trace");
    pretty_env_logger::init();

    
    let heavy_func = |chunk: u8| {
        info!("\t:) Doing some heavy operation on chunk [{:?}]", chunk);
        chunk
    };

    
    
    let mut start = Instant::now();
    match utils::simd(3985935_u32, heavy_func).await{
        Ok(result) => {
            let end = Instant::now();
            let delta = end.duration_since(start);
            let delta_ms = delta.as_secs() as f32 * 1000_f32 + (delta.subsec_nanos() as f32)/1000000 as f32; 
            assert_eq!(3985935_u32, result); //-- it'll panic on not equal condition
            info!("::::: the result with native threads is [it might be different from the input] {:?} | cost : {:?}\n\n", result, delta_ms);
        },
        Err(e) => info!("::::: error in reading chunk caused by {:?}", e),
    };


    
    
    let mut start = Instant::now();
    match utils::simd_tokio(3985935_u32, heavy_func).await{
        Ok(result) => {
            let end = Instant::now();
            let delta = end.duration_since(start);
            let delta_ms = delta.as_secs() as f32 * 1000_f32 + (delta.subsec_nanos() as f32)/1000000 as f32;
            assert_eq!(3985935_u32, result); //-- it'll panic on not equal condition
            info!("::::: the result with tokio green threads is [it might be different from the input] {:?} | cost : {:?}", result, delta_ms);
        },
        Err(e) => info!("::::: error in reading chunk caused by {:?}", e),
    };



    
}
