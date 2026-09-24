CC = gcc

all: hello

hello: hello.o
	$(CC) -g -o hello hello.o

hello.o: hello.c
	$(CC) -c -g -Wall hello.c

clean:
	rm -f hello.o hello
