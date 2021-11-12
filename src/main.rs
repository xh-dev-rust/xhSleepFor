use std::env;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;
use regex::{Error, Regex};

fn process_to_sleep(s: &String){
    let reg = Regex::new("^(?P<t>\\d+)(?P<unit>[s|m|h|d])$");
    match reg {
        Ok(v) =>{
            let xx = v.captures(&s);
            match xx {
                Some(capture)=>{
                    let unit = &capture["unit"];
                    let t = &capture["t"];
                    let num = t.parse::<u64>().unwrap();
                    let duration = match unit {
                        "s" =>  Duration::from_secs(num),
                        "m" =>  Duration::from_secs(num * 60),
                        "h" =>  Duration::from_secs(num * 3600),
                        "d" =>  Duration::from_secs(num * 86400),
                        _ => Duration::from_secs(1)
                    };
                    println!("sleep for {}",s);
                    sleep(duration);
                }
                None =>
                    exit(2)
            }
        }
        Err(e) =>
            exit(3)
    }

}

fn main() {
    let args:Vec<String> = env::args().collect();
    if args.len() == 2 {
        let time_to_sleep = &args[1];
        process_to_sleep(time_to_sleep);
    } else {
        println!("xhSleepFor [ns|nm|nh|nd]");
        exit(1)
    }
}
