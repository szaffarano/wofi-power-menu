use crate::{
    icons,
    wofi::{Item, Menu, Wofi},
};

pub fn default_menu() -> Menu {
    Menu::new(
        String::from("Power menu"),
        vec![
            Item::new("Shut down", icons::SHUTDOWN, "systemctl poweroff", true),
            Item::new("Reboot", icons::REBOOT, "systemctl reboot", true),
            Item::new("Suspend", icons::SUSPEND, "systemctl suspend", true),
            Item::new("Hibernate", icons::HIBERNATE, "systemctl hibernate", false),
            Item::new("Logout", icons::LOGOUT, "loginctl terminate-session", false),
            Item::new(
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
