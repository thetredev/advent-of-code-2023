namespace aoc_day2;


public static class Program
{
    private static readonly Dictionary<string, int> MAX_CUBES = new()
    {
        { "red", 12 },
        { "green", 13 },
        { "blue", 14 },
    };

    private static bool IsPossible(string[] groups)
    {
        foreach (string group in groups)
        {
            string[] slices = group.Split(", ");

            foreach (string value in slices)
            {
                string[] slice = value.Split(' ');
                string cubes = slice[0];
                string color = slice[1];

                if (cubes.EndsWith('\n'))
                {
                    cubes = cubes[..^-1];
                }

                if (int.Parse(cubes) > MAX_CUBES[color])
                {
                    return false;
                }
            }
        }

        return true;
    }

    public static void Main(string[] args)
    {
        int currentGameId = 0;

        Console.WriteLine(File.ReadLines(args[0]).Sum(line =>
        {
            currentGameId++;

            var data = line[(line.IndexOf(':') + 2)..];
            var groups = data.Split("; ");

            return IsPossible(groups) ? currentGameId : 0;
        }));
    }
}
