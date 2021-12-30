# Makefiles make me cry

# All is needed because for some reason it only runs the first one... why?
all: types.o emp.o emp

types.o: types\types.cpp
    g++ -c types\types.cpp

emp.o: emp.cpp
    g++ -c emp.cpp


emp: emp.o types.o
	g++ -o emp emp.o types.o