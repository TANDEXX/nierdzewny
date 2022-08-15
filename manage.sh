#!/bin/bash
# script for manage "nierdzewny" project writen in unix bash

cget() { # function imported from my unfinished (and not published) bash library project with original name: "besktopGet"

	if [ -z $1 ] || [ -z $2 ] || [ -z $3 ]; then

		echo "err: expected 3 arguments"

	else

		doThis=y
		found=n
		if ! [ -f $1 ]; then

			echo "err: is not file or not exist"
			doThis=n

		fi

		if ! [ -r $1 ] && [ $doThis = y ]; then

			echo "err: can't read file"
			doThis=n

		fi

		if [ $doThis = y ]; then

			exec 254<$1
			while [ $found != y ] && read -u 254 tmp; do

				if [ "$tmp" = "[$2]" ]; then

					found=y

				fi

			done

			if [ $found = n ]; then

				echo "err: container not found"

			else

				found=n
				while [ "$found" != y ] && read -u 254 tmp; do

					if [ "`echo "$tmp" | cut -c1-$((${#3} + 1))`" = "$3=" ] || [ "`echo "$tmp" | cut -c1-$((${#3} + 2))`" = "$3 =" ]; then

						found=y

					fi

				done

				if [ $found = n ]; then

					echo "err: value not found"

				else

					c="`echo "$tmp" | cut -c$((${#3} + 1))-$((${#3} + 1))`"
					if [ "$c" = "=" ]; then
						echo `echo "$tmp" | cut -c$((${#3} + 2))-${#tmp}`
					else
						echo `echo "$tmp" | cut -c$((${#3}+3))-${#tmp}`
					fi

				fi

			fi

			exec 254<&-

		fi

	fi

}

cline() {
	state=0
	space=0
	at=1
	buffer=""
	name=""
	args="$@"
	len="${#args}"

	while [ $at -le $len ]; do
		char="`echo "$args" | cut -c$at-$at`"

		if [ "$char" = '' ] || [ "$char" = ' ' ]; then

			space=$(( $space + 1 ))

		elif [ "$char" = '
' ]; then
			echo -n

		elif [ "$char" = = ] && [ $state = 0 ]; then

			name="$buffer"
			buffer=""
			state=1

		else

			if [ $state != 1 ]; then

				for _ in `seq 1 $space`; do

					buffer="$buffer "

				done

			else

				state=2

			fi

			space=0
			buffer+="$char"

		fi

		at=$(( $at + 1 ))
	done

	content="$buffer"

}

rod() {

	read rodinput

	if [ -z "$rodinput" ]; then

		echo $1

	else

		echo $rodinput

	fi

}

log() { # [tabs] [i, w, e] [do enter (1 if yes)] [message]
	spaces=`seq 1 $tabsize`

	for _ in `seq 1 $1`; do

		for _ in $spaces; do

			echo -n " "

		done

	done

	if [ $2 = i ]; then

		if [ $color = 1 ]; then

			echo -n "[0;1;34minfo[0m: "

		else

			echo -n "info: "

		fi

	elif [ $2 = w ]; then

		if [ $color = 1 ]; then

			echo -n "[0;1;33mwarn[0m: "

		else

			echo -n "warn: "

		fi

	elif [ $2 = e ]; then

		if [ $color = 1 ]; then

			echo -n "[0;1;31merror[0m: "

		else

			echo -n "error: "

		fi

	elif [ $2 = q ]; then

		if [ $color = 1 ]; then

			echo -n "[0;1;35mquestion[0m: "

		else

			echo -n "question: "

		fi

	fi

	if [ $3 = 1 ]; then

		echo "$4"

	else

		echo -n "$4"

	fi

}

cd "`dirname "$0"`" # set up default values

conf="config.desktop" # default settings
rh="$HOME/.cargo/bin"

b=0 # variables
bboot=0
bootconf=0
run=0
img=0
release=0
help=0
abort=2
vnc=0
doc=0
mod=0

if [ -f "$conf" ] && [ -r "$conf" ]; then

tabsize="`cget "$conf" output tab-size`" # read config
colors="`cget "$conf" output color`"
tmp="`cget "$conf" cache dir`"
vncdelay="`cget "$conf" run vncdelay`"

if [ "$colors" = yes ] || [ "$colors" = y ]; then

	color=1

elif [ "$colors" = auto ] || [ "$colors" = a ]; then

	if tty > /dev/null; then

		color=1

	else

		color=0

	fi

else

	color=0

fi

else

	echo "cannot access config file: \"$conf\". Aborting"
	exit 1

fi

if ! [ -d "$tmp" ]; then # set up tmp dir if not exist

	log 0 i 1 "setting up tmp dir"

	if [ -e "$tmp" ]; then

		rm -rf "$tmp"

	fi

	mkdir -p "$tmp"
	mkdir "$tmp/rust"
	mkdir "$tmp/gen"

fi

if ! command -v qemu-system-x86_64 > /dev/null; then # install packages if not installed

	toinstall="qemu-kvm qemu"

fi

if ! command -v vncviewer > /dev/null; then

	toinstall="$toinstall xtightvncviewer"

fi

if [ -n "$toinstall" ]; then

	log 0 q 0 "some packages are missing. install it automatically (yn)? "
	read input

	if [ "$input" = y ]; then

		if command -v apt > /dev/null; then

			sudo apt install $toinstall

		elif command -v yay > /dev/null; then

			yay -S $toinstall

		elif command -v pacman > /dev/null; then

			sudo pacman -S $toinstall

		elif command -v dnf > /dev/null; then

			sudo dnf install $toinstall

		elif command -v zypper > /dev/null; then

			sudo zypper install $toinstall

		elif command -v apk > /dev/null; then

			sudo apk add --no-cache $toinstall

		elif command -v emerge > /dev/null; then

			sudo emerge $toinstall

		elif command -v yum > /dev/null; then

			sudo yum install $toinstall

		else

			log 0 i 1 "umknown package manager, install these packages manualy: $toinstall"

		fi

	else

		log 0 i 1 "install these packages manualy: $toinstall"

	fi

fi

for arg in "$@"; do # argument check

	if [ "`echo $arg | cut -c1-2`" = "--" ]; then

		if [ "$arg" = --help ]; then

			help=1

		elif [ "$arg" = --bboot ]; then

			b=1
			bboot=1

		elif [ "$arg" = --ball ]; then

			b=1
			bboot=1

		elif [ "$arg" = --img ]; then

			img=1

		elif [ "$arg" = --boot-config ]; then

			bootconf=1

		elif [ "$arg" = --release ]; then

			release=1

		elif [ "$arg" = --run ]; then

			run=1
			img=1

		elif [ "$arg" = --no-abort ]; then

			abort=0

		elif [ "$arg" = --low-abort ]; then

			abort=1

		elif [ "$arg" = --abort ]; then

			abort=2

		elif [ "$arg" = --vnc ]; then

			vnc=1

		elif [ "$arg" = --doc ]; then

			doc=1

		elif [ "$arg" = --mods ]; then

			mod=1

		elif [ "$arg" = --mods-specify ]; then

			mod=2

		else

			log 0 w 1 "wrong option: \"$arg\""

		fi

	else

		log 0 w 1 "no options are not supported here"

	fi

done

if [ $help = 1 ]; then

	echo -n "info:
	script for manage \"nierdzewny\" project writen in unix bash
usage:
	./manage.sh [OPTIONS]
	no options are no supported here
options:
	--help	display this help message
	--bboot	build boot
	--ball	build all components (b from build, all)
	--boot-config	let you configure boot while building
	--release	build in release mode: some optimalizations, question you where place generated file
	--run	runs system
	--img	creates image file
	--no-abort	no abort, for example system will run but version before failed building (should be not used)
	--low-abort	no abort but only other things will run but system will not start
	--abort	set abort level to default
	--vnc	run vncviewer at :5900 port (use when qemu don't run it self)
	--doc	build and show documentation in browser
	--clean clean project cache (out dir, boot/boot/rust/target dir) (TODO)
	--mods	regenerate some files and link modules to boot. (Implement modules to boot)
	--mods-specify	like \`--mods\` option but you specify what modules put and what to not put
sequence:
	functions, read confiuration, setup temporary dir, install needed packages, pass arguments, display help, modules, confiure boot, generate and show boot documentation, build & install rust nightly if not installed, run in qemu
"
b=0
run=0

fi

if [ $mod != 0 ]; then

	log 0 i 1 "passing modules"
	log 1 i 1 "indexing modules"
	mods_early_init=""
	mods_shutdown=""
	mods_init=""
	mods_stop_machine=""
	mods_keyboard_int=""
	mods_timer_int=""
	mods_reboot_machine=""
	mods=""

	for m in `ls boot/mods`; do
		mod_init=false
		mod_early_init=false
		mod_shutdown=false
		mod_stop_machine=false
		mod_keyboard_int=false
		mod_timer_int=false
		mod_reboot_machine=false
		include=true

		while read line; do

			cline "$line"

			if [ "$name" = name ]; then

				mod_name="$content"

			elif [ "$name" = version ]; then

				mod_ver="$content"

			elif [ "$name" = include ]; then

				if ! $content; then

					include=false
					break

				fi

			elif [ "$name" = desc ]; then

				mod_desc="$content"

			elif [ "$name" = functions ]; then

				for fn in $content; do

					if [ "$fn" = init ]; then

						mod_init=true

					elif [ "$fn" = early_init ]; then

						mod_early_init=true

					elif [ "$fn" = shutdown ]; then

						mod_shutdown=true

					elif [ "$fn" = stop_machine ]; then

						mod_stop_machine=true

					elif [ "$fn" = reboot_machine ]; then

						mod_reboot_machine=true

					elif [ "$fn" = keyboard_int ]; then

						mod_keyboard_int=true

					elif [ "$fn" = timer_int ]; then

						mod_timer_int=true

					else

						log 2 w 1 "unknown function \"$fn\""

					fi

				done

			else

				log 2 w 1 "unknown key \"$name\""

			fi

		done < "boot/mods/$m/mod/meta.cfg"

		if $include && [ $mod = 2 ]; then

			log 2 q 0 "include \"$mod_name\": $mod_desc (yn)? "
			inp="`rod y`"

			if [ "$inp" = n ] || [ "$inp" = no ]; then

				include=false

			fi

		fi

		if $include; then

			if $mod_init; then

				mods_init+=" $m"

			fi

			if $mod_early_init; then

				mods_early_init+=" $m"

			fi

			if $mod_shutdown; then

				mods_shutdown+=" $m"

			fi

			if $mod_stop_machine; then

				mods_stop_machine+=" $m"

			fi

			if $mod_reboot_machine; then

				mods_reboot_machine+=" $m"

			fi

			if $mod_keyboard_int; then

				mods_keyboard_int+=" $m"

			fi

			if $mod_timer_int; then

				mods_timer_int+=" $m"

			fi

			mods+=" $m"

		fi

	done

	log 1 i 1 "generating boot source file"
	rm -rf boot/boot/rust/src/mods/built
	mkdir boot/boot/rust/src/mods/built
	srcf="boot/boot/rust/src/mods/built/mod.rs"

	cat > "$srcf" <<EOF
#!/bin/nano
//! this file is automatically generated by \`manage.sh\` script. You should not edit this file, generate it by \`./manage.sh --mods\` or by \`./manage.sh --specify-mods\`

EOF

	chmod 775 "$srcf"

	for m in $mods; do

		echo "pub mod $m;" >> "$srcf"

	done

	cat >> "$srcf" <<EOF

/// modules early init, first thing that boot do
pub fn early_init() {

EOF

	for m in $mods_early_init; do

		echo "	$m::early_init();" >> "$srcf"

	done

	cat >> "$srcf" <<EOF

}

/// modules init, initianize modules after most important things
pub fn init() {

EOF

	for m in $mods_init; do

		echo "	$m::init();" >> "$srcf"

	done

	cat >> "$srcf" <<EOF

}

/// modules shutdown, function called on system shutdown
pub fn shutdown() {

EOF

	for m in $mods_shutdown; do

		echo "	$m::shutdown();" >> "$srcf"

	done

	cat >> "$srcf" <<EOF

}

/// modules machine stop, function for shutdown system
pub fn stop_machine() {

EOF

	for m in $mods_stop_machine; do

		echo "	$m::stop_machine();" >> "$srcf"

	done

	cat >> "$srcf" <<EOF

}

/// modules machine reboot, function for reboot system
pub fn reboot_machine() {

EOF

	for m in $mods_reboot_machine; do

		echo "	$m::reboot_machine();" >> "$srcf"

	done

	cat >> "$srcf" <<EOF

}

/// modules keyboard interrupt, called when keyboard interrupt arrives, used for keyboard input
pub fn keyboard_int() {

EOF

	for m in $mods_keyboard_int; do

		echo "	$m::keyboard_int();" >> "$srcf"

	done

	cat >> "$srcf" <<EOF

}

/// modules timer interrupt, function called on timer interrupt
pub fn timer_int() {

EOF

	for m in $mods_timer_int; do

		echo "	$m::timer_int();" >> "$srcf"

	done

	cat >> "$srcf" <<EOF

}
EOF

	log 1 i 1 "linking modules"

	for m in $mods; do

		ln -s "../../../../../mods/$m/src/" boot/boot/rust/src/mods/built/"$m"

	done

fi

if [ $bootconf = 1 ]; then

	log 0 i 1 "configuring boot, leave empty to default"
	log 1 q 0 "terminal pages (default: 10): "
	pages="`rod 10`"
	log 1 i 1 "color: BACKGROUND_COLOR_CODE + FOREGROUND_COLOR_CODE. Sample: 0 + 7 = 07, c + f = cf"
	log 1 i 1 "color codes: 0: black, 1: blue, 2: green, 3: cyan, 4: red, 5: magenta, 6: brown, 7: light gray, 8: dark grey, 9: light blue, a: bright green, b: bright cyan, c: light red, d: bright magenta, e: yellow, f: white"
	log 1 q 0 "default color (default: 07): "
	dcolor="`rod 07`"
	log 1 q 0 "hight light color (default: 0f): "
	hcolor="`rod 0f`"
	log 1 q 0 "color when panicing (default: cf): "
	pcolor="`rod cf`"
	log 1 q 0 "lines scroll per one PgUp / PgDown press (default 7): "
	scroll="`rod 7`"
	log 1 q 0 "want to configure advanced options (yn)? "
	read input

	if [ "$input" = y ]; then

		log 1 i 1 "advanced configure options"
		log 2 q 0 "max number of devices (default 15): "
		devices="`rod 15`"
		log 2 q 0 "max size of object is device (default 8, in bytes): "
		dev_out="`rod 8`"
		log 2 q 0 "device buffer size (default 8, in bytes): "
		dev_buffer="`rod 8`"
		log 2 q 0 "size of Str structure (default 256): "
		str_size="`rod 256`"

	else

		log 1 i 1 "all advanced options are configured to default values"
		devices=15
		dev_out=10
		dev_buffer=8
		str_size=256

	fi

	cat > boot/boot/rust/src/consts/auto.rs <<EOF
#!/bin/nano
//! this file is automatically generated by \`manage.sh\` script. You can edit this and this isn't overwrited but easier is rewriting with this script using \`./manage.sh --boot-config\`
// vga default

/// vga text mode lines
pub const HEIGHT: usize = 25;
/// vga text mode columns
pub const WIDTH: usize = 80;

// terminal
/// terminal buffer length
pub const BUFFER_LEN: usize = WIDTH * HEIGHT * $pages;
/// default terminal color
pub const DEFAULT_COLOR: u8 = 0x$dcolor;
/// default terminal hightlight color
pub const HIGHTLIGHT_COLOR: u8 = 0x$hcolor;
/// lines scrolled per press
pub const SCROLL_PER_PRESS: usize = $scroll;

// panic
/// panic color
pub const PANIC_COLOR: u8 = 0x$pcolor;

// device
/// maximum devices
pub const DEVICES: usize = $devices;
/// maximum size of device writable object
pub const MAX_DEV_OUT: usize = $dev_out;
/// buffer size of every device
pub const DEV_BUFFER: usize = $dev_buffer;

// utility
/// Str size
pub const STR_SIZE: usize = $str_size;
EOF

fi

if [ $doc = 1 ]; then

	log 0 i 1 "generate documentation, cargo output:"
	cd boot/boot/rust

	if "$rh/cargo" doc; then

		log 1 i 1 "cargo finished successfully, running browser"
		cd ../../..
		docfile="file://$PWD/boot/boot/rust/target/target/doc/boot/index.html"

		if [ -z "$BROWSER" ]; then

			log 2 i 1 "browser not set, running firefox"

			if command -v firefox > /dev/null; then

				firefox "$docfile"

			else

				log 3 e 1 "firefox is not installed, if you have other browser then open in it the \"$docfile\" file"

			fi

		else

			"$BROWSER" "$docfile"

		fi

	else

		log 1 e 1 "documentation build failed"
		cd ../../..

	fi


fi

if [ $b = 1 ]; then # build project

	log 0 i 1 "building project"

	if [ $bboot = 1 ]; then # build boot

		log 1 i 1 "building assembly boot part"

		if ! nasm -f elf64 -o out/build/boot/boot/_entry.o boot/boot/asm/x86_64/entry.asm; then

			if [ $abort = 2 ]; then

				echo "boot build failed, aborting..."

			else

				echo "boot build failed"

			fi

			if [ $abort != 0 ]; then

				img=0
				run=0

			fi

		fi

		if [ -e "$rh" ]; then

			if [ "`"$rh/rustc" --version | grep nightly`" != "" ]; then

				if [ $release = 0 ]; then

					log 1 i 1 "building main (rust) boot part, cargo output:"
					cd boot/boot/rust
					if ~/.cargo/bin/cargo build; then

						log 1 i 1 "build finished successfully"
						cd ../../..

					else

						if [ "$abort" = 2 ]; then

							log 1 e 1 "boot build failed. Aborting"
							exit 1

						else

							log 1 e 1 "boot build failed"

							if [ "$abort" = 1 ]; then

								img=0
								run=0

							fi

						fi

					fi

				else

					log 1 i 1 "building main (rust) boot part, cargo output:"
					cd boot/boot/rust

					if "$rh"/cargo build --release; then

						log 1 i 1 "build finished succesfully"
						cd ../../..

					else

						if [ "$abort" = 2 ]; then

							log 1 e 1 "boot build failed. Aborting"
							exit 1

						else

							log 1 e 1 "boot build failed"

							if [ "$abort" = 1 ]; then

								img=0
								run=0

							fi

						fi

					fi

				fi

			else

				log 1 q 0 "rust NIGHTLY not installed. Run install script then install rust NIGHTLY (yn)? "
				read -e input

				if [ "$input" = y ]; then

					if ! [ -f "$tmp/rust/rustup" ]; then

						curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > "$tmp/rust/rustup"
						chmod 777 "$tmp/rust/rustup"

					fi

					"$tmp/rust/rustup"

				fi

			fi

		else

			log 1 q 0 "rust is not installed. Run install script (yn)? "
			read -e input
			if [ "$input" = y ]; then

				if ! [ -f "$tmp/rust/rustup" ]; then

					curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > "$tmp/rust/rustup"
					chmod 777 "$tmp/rust/rustup"

				fi

				log 1 i 1 "please install rust nightly to run project"
				"$tmp/rust/rustup.sh"

			fi

			exit

		fi

		log 1 i 1 "linking boot"

		if [ $release = 1 ]; then

			target=release

		else

			target=debug

		fi

		fail=false

		if ! ld -o out/build/boot/complete-boot out/build/boot/boot/* boot/boot/rust/target/target/$target/libboot.a; then

			fail=true

		fi

		if ! $fail; then

			cp out/build/boot/complete-boot out/img/boot/nierdzewny-0.4

		fi

		if $fail; then

			if [ "$abort" = 2 ]; then

				log 1 e 1 "boot link failed. Aborting"
				exit 1

			else

				log 1 e 1 "boot link failed"

				if [ "$abort" = 1 ]; then

					run=0

				fi

			fi

		fi

	fi

fi

if [ $img = 1 ]; then

	log 0 i 1 "creating disk image"

	if ! grub-mkrescue -o out/output.iso out/img; then

		if [ "$abort" = 2 ]; then

			log 1 e 1 "disk image creation failed. Aborting"
			exit 1

		else

			log 1 e 1 "disk image creation failed"

			if [ "$abort" = 1 ]; then

				run=0

			fi

		fi

	fi

fi

if [ $run = 1 ]; then # system run

	qemuargs=`cget "$conf" run qemu-args`

	if [ "$qemuargs" = [EMPTY] ]; then

		qemuargs=""

	fi

	if [ -f "out/output.bin" ]; then

		if [ $vnc = 1 ]; then

			log 0 i 1 "run system. qemu & vncviewer output:"

			sh <<EOF&

				sleep $vncdelay
				vncviewer :5900
#				touch "$tmp/qemu/vncstopped" # TODO

EOF

		else

			log 0 i 1 "run system. qemu output:"

		fi

		if [ -n $outputfile ]; then

			if qemu-system-x86_64 -cdrom out/output.iso $qemuargs; then

				log 1 i 1 "qemu ended successfully"

			else

				log 1 e 1 "qemu failed"

			fi

		else

			if qemu-system-x86_64 -cdrom "$outputfile" "$qemuargs"; then

				log 1 i 1 "qemu ended successfully"

			else

				log 1 e 1 "qemu failed"

			fi

		fi

	else

		log 1 e 1 "system is not builded. build it running: ./manage.sh --ball"

	fi

fi

log 0 i 1 "end of script reached"
