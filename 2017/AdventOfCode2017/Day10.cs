using System.Linq;
using System.Text;

namespace AdventOfCode2017
{
    public class Day10 : IAdventDay
    {
        // Public because tests and laziness
        public int ChainLength = 256;

        //gug dec 188 if zpw >= 8
        public string RunPart1(string input) =>
            input.EnumerateCells().AsInt()
                .Zip(Enumerable.Range(0, input.Length), (a, b) => (Take: a, Skip: b))
                .Scan((Pos: 0, Chain: Enumerable.Range(0, this.ChainLength)),
                    (a, b) => (
                        (a.Pos + b.Take + b.Skip) % this.ChainLength,
                        a.Chain.Repeat().Skip(a.Pos).Take(b.Take).Reverse()
                            .Concat(a.Chain.Repeat().Skip(a.Pos + b.Take).Take(this.ChainLength - b.Take))
                            .Repeat().Skip(this.ChainLength - a.Pos).Take(this.ChainLength).Memoize()
                    )
                 ).Last().Chain.Take(2).Aggregate(1, (a, b) => a * b).ToString();

        int[] Salt = new[] { 17, 31, 73, 47, 23 };
        byte[] SaltByte = new byte[] { 17, 31, 73, 47, 23 };

        public string RunPart2(string input) =>
            string.Concat(KnotHash(Encoding.ASCII.GetBytes(input)).Select(o => o.ToString("x2")));
        //string.Join("",
        //    input.Select(o => (int)o).Concat(Salt).Memoize().Repeat(64)
        //        .Zip(Enumerable.Range(0, (input.Length + Salt.Length) * 64), (a, b) => (Take: a, Skip: b))
        //        .Scan((Pos: 0, Chain: Enumerable.Range(0, this.ChainLength)),
        //            (a, b) => (
        //                (a.Pos + b.Take + b.Skip) % this.ChainLength,
        //                a.Chain.Repeat().Skip(a.Pos).Take(b.Take).Reverse()
        //                    .Concat(a.Chain.Repeat().Skip(a.Pos + b.Take).Take(this.ChainLength - b.Take))
        //                    .Repeat().Skip(this.ChainLength - a.Pos).Take(this.ChainLength).ToImmutableArray()
        //            )
        //         )
        //        .Last().Chain
        //            .Buffer(16).Select(o => ((byte)o.Aggregate(0, (a, b) => a ^ b)).ToString("x2")));

        public byte[] KnotHash(byte[] input)
        {
            var saltedInput = input.Concat(SaltByte).Memoize().Repeat(64);
            var chain = Enumerable.Range(0, 256).Select(o => (byte)o).ToArray();
            var steps = saltedInput
                .Zip(Enumerable.Range(0, (input.Length + Salt.Length) * 64), (a, b) => (Take: a, Skip: b));
            int position = 0;

            foreach (var step in steps)
            {
                for (int i = 0; i < step.Take >> 1; i++)
                {
                    int left = position + i;
                    int right = position + (step.Take - i) - 1;
                    byte t = chain[(byte)left];
                    chain[(byte)left] = chain[(byte)right];
                    chain[(byte)right] = t;
                }

                position += step.Take + step.Skip;
            }

            // Make dense
            var denseHash = new byte[16];
            for (int s = 0; s < this.ChainLength; s += 16)
            {
                byte b = 0;
                for (int i = 0; i < 16; i++)
                {
                    b ^= chain[i + s];
                }

                denseHash[s >> 4] = b;
            }

            return denseHash;
        }
    }
}