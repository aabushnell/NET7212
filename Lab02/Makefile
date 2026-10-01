
MONEY_FLAGS = -Wall -fprofile-arcs -ftest-coverage

UP_FLAGS = -g -O0 -Wall -Wextra

all: check_money_tiny uppercase

check_money_tiny: check_money_tiny.o money.o
	gcc $(MONEY_FLAGS) -o check_money_tiny check_money_tiny.o money.o `pkg-config --libs check`

check_money_tiny.o: check_money_tiny.c
	gcc $(MONEY_FLAGS) -c -o check_money_tiny.o check_money_tiny.c

money.o: money.c
	gcc $(MONEY_FLAGS) -c -o money.o money.c

uppercase: uppercase.c
	gcc $(UP_FLAGS) -o uppercase uppercase.c

clean:
	rm -f check_money_tiny check_money_tiny.o money.o
	rm -f uppercase
