# kswitch

# Key tasks

- [x] cli interface
- [ ] Set
    - [x] Global Theme
    - [ ] Terminal
    - [x] Color scheme
    - [x] Wallpaper
    - [ ] Cursor theme
    - [ ] Kvantum engine theme (for QT applications)
    - [ ] Environment Variable setting to maintain persistent light/dark mode state. 
        - [ ] ```export MY_VAR="my_value"; systemctl --user import-environment MY_VAR```

- [ ] Get
    - [ ] Environment Variable for active theme
    - [ ] Default theme given time + [sunrise](https://crates.io/crates/sunrise) crate.
- [x] Set themes
    - [x] Light
    - [x] Dark
- [ ] Toggle
- [x] Config
    - [x] Create config struct
        - [x] readable from file
        - [x] saveable to file
        - [x] editable using default editor
    - [x] Create at default location
- [ ] Schedule

## Future improvements

- [ ] Interactive configuration
- [x] synchronous
- [ ] Implement timed changes using systemctl with systemd timers
- [ ] Write custom wallpaper plugin using QML/Plasmoid dev to support transition time control. 
- [ ] Custom bash script at ~/.config/kswitch/light.sh and ~/.config/kswitch/dark.sh
- [ ] Modify config to have an option to not toggle a particular feature. 
- [ ] Pure dbus controlled branch

