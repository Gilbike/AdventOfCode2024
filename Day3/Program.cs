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

            Regex rg = new Regex(@"mul\([0-9]{1,3},[0-9]{1,3}\)");

            MatchCollection matchedCalculations = rg.Matches(input);

            Console.WriteLine(matchedCalculations.Count);

            int sum = 0;

            foreach (Match match in matchedCalculations)
            {
                int length = match.Value.Length - 5;
                string[] rawNumbers = match.Value.Substring(4, length).Split(',');
                sum += int.Parse(rawNumbers[0]) * int.Parse(rawNumbers[1]);
            }

            Console.WriteLine("Result:");
            Console.WriteLine(sum);
        }
    }
}
