# rsmediakeys

Minimalistic MPD client to use laptop multimedia keys with Music Player Daemon


## Designed for

This command line tool is a very stripped down Music Player Daemon client.
It is used to make your media keys work correct if you:
* run Linux (btw. I run Arch :)
* probably run a tiling/simple Window Manager (I use i3, but things like Xmonad, bspwm, dwm, you get the drift)
* run Dunst or another notify daemon. If notify-send works, you are probably fine.
* run Music Player Daemon as a primary music player
* have a laptop/keyboard which has media keys (play/pause/stop) which Xorg recognizes

It's probably best suited for the folks who run lean setups with slimmed-down window managers and such. 
If you run a big Desktop Environment with bells and whistles, than this functionality is probably already built into the DE.

## Purpose

This little utility solves a small irritation when setting up my mediakey binding in my window manager.
I used the music player client for this (mpc) which is extremely adequate, but has one little annoyance.
If you "toggle" play/pause, then there is no direct hint from mpc if music is playing now or paused.
This makes me cry because now I can't fully leverage Dunst by using the correct icon.

This problem can obviously and easily be solved with Bash or another call to mpc, 
but that's never the human way to solve problems.
It should be solved in an absurd, emotional and illogical way.
So that's why this is fully written in Rust. "Because it is possible" to do so.

As a secondary purpose it's a little demonstration that you can do simple things in Rust.
It doesn't always have to be the fastest webserver in the world or the most efficient JSON parser.
Sometimes it's just a small hackup of something that probably could/should be done in Bash, 
and it turns out Rust is also great for that sort of things.

## Design

The program is a CLI program with only four commands which you can give it:
toggle, next, prev, stop.
It just passes those commands to the Music Player Daemon socket and gets some info of the song title and artist.

If it detects that you are in a notify-send capable environment, it will give a popup message, 
else it will print some stuff to the terminal.

The intention and idea is to use it in a config file or script to control MPD.
I use it in my i3 config file to capture mediakeys press events and talk to MPD about it,
but it can for sure be used by sxhkd probably, or in dwm's config.h.

It uses clap for command line parameter stuff, and notify-rust for the OSD notify things.
Both are awesome libraries.
* https://crates.io/crates/clap
* https://crates.io/crates/notify-rust

## Installation

You need to have Rust installed. The programming language that is, not the game.
This is documented extensively on this great thing called the interwebz and I won't repeat it here.
But you can find a small hint here: https://www.rust-lang.org/tools/install
This code was developed on a userspace install of Rust using rustup. 

To create the program, just do the obvious. git clone this stuff and do:
`cargo build --release`
It compiles some dependancies and an executable.
Then just copy ./target/release/rsmediakeys to a useful place.

## Configuration

I use i3 and in the i3config just put something like this:
```
# set Thinkpad play/pause/next keys to control mpd
bindsym XF86AudioPlay exec --no-startup-id $somedir/rsmediakeys toggle
bindsym XF86AudioNext exec --no-startup-id $somedir/rsmediakeys next
bindsym XF86AudioPrev exec --no-startup-id $somedir/rsmediakeys prev
bindsym XF86AudioStop exec --no-startup-id $somedir/rsmediakeys stop
```
of course you need to set the $somedir variable or put a real path there.

## Icons

The iffy part for now is in the icons which are sent to the popup windows (dunst..).
You have to arrange the icons for yourself. This is because everybody has a different preference in icons.

It will look into ~/.local/share/icons for spefically named icons.
These icons are mostly present in modern icon sets, so you can look in your own icon set if they are there.
They are found in the /usr/share/icons/somethemename/scalable/actions directory.

The names are:
* media-skip-forward-symbolic.svg
* media-skip-backward-symbolic.svg
* media-playback-stop-symbolic.svg
* media-playback-start-symbolic.svg
* media-playback-pause-symbolic.svg

You can also rename them in the src/main.rs file if you really want to.
I used one of the more populair icon sets to get mine. I think it was Papyrus.
In the "normal" set (not the dark or the light variant) there is a symbolic folder with an action folder, with this files in it.

## Manpage

I'll just repeat the help from the command line here:

```
Command line mpd client tailored for mediakeys, useful for Dunst and special Thinkpad keys

USAGE:
    rsmediakeys [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --icondir <ICONDIR>    Use this directory for the icons sent to the DBUS messenger. Defaults to
                               ~/.local/share/icons
    -s, --socket <SOCKET>      Let rsmediakeys know where the mpd control socket is. Defaults to ~/.config/mpd/socket

SUBCOMMANDS:
    help      Prints this message or the help of the given subcommand(s)
    next      Switch to next song in playlist
    prev      Switch to previous song in playlist
    stop      Stops mpd from playing
    toggle    Pauses or unpauses mpd, start playing if stopped
```
## Disclaimers, FAQ, Troubleshooting and stuff

Enjoy!

This is only tested on Arch Linux with i3gaps on a Thinkpad x220 with the US keyboard layout. 
The Music Player Daemon version is 0.22.4 and it is run as a deamon under my own user account. It starts when logging into i3.
I didn't test it with a MPD daemon started at the system level or as root. As long as you can read/write the Unix socket, it should be alright.
Good luck with anything other than this setup. It shoudn't be hard to adjust though.

###_Hey I press buttons but no music is starting, what's happening? I see a popup with just a stripe/minus_
You need to make a playlist in a proper MPD client (like ncmpcpp) to be able to use the keys. 
There is no playlist management in rsmediakeys.

###_I press the "next" key and there is a popup saying "Not Playing"_
That's right. You can't "next" if MPD is not playing music. This is a MPD feature, not a rsmediakeys one. 
The fix is obvious, hit the "play/pause" button, and then the "next" button.
Fun fact, the actual string "Not Playing" comes from MPD and is an error message.
This string isn't embedded in rsmediakeys at all. The rsmediakeys program will just copy/paste error messages from MPD.

###_My popup notifications look ugly_
Please refer to the documentation of your notification manager to beautify it. Rsmediakeys just puts text into a bus and is not responsible for the display side of things.

###_I want to collaborate and/or push some changes to this microscopic project_
Any help in form of descriptive and friendly issues or comprehensive pull requests are welcome!
Please do. Just open an issue or generate a pull request. Because of the size of this project it isn't well managed, so do as you think is fit for the occasion.
There will be some more guidlines on the licenses of pull requests in the future.

## Todo
* icon management should be a little more polished
* maybe album art in the on screen pop-up? 
* documentation has an existential crisis, because it is non-existing.
* output to notification pop-up or to console should be more customizable other than just hacking it in the code
* MPD accepts compound commands, it's probably more efficient in some cases and opens up the possibility to simplify this code.
* maybe some hook pospibility, that it starts your full-fledged music client (ncmpcpp probably) if the playlist runs out.
* there is an almost infinite scale-up in functionality. It does 0,1% of what mpc does, so ... but maybe that's a strength.
* there's also the possibility to integrate volume as well, but that gets complicated very quickly and is easily done with other command line tools.

## Probably never do
* lcd backlight buttons, please use the incredible light program for this. It has the worst naming ever (try searching "light" on the internet) but it's the best out there by far: https://github.com/haikarainen/light
* full mpc replacement and/or full MPD client functionality including playlist management et al.
* it probably won't get a gui/tui. It's buttons. On the keyboard. That's it.
