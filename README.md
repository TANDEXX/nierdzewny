# nierdzewny
operating system writen in rust

### build
you need rust nightly to build project but this is installed by `manage.sh` script for unix & linux.
on windows you can build it but manually or writing own script if you known the `.bat` scripts.

build it to bin file image using: `./manage.sh --ball --release`.

### running
this is runned by script in qemu and cannot run in `virtualbox` because it generate `.bin` file.

run it using: `./manage.sh --ball --run`.

### script
this script is only for linux & unix. On Mac'os this doesn't work because this  script is writen in bash no sh.
if rust nightly or recuied packages are not installed, then script will automatically install them (if you agree to that).
script has configuration in `config.desktop` file, and it's easy to change that.

### licence
this project is `open source` and original author is `TANDEX`.

### name
name is from Polish language meaning `stainless`.
