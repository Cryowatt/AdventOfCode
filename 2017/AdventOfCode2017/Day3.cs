using System;
using System.Linq;

namespace AdventOfCode2017
{
    public class Day3 : IAdventDay
    {
        public string RunPart1(string input) => (
            // I appologize to all who may read beyond this point
            from v in new[] { int.Parse(input) }.TakeWhile(o => o > 1)
            let index = v - 1
            let rings = EnumerableEx.Generate(0, i => true, i => i + 1, i => (Ring: i, Offset: 4 * i * i - 4 * i + 1))
            let ring = rings.TakeWhile(o => index > o.Offset).Last()
            let centerStep = Math.Abs(((index - ring.Offset + (ring.Ring * 2 + 1)) % (ring.Ring * 2)) - ring.Ring)
            select ring.Ring + centerStep
        ).SingleOrDefault().ToString();

        const int m = 31;
        const int h = 2 * m - 1;
        private readonly (int X, int Y)[] T = new(int X, int Y)[] { (1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1), (0, 1), (1, 1) };

        //http://oeis.org/A141481
        // Really there should be a state monad in here, this only sorta works accidentally
        public string RunPart2(string input) => (
            from init in EnumerableEx.Defer(() =>
                 {
                     var A = new int[h + 1, h + 1];
                     A[m, m] = 1;
                     return new[] { A };
                 })
            from n in Enumerable.Range(1, ((h - 2) * (h - 2)) - 1)
            let g = (int)Math.Sqrt(n)
            //r=(g+g%2)\2;
            let r = Math.DivRem((g + g % 2), 2, out int _)
            let q = 4 * (r * r)
            let d = n - q
            let j = ((n <= q - 2 * r) ? d + 3 * r :
               (n <= q) ? r :
               (n <= q + 2 * r) ? r - d : -r) + m
            let k = ((n <= q - 2 * r) ? r :
               (n <= q) ? -d - r :
               (n <= q + 2 * r) ? -r : d - 3 * r) + m
            let s = (from t in T
                     select init[j + t.X, k + t.Y]).Sum()
            select new { X = j, Y = k, s = init[j, k] = s }
            ).SkipWhile(o => o.s <= int.Parse(input)).First().s.ToString();
    }
}
