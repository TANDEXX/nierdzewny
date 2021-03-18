#!/bin/bash
# script for manage "nierdzewny" project writen in unix bash

cget() { # function imported from my bash library project with original name: "besktopGet"

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

rod() {

	read -e rodinput

	if [ -z $rodinput ]; then

		echo $1

	else

		echo $rodinput

	fi

	echo >&2

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

#cd "`dirname "$0"`" # set up default values

conf="config.desktop" # default settings
rh="$HOME/.cargo/bin"

b=0 # variables
bboot=0
bootconf=0
run=0
release=0
help=0
abort=2
vnc=0

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

		elif [ "$arg" = --boot-config ]; then

			bootconf=1

		elif [ "$arg" = --release ]; then

			release=1

		elif [ "$arg" = --run ]; then

			run=1

		elif [ "$arg" = --no-abort ]; then

			abort=0

		elif [ "$arg" = --low-abort ]; then

			abort=1

		elif [ "$arg" = --abort ]; then

			abort=2

		elif [ "$arg" = --vnc ]; then

			vnc=1

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
	--no-abort	no abort, system will run but version before failed building (should be not used)
	--low-abort	no abort but only other things will run but system will not start
	--abort	set abort level to default
	--vnc	run vncviewer at :5900 port (use when qemu don't run it self)
	--clean clean project cache (out dir, boot/boot/rust/target dir) (TODO)
"
b=0
run=0

fi

if ! [ -e out ]; then # create out dir if not exist

	mkdir out

fi


if [ $bootconf = 1 ]; then

	log 0 i 1 "configuring boot, leave empty to default"
	log 1 q 0 "terminal pages (default: 10): "
	pages="`rod 10`"
	log 1 i 1 "color: FOREGROUND_COLOR_CODE + BACKGROUND_COLOR_CODE. Sample: 0 + 7 = 07, c + f = cf"
	log 1 i 1 "color codes: 0: black, 1: blue, 2: green, 3: cyan, 4: red, 5: magenta, 6: brown, 7: light gray, 8: dark grey, 9: light blue, a: bright green, b: bright cyan, c: light red, d: bright magenta, e: yellow, f: white"
	log 1 q 0 "default color (default: 07): "
	dcolor="`rod 07`"
	log 1 q 0 "hight light color (default: 0f): "
	hcolor="`rod 0f`"
	log 1 q 0 "color when panicing (default: cf): "
	pcolor="`rod cf`"

	cat > boot/boot/rust/src/consts/auto.rs <<EOF
#!/bin/nano
//! this file is automatically generated by \`manage.sh\` script you can edit this and this isn't overwrited but better is rewriting with this script using \`manage.sh --boot-config\`
//! vga default
pub const HEIGHT: usize = 25;
pub const WIDTH: usize = 80;

// terminal
pub const BUFFER_LEN: usize = WIDTH * HEIGHT * 10;
pub const DEFAULT_COLOR: u8 = 0x$dcolor;
pub const HIGHTLIGHT_COLOR: u8 = 0x$hcolor;

// panic
pub const PANIC_COLOR: u8 = 0x$pcolor;
EOF

fi

if [ $b = 1 ]; then # build project

	log 0 i 1 "building project"

	if [ $bboot = 1 ]; then # build boot

		if [ -e "$rh" ]; then

			if [ "`"$rh/rustc" --version | grep nightly`" != "" ]; then

				if [ $release = 0 ]; then

					log 1 i 1 "building boot, cargo output:"
					cd boot/boot/rust
					if ~/.cargo/bin/cargo bootimage; then

						log 1 i 1 "build finished successfully"
						cd ../../..
						cp boot/boot/rust/target/target/debug/bootimage-boot.bin out/output.bin

					else

						if [ "$abort" = 2 ]; then

							log 1 e 1 "boot build failed. Aborting"
							exit 1

						else

							log 1 e 1 "boot build failed"

							if [ "$abort" = 1 ]; then

								run=0

							fi

						fi

					fi

				else

					log 1 i 1 "building boot, cargo output:"
					cd boot/boot/rust

					if "$rh"/cargo bootimage --release; then

						log 1 i 1 "build finished succesfully"
						cd ../../..
						log 1 q 0 "where place generated output file (no ~ character)? "
						read outputfile
						cp boot/boot/rust/target/target/release/bootimage-boot.bin "$outputfile"

					else

						if [ "$abort" = 2 ]; then

							log 1 e 1 "boot build failed. Aborting"
							exit 1

						else

							log 1 e 1 "boot build failed"

							if [ "$abort" = 1 ]; then

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

					log 1 q 0 "install rust automatically (yn)? "
					read -e input

					if [ "$input" = y ]; then

						"$tmp/rust/rustup" --target thumbv7em-none-eabihf <<EOF
y
2

nightly

y
1
EOF

					else

						"$tmp/rust/rustup" --target thumbv7em-none-eabihf

					fi

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

				log 1 q 0 "install automatically (yn)? "
				read input

				if [ "$input" = y ]; then

					"$tmp/rust/rustup.sh" --target thumbv7em-none-eabihf <<EOF
2

nightly

y
1
EOF

				else

					log 1 i 1 "please install rust nightly to run project"
					"$tmp/rust/rustup.sh"

				fi

			fi

			if ! [ -f "$rh/bootimage" ]; then

				log 1 q 0 "rust binary \"bootimage\" is not installed, install it (yn)? "
				read -e input

				if [ $input = y ]; then

					log 1 i 1 "cargo output:"
					if "$rh/cargo" install bootimage; then

						log 1 i 1 "bootimage binary installed successfully"

					else

						log 1 e 1 "bootimage binary installization failed. try again running: cargo install bootimage"

					fi

				fi

				log 1 i 1 "run script again"
				exit

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

			if qemu-system-x86_64 -drive format=raw,file=out/output.bin $qemuargs; then

				log 1 i 1 "qemu ended successfully"

			else

				log 1 e 1 "qemu failed"

			fi

		else

			if qemu-system-x86_64 -drive format=raw,file="$outputfile" "$qemuargs"; then

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
