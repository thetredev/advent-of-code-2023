Console.WriteLine(File.ReadLines(args[0]).Sum(line => int.Parse($"{line.First(char.IsDigit)}{line.Last(char.IsDigit)}")));
