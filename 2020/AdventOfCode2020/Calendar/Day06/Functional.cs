using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text.RegularExpressions;

namespace AdventOfCode.Calendar.Day06
{
    public class Functional : SolutionBase<IEnumerable<IEnumerable<string>>>
    {
        private static IEnumerable<IEnumerable<string>> Parser(string input)
        {
            using (var reader = new StringReader(input))
            {
                string line;

                do
                {
                    var answers = new List<string>();
                    while (!string.IsNullOrEmpty(line = reader.ReadLine()))
                    {
                        answers.Add(line);
                    }

                    yield return answers;
                }
                while (line != null);
            }
        }

        public Functional() : base(Parser) { }

        public override object PartA() =>
            (from customsGroup in this.input
             let groupAnswers =
                 (from response in customsGroup
                  from answer in response
                  select answer)
             select groupAnswers.Distinct().Count()).Sum();


        public override object PartB() => "";
    }
}
