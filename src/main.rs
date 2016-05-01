
extern crate regex;
extern crate rustc_serialize;
extern crate docopt;
extern crate dbus;

use std::io::prelude::*;
use std::error::Error;
use std::{process, env, fs};
use regex::Regex;
use docopt::Docopt;
use dbus::{Connection, Message};
use dbus::MessageItem::UInt32;

const IBUS_BUSADDR_FILE:   &'static str = ".config/ibus/bus";
const IBUS_SEND_BUS_NAME:  &'static str = "org.freedesktop.IBus.KKC";
const IBUS_SEND_OBJ_PATH:  &'static str = "/org/freedesktop/IBus/Engine/1";
const IBUS_SEND_INTERFACE: &'static str = "org.freedesktop.IBus.Engine";
const IBUS_SEND_METHOD:    &'static str = "ProcessKeyEvent";
const IBUS_SEND_WAIT: i32 = 1;  // [ms]?
const DUMMY_ZERO:     u32 = 0;  // Use for keycodes, which have no sense.

/* Key triplets: [Keysym, Keycode, Modifier-State] */
const KEY_TO_ON:  [u32; 3] = [106, 44, 8];  // Alt-J
const KEY_TO_OFF: [u32; 3] = [108, 46, 8];  // Alt-L

const USAGE: &'static str = "
ibus-keysend - send a key event to the IBus daemon.

Before use, set key shortcuts of IBus-KKC as below,
\"(alt j)\" : \"set-input-mode-hiragana\",
\"(alt l)\" : \"set-input-mode-direct\",
and it works as a mode shifter between Japanese and English input mode,
or use \"key\" subcommand with the <keysym> you need.
Values of KeySym and State can be got by \"xev\".

Usage:
  ibus-keysend [off]
  ibus-keysend on
  ibus-keysend key <keysym> [-m <state>]
  ibus-keysend bus
  ibus-keysend (-h | --help)

Options:
  -h, --help    Show this help.
  [off]         Send \"Alt-L\".
  on            Send \"Alt-J\".
  key           Send a key event as you like.
  <keysym>      The value of key symbol to send.
  <state>       Modifier state:  logical sum of (Shift(1) | Ctrl(4) | Alt(8)).
  bus           Show the name of unix socket to connect with IBus.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_m:     bool,
    arg_keysym: u32,
    arg_state:  u32,
    cmd_off:    bool,
    cmd_on:     bool,
    cmd_key:    bool,
    cmd_bus:    bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                     .and_then(|d| d.decode())
                     .unwrap_or_else(|e| e.exit());

    let address = get_address().unwrap_or_else(|e| panic!("{}", e));
    if args.cmd_bus {
        println!("{}", address);
        process::exit(0);
    }

    let connect = Connection::open_private(&address)
                  .unwrap_or_else(|e| panic!("{}", e.message().unwrap()));
    let message = make_message(args).unwrap_or_else(|e| panic!("{}", e));
    let _ = connect.send_with_reply_and_block(message, IBUS_SEND_WAIT);
}

fn get_address() -> Result<String, Box<Error>> {
    let buff = &mut String::new();
    let file = try!(try!(try!(
                fs::read_dir(format!("{}/{}", try!(env::var("HOME")),
                                              IBUS_BUSADDR_FILE)))
                .nth(0).ok_or("Failed to get the busname file.")));
    let _    = try!(fs::File::open(file.path())).read_to_string(buff);
    let line = try!(buff.lines()
                    .nth(1).ok_or("Lack of 2nd line in the busname file."));

    Ok(try!(try!(try!(
        Regex::new(r"^IBUS_ADDRESS=(.+)$"))
        .captures(line).ok_or("Cannot find address after 'IBUS_ADDRESS='."))
        .at(1).ok_or("Must be unreachable error."))
        .to_string())
}

fn make_message(args: Args) -> Result<Message, Box<Error>> {
    let triplet;
    if args.cmd_on { triplet = KEY_TO_ON }
    else if args.cmd_key {
        let state = if args.flag_m {args.arg_state} else {0u32};
        triplet = [ args.arg_keysym, DUMMY_ZERO, state ]
    }
    else { triplet = KEY_TO_OFF }

    let mut message = try!(Message::new_method_call(IBUS_SEND_BUS_NAME,
                                                    IBUS_SEND_OBJ_PATH,
                                                    IBUS_SEND_INTERFACE,
                                                    IBUS_SEND_METHOD)
                           .ok_or("Fail to Message::new_method_call()."));
    let wrap_key = |y: [u32; 3]| [UInt32(y[0]), UInt32(y[1]), UInt32(y[2])];
    message.append_items(&wrap_key(triplet));
    Ok(message)
}

