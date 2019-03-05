namespace AdventOfCode2017
{
    public abstract class AdventDay<TInput> : IAdventDay
    {
        string IAdventDay.RunPart1(string input) => this.RunPart1(this.TransformInput(input));

        string IAdventDay.RunPart2(string input) => this.RunPart2(this.TransformInput(input));

        protected abstract TInput TransformInput(string input);

        public abstract string RunPart1(TInput input);

        public abstract string RunPart2(TInput input);
    }
}
