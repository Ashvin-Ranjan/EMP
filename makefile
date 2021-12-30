# Makefiles make me cry

# All is needed because for some reason it only runs the first one... why?
all: encode.o display.o emp.o emp


encode.o: types\encode.cpp
    g++ -c types\encode.cpp

types.o: types\display.cpp
    g++ -c types\display.cpp

emp.o: emp.cpp
    g++ -c emp.cpp


emp: emp.o display.o encode.o
	g++ -o emp display.o emp.o encode.o