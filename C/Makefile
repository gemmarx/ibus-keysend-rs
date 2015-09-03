
TARGET := ibus-keysend
SRCS   := ibus-keysend.c
CFLAGS := `pkg-config --cflags --libs ibus-1.0`
CFLAGS += -O2


OBJS   := $(SRCS:%.c=%.o)
CC     := gcc
CFLAGS += -MD

all: $(TARGET)

$(TARGET): $(OBJS)
	$(CC) $(CFLAGS) -o $@ $^

clean:
	rm -rf $(OBJS) *.o *.d

-include *.d

