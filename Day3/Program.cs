using System.Text.RegularExpressions;
using Utils;

namespace Day3
{
    internal class Program
    {
        static void Main(string[] args)
        {
            DayReader reader = new DayReader(Days.Day3);
            string input = reader.GetInput();

            Console.WriteLine("Part 1 Result:");
            Console.WriteLine(Part1(input));

            Console.WriteLine("Part 2 Result:");
            Console.WriteLine(Part2(input));
        }

        private static int Part1(string input)
        {
            Regex rg = new Regex(@"mul\([0-9]{1,3},[0-9]{1,3}\)");

            MatchCollection matchedCalculations = rg.Matches(input);

            int sum = 0;

            foreach (Match match in matchedCalculations)
            {
                int length = match.Value.Length - 5;
                string[] rawNumbers = match.Value.Substring(4, length).Split(',');
                sum += int.Parse(rawNumbers[0]) * int.Parse(rawNumbers[1]);
            }

            return sum;
        }

        private static int Part2(string input)
        {
            Regex rg = new Regex(@"mul\([0-9]{1,3},[0-9]{1,3}\)|do\(\)|don't\(\)");

            MatchCollection matchedCalculations = rg.Matches(input);

            int sum = 0;

            bool parse = true;

            foreach (Match match in matchedCalculations)
            {
                if (match.Value == "do()")
                {
                    parse = true;
                    continue;
                }
                else if (match.Value == "don't()")
                {
                    parse = false;
                    continue;
                }

                if (!parse)
                {
                    continue;
                }

                int length = match.Value.Length - 5;
                string[] rawNumbers = match.Value.Substring(4, length).Split(',');
                sum += int.Parse(rawNumbers[0]) * int.Parse(rawNumbers[1]);
            }

            return sum;
        }
    }
}
