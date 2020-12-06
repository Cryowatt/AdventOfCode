using System;

namespace AdventOfCode
{
    public abstract class SolutionBase<TInput> : IDay
    {
        private Func<string, TInput> parser;
        protected TInput input;

        protected SolutionBase(Func<string, TInput> parser)
        {
            this.parser = parser;
        }

        public virtual void Parse(string input)
        {
            this.input = parser(input);
        }

        public abstract object PartA();

        public abstract object PartB();
   }
}
