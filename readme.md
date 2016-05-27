ibus-keysend
----
Send a key event to the IBus daemon (on KKC engine).

Before use, set key shortcuts on IBus-KKC as below,  
"(alt j)" : "set-input-mode-hiragana",  
"(alt l)" : "set-input-mode-direct",  
and it works as a mode shifter between Japanese and English input mode,  
or use "key" subcommand with the "keysym" you need.  
Values of KeySym and State can be got by "xev".  


```sh
# Build:
$ git clone https://github.com/gemmarx/ibus-keysend-rs
$ cd ibus-keysend-rs
$ cargo build --release

The name of command is "ibus-keysend".
Try "target/release/ibus-keysend bus".

# Install:
$ cp target/release/ibus-keysend PATH/TO/BIN/
```


#### Usage
```
  ibus-keysend [off]
  ibus-keysend on
  ibus-keysend key <keysym> [-m <state>]
  ibus-keysend bus
  ibus-keysend (-h | --help)

Options:
  [off]         Send "Alt-L".
  on            Send "Alt-J".
  key           Send a key event as you like.
  <keysym>      The value of key symbol to send.
  <state>       Modifier state:  logical sum of (Shift(1) | Ctrl(4) | Alt(8)).
  bus           Show the name of unix socket to connect with IBus.
```


#### Example
```sh
# Send "Ctrl+O"
$ ibus-keysend key 111 -m 4

# Send "Alt+Shift+@"
$ ibus-keysend key 64 -m 9

# Send "Ctrl+Shift+Space"
$ ibus-keysend key 32 -m 5

# Send "Hiragana_Katakana"
$ ibus-keysend key 65319

# Send "Zenkaku_Hankaku"
$ ibus-keysend key 65322

# Send "Eisu_toggle"
$ ibus-keysend key 65328
```


#### See also
- /usr/share/libkkc/rules/default/keymap/*.json
- /usr/lib/python*/site-packages/ibus/keysyms.py
