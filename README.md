Waymenu is a launcher/menu program for wlroots based compositors written in
Rust using GTK v4. It supports listing/launching applications from `.desktop`
files, or creating a menu with your own entries for scripting purposes.

![screenshot1](screenshot1.png "App Launcher")

## Dependencies

* gtk4
* [gtk4-layer-shell](https://github.com/wmww/gtk4-layer-shell)
  [AUR](https://aur.archlinux.org/packages/gtk4-layer-shell)

## Install

Assuming you have `~/.local/bin` in your `$PATH`, use cargo to build and copy
the executable to your local bin directory.

    cargo install --path . --root ~/.local

Or use the `Makefile` to also install man pages (requires `node`).

    make install DESTDIR=~/.local

## Slow Startup

On my old ThinkPad x1 Carbon there is ~400ms delay in starting waymenu.

Turns out this affects other GTK4 apps.
<https://gitlab.gnome.org/GNOME/gtk/-/issues/4112>

There is an upstream issue tracking this in the mesa projects.
<https://gitlab.freedesktop.org/mesa/mesa/-/issues/5113>

Workaround is to use the cairo renderer.

    GSK_RENDERER=cairo waymenu launcher
