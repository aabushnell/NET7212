CC = clang

all: hello 1a

hello: hello.o
	$(CC) -g -o hello hello.o

hello.o: hello.c
	$(CC) -c -g -Wall hello.c

1a: 1a.o
	$(CC) -g -o 1a 1a.o

1a.o: 1a.c
	$(CC) -c -g -Wall 1a.c

1b: 1b.o
	$(CC) -g -o 1b 1b.o

1b.o: 1b.c
	$(CC) -c -g -Wall 1b.c

1c: 1c.o
	$(CC) -g -o 1c 1c.o

1c.o: 1c.c
	$(CC) -c -g -Wall 1c.c

2a: 2a.o
	$(CC) -g -o 2a 2a.o

2a.o: 2a.c
	$(CC) -c -g -Wall 2a.c

clean:
	rm -f hello.o hello
	rm -f 1a.o 1a
	rm -f 1b.o 1b
	rm -f 1c.o 1c
	rm -f 2a.o 2a
