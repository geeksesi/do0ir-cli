# do0ir-cli
cli tools for [do0.ir](https://do0.ir)

## what is do0.ir
do0 is a good link shorter. can protect you from fishing links.


build :
```
cargo build
```

run :
```
cargo run YOURLINK_HERE
```

## installation
download build from release ( or build it for yourself with --release flag)

move it to enviroment path ( windows : `"C:\Program Files"` linux : look at `echo $PATH`)

use `do0ir-cli Link `

## version :
v0.1.0

## todo :
- [x] make windows release
- [ ] make linux release
- [ ] make mac release
### v1.0 :
- [ ] check input is url or not.
- [ ] handle do0 errors 
### v2.0 :
- [ ] copy short link to clipboard
- [ ] get list of all link you shorted
### v3.0 :
- [ ] short website link with a shortcut and copy to clipboard
- [ ] make a ui interface
