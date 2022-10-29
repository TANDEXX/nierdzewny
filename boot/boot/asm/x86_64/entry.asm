bits 32
global grub_header, _start, load_addr
extern main_entry, boot_end
boot_start:

section .text
grub_header:
	dd 0xe85250d6 ; magic num
	dd 0 ; arch
	dd _start - grub_header ; header length
	dd 0x100000000 - (0xe85250d6 + 0 + (_start - grub_header)) ; checksum

;	I don't know how to make it work, and do it's usefull
;	dw 2 ; addr info tag, 2 bytes
;	dw 0 ; 2 bytes
;	dd 2+2+4+4+4+4+4 ; 4 bytes
;	dd grub_header ; 4 bytes
;	dd -1 ; load addr, 4 bytes
;	dd boot_end ; 4 bytes
;	dd boot_end ; 4 bytes

	dw 3 ; entry addr tag, 2 bytes
	dw 0 ; 2 bytes
	dd 2+2+4+4 ; 4 bytes
	dd _start ; 4 bytes
	dd 0 ; align, must be aligned by 8 bytes

	dw 0
	dw 0
	dd 8

_start:
	mov esp, stack_top
	mov ebp, stack_bottom

;	jmp end

	call clear_sc
	jmp _start_continue ; I don't know do this instruction is usefull

_start_continue:
	pushfd ; cpuid check
	pop eax

	mov ecx, eax
	xor eax, 1 << 21

	push eax
	popfd

	pushfd
	pop eax
	push ecx
	popfd

	cmp eax, ecx
	je cpuid_err

	mov eax, 0x80000000 ; long mode check
	cpuid
	cmp eax, 0x80000001
	jb no_64_bits

	mov eax, 0x80000001
	cpuid
	test edx, 1 << 29
	jz no_64_bits

	mov eax, p3_table ; set up page tablets
	or eax, 11b
	mov [p4_table], eax

	mov eax, p2_table
	or eax, 11b
	mov [p3_table], eax
	mov ecx, 0
	call map_2_page

	mov eax, p4_table ; enable paging
	mov cr3, eax

	mov eax, cr4
	or eax, 1 << 5
	mov cr4, eax

	mov ecx, 0xc0000080
	rdmsr
	or eax, 1 << 8
	wrmsr

	mov eax, cr0
	or eax, 1 << 31
	mov cr0, eax

	lgdt [gdt_desc]; finally enter long mode
	jmp cseg:long_entry

	jmp end

map_2_page:
	mov eax, 200000h
	mul ecx
	or eax, 10000011b
	mov [p2_table + ecx * 8], eax

	inc ecx
	cmp ecx, 512
	jne map_2_page

	ret

no_64_bits:
	push dword no_64_msg
	push dword no_64_msg_len
	jmp err

cpuid_err:
	push dword cpuid_err_msg
	push dword cpuid_err_msg_len
	jmp err

err: ; 1 32: message addr, 2 32: message length
	mov dword [0xb8000], 0x0c720c65
	mov dword [0xb8004], 0x0c6f0c72
	mov dword [0xb8008], 0x0c3a0c72
	mov ecx, [esp + 4] ; message addr

	mov eax, [esp]
	add eax, 7
	push eax
	mov dx, 0x03D4
	mov al, 0x0F
	out dx, al
	inc dl
	pop eax
	out dx, al
	push eax
	dec dl
	mov al, 0xe
	out dx, al
	inc dl
	pop eax
	shr eax, 8
	out dx, al

	mov eax, esp
	add [eax], ecx
	mov eax, 0xb800e

.0:
	cmp ecx, [esp]
	je end

	mov dl, [ecx]
	mov [eax], dl

	inc ecx
	add eax, 2
	jmp .0

clear_sc:
	mov eax, 0xb8000

.0:
	cmp eax, vga_end
	je ret

	mov dword [eax], 0x07000700

	add eax, 4
	jmp .0

end:
	hlt
	jmp end

ret:
	ret

bits 64
long_entry:
	xor ax, ax
	mov ss, ax
	mov ds, ax
	mov es, ax
	mov fs, ax
	mov gs, ax

	call main_entry

bits 32
section .data

load_addr equ 10000h
stack_top equ 4fffh
stack_bottom equ 4000h
vga_end equ 0xb8fa0
cseg equ gdt_code - gdt
cpuid_err_msg db "processor does not support cpuid (old processor)"
cpuid_err_msg_len equ $ - cpuid_err_msg
no_64_msg db "processor is 32-bit, x86_64 cpu is needed"
no_64_msg_len equ $ - no_64_msg

gdt_desc:
	dw gdt_end - gdt - 1
	dq gdt

gdt:
	dq 0

gdt_code:
	dq (1<<43) | (1<<44) | (1<<47) | (1<<53)

gdt_end:

section .bss
align 4096
p4_table resb 4096
p3_table resb 4096
p2_table resb 4096
