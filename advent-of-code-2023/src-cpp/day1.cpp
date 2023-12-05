#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <vector>
#include <numeric>
#include <map>
#include <set>

using namespace std;

vector<string> get_input() {
    ifstream file("inputs/day1.txt");

    if(!file.is_open()) {
        cerr << "Error opening file " << endl;
    }

    string line;
    vector<string> lines;

    while(std::getline(file, line)) {
        lines.push_back(line);
    }
    file.close();

    return lines;
}

vector<string> test_input_part1() {
    string input_str = "1abc2\n\
pqr3stu8vwx\n\
a1b2c3d4e5f\n\
treb7uchet";

    istringstream iss(input_str);
    vector<string> lines;

    std::string line;
    while (getline(iss, line)) {
        lines.push_back(line);
    }
    return lines;
}

vector<string> test_input_part2() {
    string input_str = "two1nine\n\
eightwothree\n\
abcone2threexyz\n\
xtwone3four\n\
4nineeightseven2\n\
zoneight234\n\
7pqrstsixteen";

    istringstream iss(input_str);
    vector<string> lines;

    std::string line;
    while (getline(iss, line)) {
        lines.push_back(line);
    }

    return lines;
}

uint32_t part1(const vector<string>& lines) {
    vector<uint32_t> numbers;
    for (auto &line: lines) {
        string filtered_line;
        copy_if(line.begin(), line.end(), back_inserter(filtered_line), [](char c) {
            return !isalpha(c);
        });    

        string number_str = filtered_line.substr(0, 1) + filtered_line.substr(filtered_line.length() - 1);
        uint32_t number = stoi(number_str);
        //cout << number << endl;
        numbers.push_back(number);
    }

    uint32_t result = accumulate(numbers.begin(), numbers.end(), 0);
    return result;
}


uint32_t part2(const vector<string>& lines) {
    map<string, uint32_t> number_map = {
        {"one", 1},
        {"two", 2},
        {"three", 3},
        {"four", 4},
        {"five", 5},
        {"six", 6},
        {"seven", 7},
        {"eight", 8},
        {"nine", 9}
    };

    vector<uint32_t> numbers;

    for (auto &line: lines) {
        // result: position number

        // find numbers in alphabet
        vector<pair<uint32_t, uint32_t>> result1;         
        for(const auto& entry: number_map) {
            size_t pos = 0;

            while((pos = line.find(entry.first, pos)) != string::npos) {
                result1.push_back(make_pair(static_cast<uint32_t>(pos), entry.second));
                pos += entry.first.length();
            }
        }

        // find numbers
        vector<pair<uint32_t, uint32_t>> result2;
        string filtered_num;
        for(size_t i = 0; i < line.length(); i++) {
            if (isdigit(line[i])) {
                result2.push_back(make_pair(static_cast<uint32_t>(i), line[i] - '0'));
            }
        }

        set<pair<uint32_t, uint32_t>> result_set(result1.begin(), result1.end());
        result_set.insert(result2.begin(), result2.end());

        uint32_t first = result_set.begin()->second;
        uint32_t last = result_set.rbegin()->second;

        //cout << line << " " << first * 10 + last << endl;
        numbers.push_back(first*10 + last);
    }

    uint32_t result = accumulate(numbers.begin(), numbers.end(), 0);
    return result;
}

int main() {
    vector<string> lines = get_input();
    cout << "PART1" << endl;
    uint32_t result_part1 = part1(lines);
    cout << "part1 reesult = " << result_part1 << endl;

    uint32_t result_part2 = part2(lines);
    cout << "part2 result = " << result_part2 << endl;
}