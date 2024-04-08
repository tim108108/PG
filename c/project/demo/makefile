
main:main.c stringedit.o linklist.o bitwise.o
	gcc main.c stringedit.o linklist.o bitwise.o -Wall -o main.out 

stringedit.o:stringedit.h
	gcc stringedit.c -Wall -c 

linklist.o:linklist.h
	gcc linklist.c -Wall -c

bitwise.o:bitwise.h
	gcc bitwise.c -Wall -c

clean:
	rm *.o *.out
