CC := g++
CFLAGS = -Wall -std=c++17 -ggdb
SRC_DIR := ./src
OBJ_DIR := ./obj
OUT_DIR := ./bin
SRC_FILES := $(wildcard $(SRC_DIR)/*.cpp)
OBJ_FILES := $(patsubst $(SRC_DIR)/%.cpp,$(OBJ_DIR)/%.o,$(SRC_FILES))

target: $(OBJ_FILES)
	$(CC) $(CFLAGS) -o $(OUT_DIR)/emp $^

$(OBJ_DIR)/%.o: $(SRC_DIR)/%.cpp
	$(CC) $(CFLAGS) -c -o $@ $<

clean:
	rm -r $(OUT_DIR)/*
	rm -r $(OBJ_DIR)/*