all:hello_word.o
	mkdir -p ./target
	g++ hello_word.o -o ./target/hello_word.o
	rm ./*.o
hello_word:hello_word.cpp
	g++ hello_word.cpp -o hello_word.o

clean:
	rm -rf ./target
