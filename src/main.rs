use positioned_io::ReadAt;
use std::fs::File;
use std::env;
use rayon::prelude::*;



fn main(){
    /***
    let mut buf = [0u8; 8];
    let file = File::open("./start_13030998016_count_16384")?;

    // We now read 8 bytes from the offset 10.
    let num_bytes_read = file.read_at(&mut buf, 1)?;
    println!("read {} bytes: {:?}", num_bytes_read, buf);
    Ok(())
    */
    //let mut buf = [0u8; 16384];
    //let mut buf: Vec<u8> = Vec::new();
    //let file = File::open("/data/nfs/1/sealed/s-t00541-34")?;

    // We now read 8 bytes from the offset 10.

    //let file = File::open("/data/nfs/1/sealed/s-t00541-83").unwrap();

    let mut DATA_PATH: String = String::new();
    let mut OFFSET: u64 = 0;

    let mut SIZE: u64 = 0;


    for argument in env::args() {
        if argument.contains("--data-path=") {
            let path_option: Vec<&str> = argument.as_str().split('=').collect();
                DATA_PATH = path_option[1].to_string();
        }
        if argument.contains("--offset=") {
            let offset_option: Vec<&str> = argument.as_str().split('=').collect();
                OFFSET = offset_option[1].parse().unwrap();
        }

        if argument.contains("--size=") {
            let offset_option: Vec<&str> = argument.as_str().split('=').collect();
            SIZE = offset_option[1].parse().unwrap();
        }
            
    }
    let file = File::open(&DATA_PATH).unwrap();
    let mut buf = vec![0; SIZE as usize];
    let bytes_read = file.read_at(OFFSET, &mut buf).unwrap();

    let mut tmp_start = 0;
    let mut start_statistics_empty = false;
    let buf_clone = buf.clone();
    println!("size={},len={},{:?}",SIZE,buf_clone.len(),&buf_clone[OFFSET as usize ..(OFFSET as usize + 100)]);
    for (index,value) in buf.into_iter().enumerate() {
   // for (index,value) in buf.into_iter().enumerate() {
        if value == 0 && start_statistics_empty == false{
            tmp_start = index; 
            start_statistics_empty = true;
        }else if value == 0 && start_statistics_empty == true{
            if index == SIZE as usize - 1 {
                println!("find an empty block,range--({},{}),buf={:?}",tmp_start,index,&buf_clone[tmp_start..(tmp_start+1000)]);
            }
        }else if value != 0 && start_statistics_empty == true  && index - tmp_start >= 10 {
            println!("find an empty block,range ({},{})", tmp_start,index);
            start_statistics_empty = false;
        }else if value != 0 && start_statistics_empty == true  && index - tmp_start < 10{
            start_statistics_empty = false; 
        }else{}

        //println!("index={},tmp_start={},start_statistics_empty={}",index,tmp_start,start_statistics_empty)
    }

}
