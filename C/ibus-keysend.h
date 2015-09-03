
#include <ibus.h>

/* 
 * Change key settings as you need.
 *
 * Keysyms example:
 *   "Hiragana_Katakana"  0xFF27
 *   "Zenkaku_Hankaku"    0xFF2A
 *   "Eisu_toggle"        0xFF30
 *
 * Keycode have no sense in this context.
 * Any value you set will work.
 *
 * KEY_MODE is a sum of values of modifier keys shifted on.
 * Shift(1), Ctrl(2) and Alt(4) are modifiers.
 *   Ex. Ctrl(2) + Alt(4) = 6
*/
/* "Alt-L" */
const guint IBUS_SEND_KEY_SYM  = 108;   // = 0x006C
const guint IBUS_SEND_KEY_CODE = 46;    // dummy
const guint IBUS_SEND_KEY_MODE = 4;     // Alt

const gchar *IBUS_SEND_BUS_NAME  = "org.freedesktop.IBus.KKC";
const gchar *IBUS_SEND_OBJ_PATH  = "/org/freedesktop/IBus/Engine/1";
const gchar *IBUS_SEND_INTERFACE = "org.freedesktop.IBus.Engine";
const gchar *IBUS_SEND_METHOD    = "ProcessKeyEvent";

