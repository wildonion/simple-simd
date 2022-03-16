






use utils;
use log::info;
use std::env;
use std::time::Instant;





// https://github.com/tokio-rs/tokio/discussions/3858
// NOTE - don't spawn an async task using tokio inside the rust native thread due to threads confliction nature
// NOTE - hadnling async task is done using tokio::spawn() method which the task will be solved based on multi threading concept using tokio green threads in the background of the app
// NOTE - sharing and mutating clonable data (Arc<Mutex<T>>) between tokio green and rust native threads is done using message passing protocol like mpsc job queue channel






#[tokio::main]
async fn main(){
    
    env::set_var("RUST_LOG", "trace");
    pretty_env_logger::init();

    
    let heavy_func = |chunk: u8| {
        info!("\t--------Doing some heavy operation on chunk [{:?}]", chunk);
        chunk
    };

    
    
    let start = Instant::now();
    match utils::simd(3985935_u32, heavy_func).await{
        Ok(result) => {
            let end = Instant::now();
            let delta = end.duration_since(start);
            let delta_ms = delta.as_secs() as f32 * 1000_f32 + (delta.subsec_nanos() as f32)/1000000 as f32; 
            // assert_eq!(3985935_u32, result); //-- it'll panic on not equal condition
            info!("::::: the result with native threads is {:?} - [it might be different from the input] - | cost : {:?}\n\n", result, delta_ms);
        },
        Err(e) => info!("::::: error in reading chunk caused by {:?}", e),
    };


    
    
    let start = Instant::now();
    match utils::simd_tokio(3985935_u32, heavy_func).await{
        Ok(result) => {
            let end = Instant::now();
            let delta = end.duration_since(start);
            let delta_ms = delta.as_secs() as f32 * 1000_f32 + (delta.subsec_nanos() as f32)/1000000 as f32;
            // assert_eq!(3985935_u32, result); //-- it'll panic on not equal condition
            info!("::::: the result with tokio green threads is {:?} - [it might be different from the input] - | cost : {:?}", result, delta_ms);
        },
        Err(e) => info!("::::: error in reading chunk caused by {:?}", e),
    };



    
}
