use crate::{
    icons,
    wofi::{Item, Menu, Wofi},
};

pub fn default_menu() -> Menu {
    Menu::new(
        String::from("Power menu"),
        vec![
            Item::new(
                "shutdown",
                "Shut down",
                icons::SHUTDOWN,
                "systemctl poweroff",
                true,
            ),
            Item::new("reboot", "Reboot", icons::REBOOT, "systemctl reboot", true),
            Item::new(
                "suspend",
                "Suspend",
                icons::SUSPEND,
                "systemctl suspend",
                true,
            ),
            Item::new(
                "hibernate",
                "Hibernate",
                icons::HIBERNATE,
                "systemctl hibernate",
                false,
            ),
            Item::new(
                "logout",
                "Logout",
                icons::LOGOUT,
                "loginctl terminate-session",
                false,
            ),
            Item::new(
                "lock-screen",
                "Lock screen",
                icons::LOCK_SCREEN,
                "loginctl lock-session",
                false,
            ),
        ],
    )
}

pub fn default_wofi() -> Wofi {
    Wofi::new("wofi", "--allow-markup --columns=1 --hide-scroll")
}
