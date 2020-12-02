namespace AdventOfCode
{
    public interface IDay
    {
        void Parse(string input);
        object PartA();
        object PartB();
    }
}
