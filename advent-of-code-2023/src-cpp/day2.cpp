#include <iostream>
#include <fstream>
#include <vector>
#include <sstream>

using namespace std;

vector<string> get_input() {
    ifstream file("inputs/day2.txt");
    
    if (!file.is_open()) {
        cerr << "Error opening file " << endl;
    }

    string line;
    vector<string> lines;
    while(getline(file, line)) {
        lines.push_back(line);
    }

    return lines;
}

vector<string> get_test_input() {
    string input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    istringstream iss(input);
    vector<string> lines;

    string line;
    while (getline(iss, line)) {
        lines.push_back(line);
    }

    return lines;
}

vector<string> split(const string& input, char delimiter) {
    vector<string> result;
    stringstream ss(input);
    string temp;

    while (getline(ss, temp, delimiter)) {
        result.push_back(temp);
    }

    return result;
}

void print_vector(vector<string>& vec) {
    for (auto &v: vec) {
        cout << "\"" <<  v  << "\"" << endl;
    }
}

const int MAX_RED_CUBES = 12;
const int MAX_GREEN_CUBES = 13;
const int MAX_BLUE_CUBES = 14;

bool is_valid(string& color, int count) {
    if(color == "blue" && count <= MAX_BLUE_CUBES) {
        return true;
    } else if(color == "green" && count <= MAX_GREEN_CUBES) {
        return true;
    } else if(color == "red" && count <= MAX_RED_CUBES) {
        return true;
    } else {
        return false;
    }
}

int part1(const vector<string>& lines) {
    int result = 0;

    for(const auto& line: lines) {
        vector<string> split_result = split(line, ':');
        
        vector<string> game_name = split(split_result[0], ' ');
        string game_id = game_name[1];        

        int game_result = 0;
        vector<string> trials = split(split_result[1], ';');
        for(const auto& trial: trials) {
            // trial = "3 blue, 4 red"
            vector<string> cubes = split(trial, ',');
            for (const auto& cube: cubes) {
                vector<string> cube_val = split(cube, ' '); 
        
                if(!is_valid(cube_val[2], stoi(cube_val[1]))) {
                    game_result += 1;
                }
            }
        }

        if(game_result == 0) {
            result += stoi(game_id);
        }
    }
    return result;
}

struct Cubes {
    uint32_t blue;
    uint32_t red;
    uint32_t green;

    void update(const string& color, uint32_t count) {
        if(color == "blue" && blue < count) {
            blue = count;
        } else if(color == "red" && red < count) {
            red = count;
        } else if(color == "green" && green < count) {
            green = count;
        }
    }

    uint32_t get_result() {
        return blue * red * green;
    }
};

uint32_t part2(const vector<string>& lines) {
    int result = 0;

    for(const auto& line: lines) {
        vector<string> split_result = split(line, ':');
        
        vector<string> game_name = split(split_result[0], ' ');
        string game_id = game_name[1];        

        Cubes cube_counts = { .red = 0, .blue = 0, .green = 0};
        vector<string> trials = split(split_result[1], ';');
        for(const auto& trial: trials) {
            // trial = "3 blue, 4 red"

            vector<string> cubes = split(trial, ',');
            for (const auto& cube: cubes) {
                vector<string> cube_val = split(cube, ' '); 
                string color = cube_val[2];
                uint32_t count = stoi(cube_val[1]);
                cube_counts.update(color, count);
            }
        }

        result += cube_counts.get_result();
    }
    return result;
}

int main() {
    vector<string> lines = get_input();

    uint32_t part1_result = part1(lines);
    cout << "result: " << part1_result << endl;

    uint32_t part2_result = part2(lines);
    cout << "result: " << part2_result << endl;
}