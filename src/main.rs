#![feature(plugin)]
#![plugin(docopt_macros)]

extern crate dbus;
extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;
use std::io::prelude::*;
use std::{io, fs, process};
use dbus::{Connection, Message};
use dbus::MessageItem::UInt32;

const IBUS_SEND_BUS_NAME:  &'static str = "org.freedesktop.IBus.KKC";
const IBUS_SEND_OBJ_PATH:  &'static str = "/org/freedesktop/IBus/Engine/1";
const IBUS_SEND_INTERFACE: &'static str = "org.freedesktop.IBus.Engine";
const IBUS_SEND_METHOD:    &'static str = "ProcessKeyEvent";
const IBUS_SEND_WAIT: i32 = 1;  // [ms]?
const DUMMY_ZERO:     u32 = 0;  // Use for keycodes, which have no sense.

/* Key triplets: [Keysym, Keycode, Modifier-State] */
const KEY_TO_ON:  [u32; 3] = [106, 44, 4];  // Alt-J
const KEY_TO_OFF: [u32; 3] = [108, 46, 4];  // Alt-L

docopt!(Args derive Debug, "
ibus-keysend - send a key event to the IBus daemon.

Before use, set key shortcuts on IBus-KKC as below,
\"(control j)\" : \"set-input-mode-hiragana\",
\"(control l)\" : \"set-input-mode-direct\",
and it works as a mode shifter between Japanese and English input mode,
or use \"key\" subcommand with the <keysym> you need.

Usage:
  ibus-keysend [off]
  ibus-keysend on
  ibus-keysend key <keysym> [-m <mode>]
  ibus-keysend bus
  ibus-keysend (-h | --help)

Options:
  -h, --help    Show this help.
  [off]         Send \"Alt-L\".
  on            Send \"Alt-J\".
  key           Send a key event as you like.
  <keysym>      The value of key symbol to send.
  <mode>        Modifier mode: logical sum of (Shift(1) | Ctrl(2) | Alt(4)).
  bus           Show the name of unix socket to connect with IBus.
", arg_keysym: u32, arg_mode: u32);

fn main() {
    let args: Args = Args::docopt().decode()
                     .unwrap_or_else(|e| e.exit());

    let address = &(get_address().unwrap());
    if args.cmd_bus {
        println!("{}", address);
        process::exit(0);
    }

    let connect = Connection::open_private(address)
                  .unwrap_or_else(|e| {
                      let _ = io::stderr().write(e.message().unwrap().as_bytes());
                      process::exit(1);
                  } );
    let message = make_message(args).unwrap();
    let _ = connect.send_with_reply_and_block(message, IBUS_SEND_WAIT);
}

fn get_address() -> Option<String> {
    let home = std::env::var("HOME").unwrap();
    let path = fs::read_dir(home + "/.config/ibus/bus").unwrap()
               .next().expect("Failed to get any files in the dir").unwrap()
               .path();
    let buff = &mut String::new();
    let _ = fs::File::open(path).unwrap()
            .read_to_string(buff);
    let line   = buff.lines().nth(1).unwrap();
    let offset = 1 + line.find('=').unwrap();
    Some(line[offset ..].to_string())
}

fn make_message(args: Args) -> Option<Message> {
    let triplet;
    if args.cmd_on { triplet = KEY_TO_ON }
    else if args.cmd_key {
        let state = if args.flag_m {args.arg_mode} else {0u32};
        triplet = [ args.arg_keysym, DUMMY_ZERO, state ]
    }
    else { triplet = KEY_TO_OFF }

    let mut message = Message::new_method_call(IBUS_SEND_BUS_NAME,
                                               IBUS_SEND_OBJ_PATH,
                                               IBUS_SEND_INTERFACE,
                                               IBUS_SEND_METHOD).unwrap();
    let wrap_key = |y: [u32; 3]| {
        [ UInt32(y[0]), UInt32(y[1]), UInt32(y[2]) ]
    };
    message.append_items(&wrap_key(triplet));
    Some(message)
}

