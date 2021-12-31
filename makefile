# Makefiles make me cry

# All is needed because for some reason it only runs the first one... why?
all: encode.o emp.o display.o emp


encode.o: types\encode.cpp
    g++ -c types\encode.cpp

display.o: types\display.cpp
    g++ -c types\display.cpp

emp.o: emp.cpp
    g++ -c emp.cpp


emp: emp.o encode.o display.o
	g++ -o emp emp.o encode.o display.o