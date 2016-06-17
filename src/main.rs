
#[macro_use] extern crate clap;
extern crate dbus;

use std::io::prelude::*;
use std::error::Error;
use std::{io, process, fs};
use clap::{App, ArgMatches};
use dbus::{Connection, Message};
use dbus::MessageItem::UInt32;

const IBUS_BUSADDR_FILE:   &'static str = ".config/ibus/bus";
const IBUS_SEND_BUS_NAME:  &'static str = "org.freedesktop.IBus.KKC";
const IBUS_SEND_OBJ_PATH:  &'static str = "/org/freedesktop/IBus/Engine/1";
const IBUS_SEND_INTERFACE: &'static str = "org.freedesktop.IBus.Engine";
const IBUS_SEND_METHOD:    &'static str = "ProcessKeyEvent";
const IBUS_SEND_WAIT: i32 = 1;  // [ms]?
const DUMMY_ZERO:     u32 = 0;  // Use for keycodes, which have no sense.

// Key triplets: [Keysym, Keycode, Modifier-State]
const KEY_TO_ON:  [u32; 3] = [106, 44, 8];  // Alt-J
const KEY_TO_OFF: [u32; 3] = [108, 46, 8];  // Alt-L

const NONE_001: &'static str = "No such file or directory.";
const NONE_002: &'static str = "Illeagal file format.";
const NONE_003: &'static str = "Cannot create dbus message.";

fn main() {
    let arg_def = load_yaml!("cli.yml");
    let args    = App::from_yaml(arg_def).get_matches();

    match exec(&args) {
        Ok(_)  => process::exit(0),
        Err(e) => {
            writeln!(&mut io::stderr(), "{}", e).unwrap();
            process::exit(1);
        },
    };
}

fn exec(args: &ArgMatches) -> Result<(), Box<Error>> {
    let address = try!(get_address());
    if args.is_present("bus") {
        println!("{}", address);
        return Ok(());
    }

    let connect = try!(Connection::open_private(&address));
    let message = try!(make_message(&args));
    let _ = connect.send_with_reply_and_block(message, IBUS_SEND_WAIT);

    Ok(())
}

fn get_address() -> Result<String, Box<Error>> {
    let buff = &mut String::new();
    let file = try!(try!(try!(
                fs::read_dir(format!("{}/{}", env!("HOME"),
                                              IBUS_BUSADDR_FILE)))
                .nth(0).ok_or(NONE_001)));
    let _    = try!(fs::File::open(file.path())).read_to_string(buff);
    let line = try!(buff.lines().nth(1).ok_or(NONE_002));
    let offs = 1 + try!(line.find('=').ok_or(NONE_002));

    Ok(line[offs ..].to_string())
}

fn make_message(args: &ArgMatches) -> Result<Message, Box<Error>> {
    let triplet;
    if args.is_present("on") { triplet = KEY_TO_ON }
    else if let Some(k) = args.subcommand_matches("key") {
        triplet = [ try!(k.value_of("keysym").unwrap().parse::<u32>()),
                    DUMMY_ZERO,
                    try!(k.value_of("state").unwrap_or("0").parse::<u32>()) ]
    }
    else { triplet = KEY_TO_OFF }

    let mut message = try!(Message::new_method_call(IBUS_SEND_BUS_NAME,
                                                    IBUS_SEND_OBJ_PATH,
                                                    IBUS_SEND_INTERFACE,
                                                    IBUS_SEND_METHOD)
                           .ok_or(NONE_003));
    let wrap_key = |y: [u32; 3]| [UInt32(y[0]), UInt32(y[1]), UInt32(y[2])];
    message.append_items(&wrap_key(triplet));
    Ok(message)
}

