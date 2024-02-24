# waymenu(1)

## NAME

waymenu - launcher/menu written in Rust/GTK4 for wlroots based wayland compositors

## SYNOPSIS

**waymenu** [options...] \<COMMAND> [command options...]

## DESCRIPTION

Waymenu is a launcher/menu program for wlroots based compositors written in
Rust using GTK v4. It supports listing/launching applications from `.desktop`
files, or creating a menu with your own entries for scripting purposes.

## COMMANDS

**launcher**
	Show launcher for installed application

**menu**
	Show custom menu of options and optionally output selection to stdout

**init-config**
	Write default `config.jsonc`, `style.css` files and exit

**help**
	Print help or the help of the given subcommand

## OPTIONS

**--help**
	Print help

**-s, --style** \<STYLE>
	Path to stylesheet.
	Default: `$WAYMENU_HOME/style.css` or `$XDG_CONFIG_HOME/waymenu/style.css`.

**-c, --config** \<CONFIG>
	Path to config file.
	Default: `$WAYMENU_HOME/config.jsonc` or `$XDG_CONFIG_HOME/waymenu/config.jsonc`.

**-v**
	Enable verbose logging (or set env var `G_MESSAGES_DEBUG=all`).

**--width** \<WIDTH>
	Window width, default is 640.

**--height** \<HEIGHT>
	Window height, default is 480.

**--orientation** \<ORIENTATION>
	Display menu in vertical or horizontal orientation.
	Default: vertical (possible values: horizontal, vertical).

**--hide-search**
	Hide search field.

**--history-size** \<HISTORY_SIZE>
	Max recent apps at top of launcher list.
	Default: 5 (0 disables launch history).

## ENVIRONMENT

**WAYMENU_HOME**
	Directory where waymenu attempts to find `config.jsonc` and `style.css`.
	When `$WAYMENU_HOME` is not set, waymenu uses `$XDG_CONFIG_HOME/waymenu`.
	When `$XDG_CONFIG_HOME` is not set, waymenu uses `$HOME/.config/waymenu`.
