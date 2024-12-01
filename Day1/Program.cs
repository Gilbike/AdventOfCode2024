using Utils;

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

            leftList.Sort();
            rightList.Sort();

            // maybe not needed
            if (leftList.Count != rightList.Count)
            {
                throw new IOException();
            }

            List<int> differences = new List<int>();

            int count = leftList.Count;
            for (int index = 0; index < count; index++)
            {
                differences.Add(Math.Abs(leftList[index] - rightList[index]));
            }

            int result = differences.Sum(x => x);

            Console.WriteLine("Result:");
            Console.WriteLine(result);
        }
    }
}
