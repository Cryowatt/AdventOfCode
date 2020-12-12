using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text.RegularExpressions;

namespace AdventOfCode.Calendar.Day09
{
    public unsafe class DataStructures : SolutionBase<ReadOnlyMemory<long>>
    {
        public DataStructures() : base(Parsers.Lines().Long().Memory()) { }

        public override object PartA() => FirstExploit(this.input, 25);

        public long FirstExploit(ReadOnlyMemory<long> data, int preambleLength)
        {
            var preamble = new Queue<long>(data.Slice(0, preambleLength).ToArray());
            var checkSet = new HashSet<long>(preamble);
            data = data.Slice(preambleLength);

            using (var dataHandle = data.Pin())
            {
                var p = (long*)dataHandle.Pointer;

                for (int i = 0; i < data.Length; i++)
                {
                    var current = *(p++);
                    Func<long, bool> checkRule = o =>
                    {
                        var check = current - o;
                        if (check == o)
                        {
                            return false;
                        }
                        else
                        {
                            return checkSet.Contains(check);
                        }
                    };

                    if (!checkSet.Any(checkRule))
                    {
                        return current;
                    }

                    preamble.Enqueue(current);
                    checkSet.Add(current);
                    var oldest = preamble.Dequeue();
                    checkSet.Remove(oldest);
                }
            }

            throw new InvalidOperationException();
        }

        public override object PartB()
        {
            var exploit = FirstExploit(this.input, 25);
            using (var dataHandle = this.input.Pin())
            {
                var head = (long*)dataHandle.Pointer;
                var tail = ((long*)dataHandle.Pointer) + 1;
                var end = ((long*)dataHandle.Pointer) + this.input.Length;

                long sum = *head + *tail;

                while (tail != end)
                {
                    if (sum == exploit)
                    {
                        long min = *head;
                        long max = *tail;

                        for (long* p = head; p <= tail; p++)
                        {
                            min = Math.Min(min, *p);
                            max = Math.Max(max, *p);
                        }

                        return min + max;
                    }
                    else if (sum < exploit)
                    {
                        sum += *(++tail);
                    }
                    else if (sum > exploit)
                    {
                        sum -= *(head++);
                    }
                }
            }

            throw new InvalidOperationException();
        }
    }
}
