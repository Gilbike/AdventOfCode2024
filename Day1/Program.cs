using Utils;

// https://adventofcode.com/2024/day/1

namespace Day1
{
    internal class Program
    {
        static void Main(string[] args)
        {
            DayReader reader = new DayReader(Days.Day1);
            string[] input = reader.GetInputLines();

            List<int> leftList = new List<int>();
            List<int> rightList = new List<int>();

            foreach (string line in input)
            {
                string[] sides = line.Split("   ");

                leftList.Add(int.Parse(sides[0]));
                rightList.Add(int.Parse(sides[1]));
            }

            Console.WriteLine("Part 1 Result:");
            Console.WriteLine(Part1(leftList, rightList));

            Console.WriteLine("Part 2 Result:");
            Console.WriteLine(Part2(leftList, rightList));
        }

        private static int Part1(List<int> leftSide, List<int> rightSide)
        {
            leftSide.Sort();
            rightSide.Sort();

            // maybe not needed
            if (leftSide.Count != rightSide.Count)
            {
                throw new IOException();
            }

            List<int> differences = new List<int>();

            int count = leftSide.Count;
            for (int index = 0; index < count; index++)
            {
                differences.Add(Math.Abs(leftSide[index] - rightSide[index]));
            }

            int result = differences.Sum(x => x);

            return result;
        }

        private static int Part2(List<int> leftSide, List<int> rightSide)
        {
            Dictionary<int, int> appereances = new Dictionary<int, int>();

            // Add keys
            foreach (int id in leftSide)
            {
                appereances.Add(id, 0);
            }

            foreach (int id in rightSide)
            {
                if (!appereances.ContainsKey(id)) {
                    continue;
                }
                appereances[id] += 1;
            }

            int result = appereances.Sum(kv => kv.Key * kv.Value);

            return result;
        }
    } 
}
