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
- this ability is not completed because i don't know how can i run this as service
just open the executable and don't close the terminal.

copy link and press `CTRL+SHIFT+Q`\
program will short your link and will show notification after job finished. (it's a global hotkey)


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


## version :
v1.9.0

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
- [ ] run in startup ( as service )

### v3 :
- [ ] make installation package
- [ ] add system try icon 
- [ ] hotkey changeable
- [ ] set private api key on config file
- [ ] make an local history (do0 has history but i don't want to use it)
- [ ] make a ui interface


# Thired party libraries 
- [reqwest](https://github.com/seanmonstar/reqwest)
- [serde](https://github.com/serde-rs/serde)
- [clipboard](https://github.com/aweinstock314/rust-clipboard)
- [hotkey](https://github.com/UnwrittenFun/hotkey-rs)
- [notifica](https://github.com/frewsxcv/rust-notifica)

# License 
- WTFPL
- AGPL-v3.0

as you wish :D