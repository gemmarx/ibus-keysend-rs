
/*
 *  Send a key event to IBus daemon.
 */

#include <ibus.h>
#include "ibus-keysend.h"

int main (void)
{
    IBusBus *bus = ibus_bus_new ();
    GDBusConnection *connection = ibus_bus_get_connection (bus);
    GError *error = NULL;
    g_dbus_connection_call_sync (connection,
                                 IBUS_SEND_BUS_NAME,
                                 IBUS_SEND_OBJ_PATH,
                                 IBUS_SEND_INTERFACE,
                                 IBUS_SEND_METHOD,
                                 g_variant_new ("(uuu)",
                                                IBUS_SEND_KEY_SYM,
                                                IBUS_SEND_KEY_CODE,
                                                IBUS_SEND_KEY_MODE),
                                 NULL, G_DBUS_CALL_FLAGS_NONE, -1, NULL,
                                 &error);
    return (0);
}

