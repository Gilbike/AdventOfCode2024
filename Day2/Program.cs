using Utils;

namespace Day2
{
    internal class Program
    {
        static void Main(string[] args)
        {
            DayReader reader = new DayReader(Days.Day2);
            string[] input = reader.GetInputLines();

            int part1 = Part1(input);
            int part2 = Part2(input);

            Console.WriteLine("Part 1 Result:");
            Console.WriteLine(part1);

            Console.WriteLine("Part 2 Result:");
            Console.WriteLine(part2);
        }

        private static int Part1(string[] input)
        {
            int safe = 0;

            foreach (string line in input)
            {
                List<int> numbers = new List<int>();

                foreach (string raw in line.Split(' '))
                {
                    numbers.Add(int.Parse(raw));
                }

                if (IsRowSafe(numbers))
                {
                    safe++;
                }
            }

            return safe;
        }

        private static int Part2(string[] input)
        {
            int safe = 0;

            foreach (string line in input)
            {
                List<int> numbers = new List<int>();

                foreach (string raw in line.Split(' '))
                {
                    numbers.Add(int.Parse(raw));
                }
                
                for (int index = 0; index < numbers.Count; index++)
                {
                    List<int> attemptedNumbers = new List<int>(numbers);
                    attemptedNumbers.RemoveAt(index);
                    if (IsRowSafe(attemptedNumbers))
                    {
                        safe++;
                        break;
                    }
                }
            }

            return safe;
        }

        private static bool IsRowSafe(List<int> numbers)
        {
            if (!numbers.SequenceEqual(numbers.OrderBy(x => x)) && !numbers.SequenceEqual(numbers.OrderByDescending(x => x)))
            {
                return false;
            }

            bool invalid = false;
            List<int> ordered = numbers.OrderByDescending(x => x).ToList();
            for (int index = 0; index < ordered.Count - 1; index++)
            {
                int diff = ordered[index] - ordered[index + 1];
                if (diff > 3 || diff < 1)
                {
                    invalid = true;
                    break;
                }
            }

            if (invalid)
            {
                return false;
            }

            return true;
        }
    }
}
