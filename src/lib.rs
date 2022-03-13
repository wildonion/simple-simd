



use std::thread;
use std::sync::mpsc;





// ------------------------------ using mpsc job queue channel
// -----------------------------------------------------------------
// -----------------------------------------------------------------
// -----------------------------------------------------------------

pub fn simd<F>(number: u32, ops: F) -> u32 where F: Fn(u8) -> u8 + std::marker::Send + 'static + Clone{ //-- in order to move the F between threads it must be bounded to Send trait
        
        
    let threads = 4; //-- the total number of all packs or chunks containing 8 bits which in this case is 4 cause our number is of type u32
    let (sender, receiver) = mpsc::channel::<u8>();
    let big_end_bytes = number.to_be_bytes(); //-- network bytes - since there are 4 chunks of 8 bits in the context of u32 bits there will be 4 chunks of 8 bits each chunk between 0 up to 255 
    let mut index = 0;
    


    while index < big_end_bytes.len(){
        
        println!("chunk {:?} -> {:?}", index, big_end_bytes[index]);
        let cloned_sender = sender.clone();
        let cloned_ops = ops.clone();
        thread::spawn(move || {
            let new_chunk = cloned_ops(big_end_bytes[index]);
            println!("\tsending updated chunk {:?} -channel-> ... ", index);
            cloned_sender.send(new_chunk).unwrap();
        });
        index+=1

    }

        
    
    let bytes: Vec<u8> = receiver.iter().take(threads).collect(); //-- collecting 4 packs of 8 bits to gather all incoming chunks from the channel
    let boxed_slice = bytes.into_boxed_slice();
    let boxed_array: Box<[u8; 4]> = match boxed_slice.try_into() {
        Ok(arr) => arr,
        Err(o) => panic!("vector length must be 4 but it's {}", o.len()),
    };
    let result = *boxed_array; //-- dereferencing the box pointer to get the value inside of it 
    let final_res = u32::from_be_bytes(result); //-- will create a u32 number from 4 pack of 8 bits 


    final_res

}




