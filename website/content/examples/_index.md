+++
title = "Examples"
+++

![default](screenshot_default.png "Default App Launcher")

_Out of the box_ look and feel with the default GTK Adwaita theme.

---

![styled](screenshot_styled.png "Styled App Launcher")

This example shows waymenu styled with CSS.

---

![power](screenshot_power.png "Power Actions Menu")

Custom menu with "Power" related actions.

```json
[
    {
        "label": "Lock",
        "icon": "/usr/share/icons/Pop/128x128/actions/system-lock-screen.svg",
        "exec": ["swaylock", "-f", "-e"]
    },
    {
        "label": "Suspend",
        "icon": "/usr/share/icons/Pop/128x128/actions/system-suspend.svg",
        "exec": ["systemctl", "suspend"]
    },
    {
        "label": "Logout",
        "icon": "/usr/share/icons/Pop/128x128/actions/system-log-out.svg",
        "exec": ["hyprctl", "dispatch", "exit"]
    },
    {
        "label": "Restart",
        "icon": "/usr/share/icons/Pop/128x128/actions/system-restart.svg",
        "exec": ["systemctl", "reboot"]
    },
    {
        "label": "Shutdown",
        "icon": "/usr/share/icons/Pop/128x128/actions/system-shutdown.svg",
        "exec": ["systemctl", "poweroff"]
    }
]
```

```css
#window {
    border-radius: 10px;
}

#list row {
    padding: 10px;
}

#list row image {
    -gtk-icon-size: 96px;
    padding-bottom: 5px;
}
```

```json
{
    "width": 580,
    "height": 142,
    "orientation": "horizontal",
    "hide_search": true
}
```