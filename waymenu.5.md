# waymenu(5)

## NAME

waymenu - config file, styling, and menu file format

## DESCRIPTION

Oh look! Another description to fill in.

## CONFIG

The default location of the con

Use the `init-config` command to write the default `config.jsonc` to the config
directory, which includes comments describing the various settings.

Options provided on the command line take precedence over config file settings.

## STYLING

Waymenu style is fully customizable using CSS.

The default location of the stylesheet is `$XDG_CONFIG_HOME/waymenu/style.css`.
Use `waymenu init-config` to save the default stylesheet as a starting point.

The widget hierarchy is as follows:

```
window#window
|_ box#window-box
   |_ entry#search
   |_ scrollwindow#scroll
      |_ listview#list
         |_ row
            |_ box
               |_ image
               |_ label
```

The GTK debugger can be helpful for inspecting widgets and making CSS changes
while the app is running.

`GTK_DEBUG=interactive waymenu launcher`

## MENU FORMAT

When using the `menu` command, waymenu reads the menu JSON data from a file
passed to the command, or stdin when the file is not provided.

File format.

```json
[
	{
		// Label of the menu item
		"label": "string",
		// Optional path to icon file. Any file type supported by gio::FileType
		// should be supported https://docs.gtk.org/gio/class.FileIcon.html.
		"icon": "optional[string]",
		// Optional command to execute when the menu item is selected.
		// When not provided, the label is printed to stdout when selected.
		// Provide the command name as a string, or an array of strings to
		// include one or more parameters to the command.
		"exec": "optional[string|array[string]]"
	},
	// ...
]
```