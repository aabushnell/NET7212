CC = clang

all: hello 1a

hello: hello.o
	$(CC) -g -o hello hello.o

hello.o: hello.c
	$(CC) -c -g -Wall hello.c

1a: 1a.o
	$(CC) -g -o 1a 1a.o

1a.o: 1a.c
	$(CC) -c -g -Wall -O0 1a.c

clean:
	rm -f hello.o hello
	rm -f 1a.o 1a
