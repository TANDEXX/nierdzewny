## about
this is my operating system writen in rust.
I write it for fun, but I have some interesting ideas to make

### first time
Only on linux or mac, for windows `manage.py` script won't work properly.
when you have downloaded this project, then go to this project directory and run:
`chmod -R +x . && mkdir boot/boot/rust/.cargo && ln -s "$(pwd)"/boot/boot/rust/config.toml boot/boot/rust/.cargo/config.toml && mkdir -p out/build/boot/objects`
This makes executable permitions for project and creates some directories

### build
You need rust nightly for building this project

build it to bin file image using: `./manage.py -bi --release`.

### running
run it using: `./manage.py -r`.

### script
`manage.py` script is writen for linux and maybe will work on mac'os, but won't work on windows.

### licence
this project is `open source` and original author is `TANDEX`.

### name
name is from Polish language meaning `stainless`. 

### changing version
If you are developing this system and want to change version:
Change variable `boot_name` and version display function or someting in manage.py
Then modify `out/img/boot/grub/grub.cfg` to pass boot name
Also you can change version in `boot/boot/rust/Cargo.toml` (not neccesary)
