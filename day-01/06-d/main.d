import std.algorithm;
import std.array;
import std.stdio;
import std.ascii : isDigit;
import std.conv : to;
import std.string : format;

void main(string[] args) {
    auto file = File(args[1]);

    writeln(file.byLine()
        .map!(line => line.filter!(c => isDigit(c)).array())
        .map!(number => format("%s%s", number[0], number[$-1]).to!int)
        .sum!()
    );

    file.close();
}
