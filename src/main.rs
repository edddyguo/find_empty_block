use positioned_io::ReadAt;
use rayon::prelude::*;
use std::env;
use std::fs::File;
use std::sync::{Arc, RwLock};


fn main() {
    let mut DATA_PATH: String = String::new();
    let mut OFFSET: u64 = 0;
    let mut SIZE: u64 = 0;
    let mut PARALLEL: u64 = 2;

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

        if argument.contains("--parallel=") {
            let parallel_option: Vec<&str> = argument.as_str().split('=').collect();
            PARALLEL = parallel_option[1].parse().unwrap();
        }
    }

    let mut window_size = SIZE / PARALLEL;
    let mut last_window_size = SIZE % PARALLEL;
    let file = File::open(&DATA_PATH).unwrap();
    let arc_file = Arc::new(RwLock::new(file));
    println!("all data scope is  start[{},{}]", OFFSET, SIZE);
    rayon::scope(|s| {
        for thread_index in 0..=PARALLEL {
            let thread_start = OFFSET + thread_index *  window_size;
            if thread_index == PARALLEL {
                if last_window_size != 0 {
                    window_size = last_window_size;
                }else{
                    break;
                }
            }
            let thread_start_clone = thread_start.clone();
            let window_size_clone = window_size.clone();
            let thread_index_clone = thread_index.clone();
            let arc_file_2 = arc_file.clone();


            s.spawn(move |_| {
                let mut buf = vec![0; window_size_clone as usize];
                let thread_file = arc_file_2.read().unwrap();
                let bytes_read = thread_file.read_at(thread_start_clone, &mut buf).unwrap();

                let mut tmp_start = 0;
                let mut start_statistics_empty = false;
                let buf_clone = buf.clone();
                println!(
                    "this window start[{},{}],index={}",
                    thread_start_clone,
                    thread_start_clone as usize + buf_clone.len(),
                    thread_index_clone
                );
                for (index, value) in buf.into_iter().enumerate() {
                    if value == 0 && start_statistics_empty == false {
                        tmp_start = index;
                        start_statistics_empty = true;
                    } else if value == 0 && start_statistics_empty == true {
                        if index == SIZE as usize - 1 {
                            println!("find an empty block,range--({},{}),buf={:?}",tmp_start,index,&buf_clone[tmp_start..(tmp_start+index)]);
                        }
                    } else if value != 0 && start_statistics_empty == true && index - tmp_start >= 5
                    {
                        println!("find an empty block,range ({},{}) at thread_index {},buf={:?}", tmp_start, index,thread_index_clone,&buf_clone[tmp_start..(tmp_start+100)]);
                        start_statistics_empty = false;
                    } else if value != 0 && start_statistics_empty == true && index - tmp_start < 5
                    {
                        start_statistics_empty = false;
                    } else {
                    }

                    //println!("index={},tmp_start={},start_statistics_empty={}",index,tmp_start,start_statistics_empty)
                }
            });
        }
    });
    println!("Scanning file finished!");
}
