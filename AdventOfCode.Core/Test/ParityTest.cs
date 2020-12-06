using System;
using System.IO;
using System.Text.RegularExpressions;
using Xunit;

namespace AdventOfCode.Test
{
    public class ParityTest<TDay, TCheckDay>
        where TDay : IDay, new()
        where TCheckDay : IDay, new()
    {
        [Fact]
        public void PartA()
        {
            RunPart(o => o.PartA());
        }

        [Fact]
        public void PartB()
        {
            RunPart(o => o.PartB());
        }

        private void RunPart(Func<IDay, object> partFunc)
        {
            var namespaceParser = new Regex(@"Day\d+");
            var match = namespaceParser.Match(typeof(TCheckDay).Namespace);
            var dayId = match.Value;
            var input = File.ReadAllText($"Calendar/{dayId}/Input.txt");
            var checkDay = new TCheckDay();
            checkDay.Parse(input);
            var day = new TDay();
            day.Parse(input);
            Assert.Equal(partFunc(checkDay).ToString(), partFunc(day).ToString());
        }
    }
}
