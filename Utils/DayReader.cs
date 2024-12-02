namespace Utils
{
    public enum Days
    {
        Day1,
        Day2
    }

    public class DayReader
    {
        private Days day;

        private string[] lines;

        public DayReader(Days day)
        {
            this.day = day;

            string path = Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "Inputs", $"{day}.txt");

            if (!File.Exists(path))
            {
                throw new FileNotFoundException();
            }

            lines = File.ReadAllLines(path);
        }

        public string[] GetInputLines()
        {
            return lines;
        }
    }
}
