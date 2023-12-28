import std.algorithm;
import std.array;
import std.stdio;
import std.conv : to;

bool is_possible(int[string] max_cubes, string data) {
    string[] groups = data.split("; ");

    foreach (group; groups) {
        string[] slices = group.split(", ");

        foreach (slice; slices) {
            string[] value = slice.split(" ");
            int cubes = value[0].to!int;
            string color = value[1];

            if (color.endsWith('\n')) {
                color = color[0..$-1];
            }

            if (cubes > max_cubes[color]) {
                return false;
            }
        }
    }

    return true;
}

void main(string[] args) {
    int[string] max_cubes = [
        "red": 12,
        "green": 13,
        "blue": 14
    ];

    auto file = File(args[1]);

    writeln(file.byLine()
        .map!(line => line[countUntil(line, " ")+1..$])
        .map!(line => line.split(": "))
        .map!(data => [data[0], data[1]])
        .filter!(data => is_possible(max_cubes, data[1].to!string))
        .map!(data => data[0].to!int)
        .sum()
    );

    file.close();
}
