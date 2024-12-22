//External References
use std::{thread::{self}, time::Duration};
use tracing::{span, info, Level};

//Module Declarations
mod args;
mod settings;

//Internal References
use settings::SETTINGS;

fn main() {

    //setting up console tracing subscriber
    tracing_subscriber::fmt::init();

    //dummy call to invoke lazy call for ARGS 
    let _ = args::ARGS.env;

    info!("Program started with following arguments:\n{:#?}", args::ARGS);

    //demonstrate setting values used
    let data_cached = SETTINGS.get_bool("data.cached").unwrap();
    let data_cache_location = SETTINGS.get_string("data.cache.location").unwrap();
    info!("Data: Cached = {data_cached}, Location =  {data_cache_location}");

    let mut thread_handles = Vec::new();

    for i in 1..=5 {
        info!("Inititing task #{}", i);
        let thread_handle = thread::spawn(move || {

            //create tracing span to output task id in each task events 
            let span = span!(Level::INFO, "Task Processing", task_id = i);
            let _task_guard = span.enter();
            
            info!("Starting to process task");

            let dummy_processing_time = rand::random::<u16>() % (1000 as u16);
            thread::sleep(Duration::from_millis(dummy_processing_time as u64));
            
            info!("Task ran on thread {:?} for {} milli-seconds", thread::current().id(), dummy_processing_time);

            info!("Completed processing task")
            
        });
        thread_handles.push(thread_handle);
    }

    for handle in thread_handles {
        handle.join().unwrap();
    }

    info!("Program Ended");

}
