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

## Usage

Just do the obvious. git clone this stuff and do:
`cargo build --release`
It compiles some dependancies and an executable.
Then just copy ./target/release/rsmediakeys to a useful place.

I use I3 and in the i3config just put something like this:
```
# set Thinkpad play/pause/next keys to control mpd
bindsym XF86AudioPlay exec --no-startup-id $somedir/rsmediakeys toggle
bindsym XF86AudioNext exec --no-startup-id $somedir/rsmediakeys next
bindsym XF86AudioPrev exec --no-startup-id $somedir/rsmediakeys prev
bindsym XF86AudioStop exec --no-startup-id $somedir/rsmediakeys stop
```
of course you need to set the $somedir variable or put a real path there.

Enjoy!

This is only tested on Arch Linux with i3 on a Thinkpad x220.
Good luck with anything other than that. It shoudn't be hard to adjust though.

## Todo
* icon management should be a little more polished
* maybe album art in the on screen pop-up? 
* documentation has a non-existential crisis, because it is non-existing.
* output to notification pop-up or to console should be more customizable other than just hacking it in the code
* maybe some hook pospibility, that it starts your full-fledged music client (ncmpcpp probably) if the playlist runs out.
* there is an almost infinite scale-up in functionality. It does 0,1% of what mpc does, so ... but maybe that's a strength.
* there's also the possibility to integrate volume as well, but that gets complicated very quickly and is easily done with other command line tools.

## probably never do
* lcd backlight buttons, please use the incredible light program for this. It has the worst naming ever (try searching "light" on the internet) but it's the best out there by far: https://github.com/haikarainen/light
* full mpc replacement and/or full MPD client functionality including playlist management et al.
* it probably won't get a gui/tui. It's buttons. On the keyboard. That's it.


