SOURCE			:= illuminate.rs
BUILD			:= defs.rs
CFLAGS			:= -C opt-level=z -C lto=yes
TARGET			:= illuminate
COMPILER		:= rustc
INSTALL			:= install
INSTALL_ARGS	:= -o root -g root -m 4755
INSTALL_DIR		:= /usr/local/bin/

BRIGHTNESSFILE = $(shell find /sys/class/backlight/*/brightness | head -1)
MAXVALUE = $(shell find /sys/class/backlight/*/max_brightness | head -1 | xargs cat)

all: clean build

build:
	@echo "pub const MAX: i32 = $(MAXVALUE);"									> $(BUILD)
	@echo "pub const BRIGHTNESSFILE: &'static str = \"$(BRIGHTNESSFILE)\";" 	>> $(BUILD)

	$(COMPILER) $(SOURCE) $(CFLAGS) -o $(TARGET)
	strip --strip-unneeded $(TARGET)

install:
	$(INSTALL) $(INSTALL_ARGS) $(TARGET) $(INSTALL_DIR)

uninstall: clean
	rm -f $(INSTALL_DIR)$(TARGET)

clean:
	rm -f $(TARGET) $(BUILD)
