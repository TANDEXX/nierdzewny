#!/bin/python3
# script designed for linux, it won't work properly on windows, should work on mac'os
import sys, getopt, os, os.path, subprocess, shutil

# constants to modify if you want or need to:
boot_name = "nierdzewny-0.7"
tab_size = 2
color_term = True
log_info_color = "0;1;34"
log_warn_color = "0;1;33"
log_err_color = "0;1;31"
log_input_color = "0;1;35"
log_question_color = "0;1;35"
log_text_color = "0"
log_uinput_color = "0;1;32"
regular_text_color = "0"

# variables you can change:
build_arch = "x86_64" # no other architectures for now so don't change
build_profile = "dev"
bboot = False
release = False
mods = "no"
gen_iso = False
run_virt = "no"
virt_graphic = "gtk"

# variables, do not change them
clean = "no"
cont = True
display_help = False
display_version = False
wrong_options = []

def nop():

	# just to do some operation
	nop = True


# make following directory tree if not exists
def mkdirs(dirs):

	for dir in dirs:
		try:
			os.mkdir(dir)
		except OSError as _:
			nop()


# function to quickly run commands/subprocesses, put "exit code " at end of fail_message because it also adds exit code
def run_comm(args, fail_message_tabs, fail_message):

	process = subprocess.run(args)

	if process.returncode != 0:

		print(log(fail_message_tabs, "err", fail_message + str(process.returncode)))
		return False
	return True


# must be directory obviously, these paths on output that don't end with slash are files
def tree(dir):
	dirs = os.listdir(dir)
	list = []

	for object in dirs:
		path = dir + '/' + object

		if os.path.isdir(path):
			another_list = tree(path)
			list.append(path + '/')
			for another_object in another_list:
				list.append(another_object)
		else:
			list.append(path)

	return list


# destination must be absolute path, yes this code is very ugly
def copy_tree(src, dst):

	try:
		os.makedirs(".cache/copy tree fn/" + src)
	except OSError as _:
		nop()
	try:
		os.rmdir(".cache/copy tree fn/" + src)
	except OSError as _:
		try:
			os.unlink(".cache/copy tree fn/" + src)
		except OSError as _:
			nop()
	os.symlink(dst, ".cache/copy tree fn/" + src)

	for object in tree(src):
		object_dst = dst + object[len(src):len(object)]

		try:
			if object[len(object) - 1] == "/":
				os.mkdir(object_dst)
			else:
				shutil.copy2(object, object_dst)
		except OSError as _:
			nop()


# used by log function
def term_color(color):
	if color_term:
		return "\x1b[" + color + "m"

	return ""


# log infos, errors, etc
def log(tabs, type, msg):
	outs = ""

	for a in range(0, tabs):

		outs += tab

	if type == "info":
		outs += term_color(log_info_color) + "info"
	elif type == "warn":
		outs += term_color(log_warn_color) + "warn"
	elif type == "err":
		outs += term_color(log_err_color) + "err"
	elif type == "input":
		outs += term_color(log_input_color) + "input"
	elif type == "question":
		outs += term_color(log_question_color) + "question"

	outs += ": " + term_color(log_text_color) + msg

	if type == "input":
		outs += "> " + term_color(log_uinput_color)
	elif type == "question":
		outs += "? " + term_color(log_uinput_color)

	return outs


# function for parsing configuration files
def conf(file):
	rf = 'n'
	fn = ""
	fv = ""
	cg = "any"
	spaces = 0
	fsg = []
	fsn = []
	fsv = []

	for a in range(0, len(file)):

		if file[a] == ' ' or file[a] == '\t':
			spaces += 1
		elif file[a] == '\n' or file[a] == '\r':
			if rf != 'n' and rf != 'g' and rf != 'c' and rf != 'G':
				fsg.append(cg)
				fsn.append(fn)
				fsv.append(fv)
			fn = ""
			fv = ""
			rf = 'n'
			spaces = 0
		elif file[a] == '=' and rf != 'c':
			rf = 'v'
		elif file[a] == '#':
			rf = 'c'
		elif file[a] == '[' and rf != 'c':
			rf = 'g'
			cg = ""
		elif file[a] == ']' and rf == 'g':
			rf = 'G'
		elif rf != 'c':

			for b in range(0, spaces):
				if rf == 'n':
					fn += ' '
				elif rf == 'g':
					cg += ' '
				elif rf == 'v':
					fv += ' '
			if rf == 'n':
				if len(fn) == spaces:
					fn = ""
				fn += file[a]
			elif rf == 'g':
				if len(cg) == spaces:
					cg = ""
				cg += file[a]
			elif rf == 'v':
				if len(fv) == spaces:
					fv = ""
				fv += file[a]
			spaces = 0

	return (fsg, fsn, fsv)


# saves last modify time for all source files, don't have use for now or ever
def save_src_modified_time():
	smt = open(ppath + ".cache/smt.txt", "w")
	data = ""

	for object in tree("boot/boot/rust/src"):
		if object[len(object) - 1:len(object)] != '/':
			data += object + " = " + str(os.path.getmtime(object)) + "\n"

	smt.write(data)
	smt.close()
	return

# some "random constants"
home = os.environ["HOME"]
rh = home + "/.cargo/bin/"

# getting project/script full path
pwd = os.getcwd()
rpath = sys.argv[0]
z = len(rpath) - 1

while rpath[z] != '/':
	z -= 1
	if z < 0:
		break

sname = rpath[z + 1:len(rpath)]
ppath = pwd + '/' + rpath[0:z+1]

# preparing for log function
tab = ""
for a in range(0, tab_size):
	tab = tab + " "


#	creating .cache and prepearing project (it does it's work only when needed)
if not os.path.exists(ppath + ".cache/"):
	os.mkdir(ppath + ".cache/")

#if not os.path.exists(ppath + ".cache/smt.txt")
#	save_src_modified_time()
#save_src_modified_time()

# creating out/ directory tree
for object in ["out/build/boot/objects", "out/image/iso/boot"]:
	try:
		os.makedirs(ppath + object)
	except OSError as _:
		nop()

# parsing arguments
try:
	opts, args = getopt.getopt(sys.argv[1:], "brvimM", ["help", "version", "ball", "bboot", "gen-iso", "run", "vnc", "release", "run-term", "run-gtk", "clean", "mods-auto", "mods-specify", "mods-specify-all"])

	for (opt, arg) in opts:

		if opt == "--help":
			display_help = True
		elif opt == "--version":
			display_version = True
		elif opt == "--ball" or opt == "-b":
			bboot = True
		elif opt == "--bboot":
			bboot = True
		elif opt == "--mods-auto" or opt == "-m":
			mods = "auto"
		elif opt == "--mods-specify" or opt == "-M":
			mods = "specify"
		elif opt == "--mods-specify-all":
			mods = "specify-all"
		elif opt == "--gen-iso" or opt == "-i":
			gen_iso = True
		elif opt == "--run" or opt == "-r":
			run_virt = "qemu"
		elif opt == "--vnc" or opt == "-v":
			virt_graphic = "vnc"
		elif opt == "--run-term":
			virt_graphic = "term"
		elif opt == "--run-gtk":
			virt_graphic = "gtk"
		elif opt == "--clean":
			clean = "builds"
		elif opt == "--release":
			build_profile = "release"
#		else:
#			wrong_options.append(opt)

#	if len(wrong_options) > 0:
#		wopts = wrong_options[0]

#		for a in range(1, len(wrong_options)):

#			wopts += wrong_options[a]

#		print(log(0, "err", "these options are wrong: " + wopts))
#		cont = False

except:
	print(log(0, "err", "argument parsing error"))
	cont = False

# setting some variables / constants
build_profile_dir = build_profile

if build_profile == "dev":
	build_profile_dir = "debug"

# displaying help message
if cont and display_help:

	print(term_color(regular_text_color) + "info:")
	print("	python script for managing \"nierdzewny\" operating system project")
	print("usage:")
	print("	./manage.py [OPTIONS]")
	print("	regular arguments are not supported")
	print("options:")
	print("	--help	display this message")
	print("	--version	display version information")
	print("	--ball	-b	build all code (for now the same as --bboot)")
	print("	--bboot	build boot including built in modules")
	print("	--mods-auto -m	link modules to boot automatically, includes nearly all modules")
	print("	--mods-specify -M	you specify modules and basic configuration, core modules are included by default")
	print("	--mods-specify-all	you specify all modules and full configuration")
	print("	--release	select release profile when building boot")
	print("	--iso-img -i	generate runnable iso image with grub")
	print("	--iso-run -r	run generated iso image in virtual machine (qemu)")
	print("	--run-term -t	run virtual machine in terminal")
	print("	--run-gtk	run virtual machine in gtk window (default)")
	print("	--run-vnc -v	start vnc viewer (vncviewer) when running virtual machine TODO")
	print("	--clean	clean all builds")
	print("script running sequence:")
	print("	building: boot")

	cont = False


# displaying version
if cont and display_version:

	print(term_color(regular_text_color) + "nierdzewny:	version 0.7, original author and currently only is `TANDEX` (name Jonatan), licence: `open source`")
	print("manage.py script:	listed version is the same as nierdzewny, author the same too, licence: same as whole project")

	cont = False


# cleaning builds and others
if cont and clean != "no":
	print(log(0, "info", "cleaning builds"))

	cont = run_comm(["rm", "-r", ppath + "boot/boot/rust/target", ppath + "out"], 1, "rm command failed to remove boot/boot/rust/target directory, exit code ")


# importing modules to boot
glob_calls_fnn = [] # fn name
glob_calls_mn = [] # mod name
glob_calls_ufnn = ["early_init", "init", "shutdown", "stop_machine", "reboot_machine", "panic", "timer_int", "keyboard_int", "mouse_int"] # unique fn name
mods_name = []
mods_desc = []
if cont and mods != "no":
	print(log(0, "info", "linking boot modules"))
	print(log(1, "info", "parsing"))

	for module in os.listdir(ppath + "boot/mods"):
		mod_name = module
		mod_desc = "<no module description>"
		mod_include = "never" # possible values are: (always, default yes, default no, never)

		loc_glob_calls_fnn = []
		loc_glob_calls_mn = []

		module_cfg_file = open(ppath + "boot/mods/" + module + "/module.cfg")
		module_cfg = conf(module_cfg_file.read())
		module_cfg_file.close()

		for a in range(0, len(module_cfg[0])):
			mod_cfg_cg = module_cfg[0][a]
			mod_cfg_fn = module_cfg[1][a]
			mod_cfg_fv = module_cfg[2][a]

			if mod_cfg_cg == "any":
				if mod_cfg_fn == "description":
					mod_desc = mod_cfg_fv
				elif mod_cfg_fn == "include":
					if mod_cfg_fv == "never" or mod_cfg_fv == "default no" or mod_cfg_fv == "default yes" or mod_cfg_fv == "always":
						mod_include = mod_cfg_fv
					else:
						print(log(2, "err", "invalid include value \"" + mod_cfg_fv + "\" in module \"" + mod_name + "\""))
						cont = False
			elif mod_cfg_cg == "global calls":
				loc_glob_calls_fnn.append(mod_cfg_fn)
				loc_glob_calls_mn.append(mod_name)

		include_it = True
		if mod_include == "never" or mod_include == "default no":
			include_it = False

		if (mods == "specify" and (mod_include == "default no" or mod_include == "default yes")) or mods == "specify-all":

			inp = input(log(2, "question", "include \"" + mod_name + "\" module being \"" + mod_desc + "\" (yn)"))
			if inp == "y" or inp == "yes":
				include_it = True
			else:
				include_it = False

		if include_it:
			mods_name.append(mod_name)
			mods_desc.append(mod_desc)
			for loc_glob_call in loc_glob_calls_fnn:
				glob_calls_fnn.append(loc_glob_call)
			for loc_glob_call in loc_glob_calls_mn:
				glob_calls_mn.append(loc_glob_call)

#		print("name: " + mod_name + ", desc: " + mod_desc + ", include: " + mod_include)

if cont and mods != "no":

	for glob_call_fnn in glob_calls_fnn:
		unique = True

		for glob_call_ufnn in glob_calls_ufnn:

			if glob_call_fnn == glob_call_ufnn:

				unique = False

		if unique:
			glob_calls_ufnn.append(glob_call_fnn)

	print(log(1, "info", "importing"))

	cont = run_comm(["rm", "-r", ppath + "boot/boot/rust/src/mods"], 2, "failed to remove directory content, exit code ")

if cont and mods != "no":
	cont = run_comm(["mkdir", ppath + "boot/boot/rust/src/mods"], 2, "failed to recreate directory, exit code ")

if cont and mods != "no":

	for mod in mods_name:

		if cont:
			cont = run_comm(["ln", "-s", ppath + "boot/mods/" + mod + "/src", ppath + "boot/boot/rust/src/mods/" + mod], 2, "symbolic link creating failed, ln exit code ")

if cont and mods != "no":
	genfile = "#!/bin/nano\n// file generated by `manage.py` script\n\n"

	for mod in mods_name:
		genfile += "pub mod " + mod + ";\n"

	genfile += "\n"

	for glob_call in glob_calls_ufnn:

		genfile += "pub fn " + glob_call + "() {\n\n"

		a = 0
		while a < len(glob_calls_fnn):

			if glob_call == glob_calls_fnn[a]:

				genfile += "\t" + glob_calls_mn[a] + "::" + glob_call + "();\n"

			a += 1

		genfile += "\n}\n\n"

	realgenfile = open(ppath + "boot/boot/rust/src/mods/mod.rs", "w")
	realgenfile.write(genfile)
	realgenfile.close()

# building boot, assembly part (entry)
if cont and bboot:
	print(log(0, "info", "building boot"))
	print(log(1, "info", "building assembly part"))

	cont = run_comm(["nasm", "-f", "elf64", "-o", ppath + "out/build/boot/entry.o", ppath + "boot/boot/asm/" + build_arch + "/entry.asm"], 2, "build failed, nasm exit code ")

# building boot, assembly part (end)
if cont and bboot:

	cont = run_comm(["nasm", "-f", "elf64", "-o", ppath + "out/build/boot/end.o", ppath + "boot/boot/asm/" + build_arch + "/end.asm"], 2, "build failed, nasm exit code ")

# copying proper rust target
if cont and bboot:

	cont = run_comm(["cp", ppath + "boot/target/" + build_arch + ".json", ppath + "boot/target.json"], 2, "target .json file copy failed, exit code ")

# building rust (main) part
if cont and bboot:
	print(log(1, "info", "building rust part"))

	cargo_args = [rh + "cargo", "build"]
	if build_profile == "release":
		cargo_args.append("--release")
	os.chdir(ppath + "boot/boot/rust")
	cargo_process = subprocess.run(cargo_args)
	os.chdir(ppath)

	if cargo_process.returncode != 0:

		print(log(2, "err", "build failed, exit code " + str(cargo_process.returncode)))
		cont = False

# linking boot parts
if cont and bboot:
	print(log(1, "info", "linking boot"))
	link_args = ["ld", "--gc-sections", "-o", ppath + "out/build/boot/" + boot_name, ppath + "out/build/boot/entry.o"]

	for object in os.listdir(ppath + "out/build/boot/objects/"):

		link_args.append(ppath + "out/build/boot/objects/" + object)

	link_args.append(ppath + "boot/boot/rust/target/target/" + build_profile_dir + "/libboot.a")
	link_args.append(ppath + "out/build/boot/end.o")
	cont = run_comm(link_args, 2, "link failed, ld exit code ")

	#ld -o out/build/boot/complete-boot out/build/boot/boot/* boot/boot/rust/target/target/$target/libboot.a

# copying done build to image directory
if cont and bboot:

	cont = run_comm(["cp", ppath + "out/build/boot/" + boot_name, ppath + "out/img/iso/boot/" + boot_name], 2, "failed to copy done build to image directory, cp exit code ")


# generate iso image
if cont and gen_iso:
	print(log(0, "info", "generating iso image"))
	cont = run_comm(["grub-mkrescue", "-o", ppath + "out/output.iso", ppath + "out/img/iso"], 1, "grub-mkrescue failed, exit code ")


# run ritual machine (qemu)
if cont and run_virt == "qemu":
	print(log(0, "info", "running virtual machine qemu"))
	qemu_args = ["qemu-system-x86_64", "-cdrom", ppath + "out/output.iso"]

	if virt_graphic == "term":
		qemu_args.append("-nographic")
	elif virt_graphic == "gtk":
		qemu_args.append("-display")
		qemu_args.append("gtk")

	cont = run_comm(qemu_args, 1, "qemu crached, exit code ")

print(log(0, "info", "end of script reached") + term_color("0"))
