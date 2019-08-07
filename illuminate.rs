use std::env;
use std::process;
use std::io::{Write, Read};
use std::fs::OpenOptions;

mod defs;

fn main()
{   
    let mut args: Vec<String> = env::args().collect();

    let mut bright_up: bool = false;
    let mut offset = 0;
    if args.len() > 1
    {   
        if args[1].starts_with("+")
        {
            args[1].remove(0);
            offset = args[1].parse::<i32>().unwrap();
            bright_up = true;
        } 
        else if args[1].starts_with("-")
        {
            args[1].remove(0);
            offset = args[1].parse::<i32>().unwrap();
        }
        else { usage(); }
    }
    else { usage(); }


    // Read current brightness-value from file
    match OpenOptions::new()
        .create(false)
        .read(true)
        .write(false)
        .open(defs::BRIGHTNESSFILE)
    {
        Ok(ref mut file) => {
            let mut buff = String::new();

            file.read_to_string(&mut buff)
                .expect("Failed to read brightness file");

            let current_value = buff
                .trim()
                .parse::<i32>()
                .unwrap();
            
            // set brightness
            if bright_up { up(current_value, offset); }
            else { down(current_value, offset); }
        },
        Err(err) => { panic!("Failed to open brightness file: {}", err); }
    }
}


fn up(current: i32, offset: i32)
{
    let write_value = current + offset;
    if write_value < defs::MAX { write_brightness(write_value); }
    else { write_brightness(defs::MAX); }
}

fn down(current: i32, offset: i32)
{
    let write_value = current - offset;
    if write_value > 0 { write_brightness(write_value); }
    else { write_brightness(0); }
}

fn write_brightness(value: i32)
{
    match OpenOptions::new()
        .create(false)
        .read(false)
        .write(true)
        .open(defs::BRIGHTNESSFILE)
    {
        Ok(ref mut file) => {
            write!(file, "{}", value).expect("Failed to change brightness");
        },
        Err(err) => { panic!("Failed to open brightness file: {}", err); }
    }
}

fn usage()
{
    println!("Usage:\nilluminate [+-[0-{}]]", defs::MAX);
    process::exit(0);
}