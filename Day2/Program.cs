using Utils;

namespace Day2
{
    internal class Program
    {
        static void Main(string[] args)
        {
            DayReader reader = new DayReader(Days.Day2);
            string[] input = reader.GetInputLines();

            int safe = 0;

            foreach (string line in input)
            {
                List<int> numbers = new List<int>();

                foreach(string raw in line.Split(' '))
                {
                    numbers.Add(int.Parse(raw));
                }

                if (!numbers.SequenceEqual(numbers.OrderBy(x => x)) && !numbers.SequenceEqual(numbers.OrderByDescending(x => x)))
                {
                    continue;
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
                    continue;
                }

                safe++;
            }

            Console.WriteLine("Result:");
            Console.WriteLine(safe);
        }
    }
}
