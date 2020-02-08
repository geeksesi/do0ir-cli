# Warning
Do0.ir is out of access.

# do0ir-cli
cli tools for [do0.ir](https://do0.ir)

## what is do0.ir
do0 is a good link shorter. can protect you from fishing links.


## CLI abilities :
- easy to use short link
- just input your link... short link will copy to your clipboard
- support all type of shell ( even CMD :D )





## installation
download build from release ( or build it for yourself with --release flag)

move it to enviroment path ( windows : `"C:\Program Files"` linux : look at `echo $PATH`)

- also rename to `do0.exe` or `do0` it's easier use :)

## Usage :

### as a service (better way)
first step. for you this ability you must put the executable version in your startup ( if you want ) or run app and don't close it.\

copy link and press `CTRL+ALT+d`\
program will short your link and will show notification after job finished. (it's a global hotkey)

- some information to how add app to system try or run app on startup :
#### Windows 
- [trayconizer](https://do0.ir/4qOsV)
- [add app to startup win10](https://do0.ir/DOXGh)
#### linux
- [add to startup _persian_](https://forum.ubuntu.ir/index.php?topic=150026.msg1166712)
- [add to startup](https://www.simplified.guide/linux/automatically-run-program-on-startup)
- also if you have KDE or GNOME or XFCE it's easy to put it on startup setting.
- also if you are using i3 or other fucking minimal wm. you can put it on config file.
- also linux is good. and if your a linux user you can find a way. ❤️

### Cli argument (old way)
we have 2 cli argument. 
- LINK : it's your link
- `--no-service` : this will cancel run as service mode.

usage example : 
```
- do0.exe "https://google.com"
- do0.exe "https://google.com" --no-service
- do0.exe https://google.com --no-service
```

* tip : if your link has encoded character or space you must put it on double qoute. `""`

## developer usage :

build :
```
cargo build
```

run :
```
cargo run YOURLINK_HERE
```

release :
```
cargo build --release
```

## version :
v2.0.0

## todo :
- [x] make windows release
- [ ] make linux release
- [ ] make mac release
### v1 :
- [x] check input is url or not.
- [x] handle do0 errors 
### v2 :
- [x] copy short link to clipboard
- [x] short website link with a shortcut and copy to clipboard
- [x] notification center (it's done but has a problem in windows 10)
- [x] better Cli argument.
- [x] ~~run in startup ( as service )~~ (you can do it by yourself. with add it to startup)

### v3 :
- [ ] make installation package
- [ ] add system try icon 
- [ ] hotkey changeable
- [ ] set private api key on config file
- [ ] make an local history (do0 has history but i don't want to use it)
- [ ] make a ui interface
- [ ] you can choice wich link shorter service you want to use. ( please tell me on issues. ) (for now i want goo.gl and g02.ir)


# Thired party libraries 
- [reqwest](https://github.com/seanmonstar/reqwest)
- [serde](https://github.com/serde-rs/serde)
- [clipboard](https://github.com/aweinstock314/rust-clipboard)
- [hotkey](https://github.com/UnwrittenFun/hotkey-rs)
- [notifica](https://github.com/frewsxcv/rust-notifica)


# known issues : 
- 1 - hotkey has problem on my linux (https://github.com/UnwrittenFun/hotkey-rs/issues/1)
- 2 - notifica will not work correct on windows 10 (https://github.com/frewsxcv/rust-notifica/issues/18)
- 4 - clipboard in linux will clean when proccess did finish (in --no-serve moce can't paste to clipboard.) (https://github.com/aweinstock314/rust-clipboard/issues/61)

# License 
- WTFPL
- AGPL-v3.0

as you wish :D
