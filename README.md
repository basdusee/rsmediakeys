# rsmediakeys

Minimalistic MPD client to use laptop multimedia keys with Music Player Daemon


## Designed for

This command line tool is a very stripped down Music Player Daemon client.
It is used to make your media keys work correct if you:
* run Linux
* run a tiling/simple Window Manager (I use i3, but Xmonad, dwm, you get the drift)
* run Dunst or another notify daemon. Notify-send should "work".
* run Music Player Daemon as a primary music player
* have a laptop which has media keys which Xorg recognizes (I have a Thinkpad X220)

## Purpose

This little utility solves a small annoyancing thing in mpc, the music player client.
If you "toggle" play/pause, then there is no direct hint from mpc if music is playing now or paused.
This makes me cry because now I can't fully leverage Dunst by using the correct icon.

This problem can obviously and easily be solved with Bash, but that's never the way to solve problems.
The logical way I mean. It should be solved in an absurd and illogical way.
So that's why this is fully written in Rust. "Because it is possible".

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


