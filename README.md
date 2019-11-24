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

use `do0ir-cli Link `


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
### v1.0 :
- [x] check input is url or not.
- [x] handle do0 errors 
### v2.0 :
- [x] copy short link to clipboard
- [x] short website link with a shortcut and copy to clipboard
- [x] notification center (it's done but has a problem in windows 10)
- [x] better Cli argument.
- [ ] run in startup ( as service )

### v3.0 :
- [ ] get list of all link you shorted
- [ ] make a ui interface
