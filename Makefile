all: build

CLIENT = cargo run --bin client
SERVER = cargo run --bin server

build:
	cargo clean
	cargo build

server:
	$(SERVER) 192.168.1.79 50000

serverip6:
	$(SERVER) fe80::8b51:f72:7e67:2390 50000

t1s:
	$(SERVER) 

t1c:
	$(CLIENT) 

t2s:
	$(SERVER) 192.168.1.79 50000 a b c

t2c:
	$(CLIENT) a a 192.168.1.79 50000 abc abc abc

t3: 
	$(SERVER) 1.1.1.1 50000

t4:
	$(CLIENT) a "aa ** ,," 192.168.1.79 50000

t5:
	$(CLIENT) a a 192.168.1.79 50000

t6:
	$(CLIENT) "ABC ** DEF 123" "may" 192.168.1.79 50000

t7:
	$(CLIENT) hello apple 192.168.1.79 50000

t8:
	$(CLIENT) hello appleapple 192.168.1.79 50000

t9:
	$(CLIENT) hellohelloooo apple 192.168.1.79 50000
	
t10:
	$(CLIENT) may may fe80::8b51:f72:7e67:2390 50000

t11:
	$(SERVER) 192.168.1.79 50000