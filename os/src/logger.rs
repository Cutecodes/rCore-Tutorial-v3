use core::fmt::Arguments;

use log::{Level,LevelFilter,Log};

use crate::println;

static LOGGER:Logger = Logger;

pub fn init(){
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(
        match option_env!("LOG"){
            Some("ERROR")=>LevelFilter::Error,
            Some("WARN")=>LevelFilter::Warn,
            Some("INFO")=>LevelFilter::Info,
            Some("DEBUG")=>LevelFilter::Debug,
            Some("TRACE")=>LevelFilter::Trace,
            _=>LevelFilter::Off,
        }
    )
  
}





struct Logger;

impl Log for Logger{
    fn enabled(&self,_metadata:&log::Metadata)->bool{
        true
    }
    
    fn log(&self,record:&log::Record){
        if !self.enabled(record.metadata()){
            return;
        }
       
        print_with_color(
            format_args!("[{:>5}]:{}",record.level(),record.args()),
            get_log_level_color(record.level())
        )
    }
    
    fn flush(&self){}
}

fn print_with_color(content:Arguments,color:usize){
    println!("\x1b[{}m{}\x1b[0m",color,content);
}

fn get_log_level_color(level:Level)->usize{
    match level{
       Level::Error => 31,
       Level::Warn =>93,
       Level::Info =>34,
       Level::Debug =>32,
       Level::Trace =>90, 
    }
}
