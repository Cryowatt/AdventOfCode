namespace AdventOfCode.Calendar.Day03
{
    public class StringOps : SolutionBase<string[]>
    {
        public StringOps() : base(Parsers.Lines().ToArray()) { }

        public override object PartA() => CountTrees(new Point(3, 1));

        private int CountTrees(Point velocity)
        {
            int width = this.input[0].Length;
            int trees = 0;

            for (var position = new Point(0, 0); position.Y < this.input.Length; position += velocity)
            {
                if (this.input[position.Y][position.X % width] == '#')
                {
                    trees++;
                }
            }

            return trees;
        }

        public override object PartB() =>
            (long)CountTrees(new Point(1, 1)) *
            CountTrees(new Point(3, 1)) *
            CountTrees(new Point(5, 1)) *
            CountTrees(new Point(7, 1)) *
            CountTrees(new Point(1, 2));
    }
}
