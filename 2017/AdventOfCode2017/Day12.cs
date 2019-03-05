using System.Collections.Generic;
using System.Linq;

namespace AdventOfCode2017
{
    public class Day12 : IAdventDay
    {
        public string RunPart1(string input)
        {
            var pipeMap =
                (from line in input.EnumerateLines()
                 let tokens = line.Split(" <-> ")
                 let programId = int.Parse(tokens[0])
                 let pipeTargets = tokens[1].EnumerateCells().AsInt()
                 select (programId, pipeTargets))
                      .ToDictionary(o => o.programId, o => o.pipeTargets);

            var todo = new Queue<int>();
            var seen = new HashSet<int>();
            todo.Enqueue(0);

            while (todo.Count > 0)
            {
                int pid = todo.Dequeue();
                seen.Add(pid);

                foreach (var cpid in pipeMap[pid].Except(seen).Except(todo))
                {
                    todo.Enqueue(cpid);
                }
            }

            return seen.Count.ToString();
        }

        public string RunPart2(string input)
        {
            var pipeMap =
                (from line in input.EnumerateLines()
                 let tokens = line.Split(" <-> ")
                 let programId = int.Parse(tokens[0])
                 let pipeTargets = tokens[1].EnumerateCells().AsInt()
                 select (programId, pipeTargets))
                      .ToDictionary(o => o.programId, o => o.pipeTargets);

            var seen = new HashSet<int>();
            int groupCount = 0;

            while (seen.Count < pipeMap.Count)
            {
                groupCount++;
                var todo = new Queue<int>();
                todo.Enqueue(pipeMap.Keys.Except(seen).First());

                while (todo.Count > 0)
                {
                    int pid = todo.Dequeue();
                    seen.Add(pid);

                    foreach (var cpid in pipeMap[pid].Except(seen).Except(todo))
                    {
                        todo.Enqueue(cpid);
                    }
                }
            }

            return groupCount.ToString();
        }
    }
}