
extern crate clap;
use clap::{Arg, App, SubCommand};
use notify_rust::{Notification, Hint};

use rsmediakeys::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // This is all boilerplate clap, see clap manual for explanation
    let matches = App::new("RSmediakeys")
                          .version(env!("CARGO_PKG_VERSION"))
                          .author("Bas Dusee, https://github.com/basdusee/rsmediakeys")
                          .about("Command line mpd client tailored for mediakeys, useful for Dunst and special Thinkpad keys")
                          .arg(Arg::with_name("socket")
                               .short("s")
                               .long("socket")
                               .value_name("SOCKET")
                               .help("Let rsmediakeys know where the mpd control socket is. Defaults to ~/.config/mpd/socket")
                               .takes_value(true))
                          .arg(Arg::with_name("icondir")
                               .short("i")
                               .long("icondir")
                               .value_name("ICONDIR")
                               .help("Use this directory for the icons sent to the DBUS messenger. Defaults to /usr/share/icons/hicolor/scalable")
                               .takes_value(true))
                          .subcommand(SubCommand::with_name("next")
                                      .about("Switch to next song in playlist"))
                          .subcommand(SubCommand::with_name("prev")
                                      .about("Switch to previous song in playlist"))
                          .subcommand(SubCommand::with_name("stop")
                                      .about("Stops mpd from playing"))
                          .subcommand(SubCommand::with_name("toggle")
                                      .about("Pauses or unpauses mpd, start playing if stopped"))
                          .get_matches();

    // Communication with the Music Playing Daemon is via a unix socket.
    // Get the name of the socket or use a sane default
    let defaultsocket = format!("{}/.config/mpd/socket", env!("HOME"));
    let sock = matches.value_of("socket").unwrap_or(&defaultsocket); 
    let mut mpd = Connection::new(sock)?;

    let defaulticondir = format!("{}/.local/share/icons", env!("HOME"));
    let icondir = matches.value_of("icondir").unwrap_or(&defaulticondir); 
    let nexticon = format!("{}{}", icondir, "media-skip-forward-symbolic.svg");
    let previcon = format!("{}{}", icondir, "media-skip-backward-symbolic.svg");
    let stopicon = format!("{}{}", icondir, "media-playback-stop.svg");
    let playicon = format!("{}{}", icondir, "media-playback-start-symbolic.svg");
    let pauseicon = format!("{}{}", icondir, "media-playback-pause-symbolic.svg");

    // Check if we are in X windows with notify_send capabilities
    match std::env::var("XDG_SEAT") { 
        Ok(_v) => {
            match matches.subcommand_name() {
                Some("next") => { Notification::new()
                                    .summary(&mpd.next()?.0)
                                    .hint(Hint::Category("mpd".to_owned()))
                                    .icon(&nexticon)
                                    .show()?; },
                Some("prev") => { Notification::new()
                                    .summary(&mpd.previous()?.0)
                                    .hint(Hint::Category("mpd".to_owned()))
                                    .icon(&previcon)
                                    .show()?; },
                Some("stop") => { Notification::new()
                                    .summary(&mpd.stop()?.0)
                                    .hint(Hint::Category("mpd".to_owned()))
                                    .icon(&stopicon)
                                    .show()?; },
                Some("toggle") => { let cmd = mpd.toggle()?;
                                    Notification::new()
                                    .summary(&cmd.0)
                                    .hint(Hint::Category("mpd".to_owned()))
                                    .icon( match cmd.1 {
                                               StateOfPlay::Stop => &stopicon,
                                               StateOfPlay::Pause => &pauseicon,
                                               StateOfPlay::Play => &playicon,
                                           })
                                    .show()?; },
                None => println!("Please give a command, or ask --help"),
                _ => println!("Command is not implemented (yet). You never know, keep wishing"),
            }
        }
        // else we just output to the console
        Err(_e) => {
            match matches.subcommand_name() {
                Some("next") => println!("{}", mpd.next()?.0),
                Some("prev") => println!("{}", mpd.previous()?.0),
                Some("stop") => println!("{}", mpd.stop()?.0),
                Some("toggle") => println!("{}", mpd.toggle()?.0),
                None => println!("Please give a command, or ask --help"),
                _ => println!("Command is not implemented (yet). You never know, keep wishing"),
            }
        }
    }

    Ok(())
}
