using System;
using System.Collections.Concurrent;
using System.Collections.Generic;
using System.Collections.Specialized;
using System.Diagnostics;
using System.IO;
using System.Linq;
using System.Security.Cryptography;
using System.Text;
using System.Text.RegularExpressions;
using System.Threading;
using System.Threading.Tasks;
using AdventOfCode.Properties;
using System.Reflection.Emit;
using System.Collections;

namespace AdventOfCode
{
    class Program
    {
        static void Main(string[] args)
        {
            //Day1();
            //Day2();
            //Day3();
            //Day4();
            //Day5(); // Too fucking slow
            //Day6();
            //Day7();
            //Day8(); // Animation is slow-ish
            //Day9();
            //Day10().Wait();
            //Day11();
            //Day12();
            //Day13();
            //Day14();
            Day15();
        }

        private static void Day15()
        {
            //Disc #1 has 13 positions; at time=0, it is at position 1.
            //Disc #2 has 19 positions; at time=0, it is at position 10.
            //Disc #3 has 3 positions; at time=0, it is at position 2.
            //Disc #4 has 7 positions; at time=0, it is at position 1.
            //Disc #5 has 5 positions; at time=0, it is at position 3.
            //Disc #6 has 17 positions; at time=0, it is at position 5.
            int[] positions = { 13, 19, 3, 7, 5, 17, 11 };
            int[] offset = { 1, 10, 2, 1, 3, 5, 0 };
            int count = 7;

            var f = from time in Enumerable.Range(0, int.MaxValue)
                    let machineState = from disk in Enumerable.Range(0, count)
                                       select (offset[disk] + time + disk + 1) % positions[disk]
                    where machineState.All(o => o == 0)
                    select time;

            Console.Write("Time {0}", f.First());
        }

        static ConcurrentBag<MD5> md5Pool = new ConcurrentBag<MD5>();

        private static string Hash(int index)
        {
            //const string salt = "abc";
            const string salt = "qzyelonm";
            return Hash(salt + index);
        }

        private static string Hash(string thing)
        {
            MD5 algo;
            if (!md5Pool.TryTake(out algo))
            {
                algo = MD5.Create();
            }

            byte[] buffer = Encoding.ASCII.GetBytes(thing);
            buffer = algo.ComputeHash(buffer);
            md5Pool.Add(algo);
            return BitConverter.ToString(buffer).Replace("-", "").ToLower();
        }

        private static string HashStretch(int index)
        {
            string h = Hash(index);

            for (int i = 0; i < 2016; i++)
            {
                h = Hash(h);
            }

            return h;
        }

        public class PotentialKey
        {
            public int Index;
            public char Key;
            public bool IsKey;
            public bool IsValidator;
            public string Hash;
        }

        private static void Day14()
        {
            Regex repeats = new Regex(@"(\w)\1{2,}");

            var interestingKeys = from index in Enumerable.Range(0, int.MaxValue).AsParallel().AsOrdered()
                                  let hash = HashStretch(index)
                                  let matches = repeats.Matches(hash).OfType<Match>().ToList()
                                  from repeat in matches
                                  where repeat.Success
                                  let key = new PotentialKey
                                  {
                                      Index = index,
                                      Key = repeat.Groups[1].Value[0],
                                      IsKey = matches.First() == repeat,
                                      IsValidator = repeat.Value.Length >= 5,
                                      Hash = hash
                                  }
                                  where key.IsKey || key.IsValidator
                                  select key;

            HashSet<PotentialKey> keyQueue = new HashSet<PotentialKey>();
            List<PotentialKey> confirmed = new List<PotentialKey>();
            var interestingKeysEnumerator = interestingKeys.GetEnumerator();

            while (confirmed.Count < 80 || keyQueue.Min(o => o.Index) < confirmed.ElementAt(63).Index + 1000)
            {
                interestingKeysEnumerator.MoveNext();
                var current = interestingKeysEnumerator.Current;
                keyQueue.RemoveWhere(o => o.Index + 1000 <= current.Index);

                if (current.IsValidator)
                {
                    var confirmedBatch = keyQueue.Where(o => o.Key == current.Key).ToList();
                    if (confirmedBatch.Count > 0)
                    {
                        Console.WriteLine("Confirmed {0} Key(s)", confirmedBatch.Count);
                    }
                    confirmed.AddRange(confirmedBatch);
                    keyQueue.ExceptWith(confirmedBatch);
                }

                if (current.IsKey)
                {
                    keyQueue.Add(current);
                }
            }

            var list = confirmed.OrderBy(o => o.Index).ToList();

            Console.WriteLine("{0} {1}", list.First().Index, list.First().Hash);
            Console.WriteLine(list.ElementAt(63).Index);
        }

        private static void Day13()
        {
            for (int y = 0; y < 50; y++)
            {
                for (int x = 0; x < 50; x++)
                {
                    Console.CursorLeft = x;
                    Console.CursorTop = y;
                    bool isOpen = new BitArray(new[] { (x * x + 3 * x + 2 * x * y + y + y * y) + 1358 }).OfType<bool>().Count(b => b) % 2 == 0;
                    Console.BackgroundColor = Console.ForegroundColor = isOpen ? ConsoleColor.Black : ConsoleColor.White;
                    Console.Write(isOpen ? ' ' : '#');
                }
            }

            Console.ResetColor();

            Point start = new Point { X = 1, Y = 1 };
            Point end = new Point { X = 31, Y = 39 };
            Queue<Point> q = new Queue<Point>();
            HashSet<Point> closed = new HashSet<Point>();
            Dictionary<Point, int> dist = new Dictionary<Point, int>();
            Dictionary<Point, Point> prev = new Dictionary<Point, Point>();
            dist[start] = 0;
            q.Enqueue(start);
            int checks = 0;
            Stopwatch timer = Stopwatch.StartNew();

            while (true)
            {
                checks++;
                var u = q.Dequeue();
                if (closed.Contains(u))
                {
                    continue;
                }

                if (timer.Elapsed.TotalSeconds > 1)
                {
                    Console.CursorTop = 0;
                    Console.WriteLine("{0} checks/s", checks);
                    checks = 0;
                    timer = Stopwatch.StartNew();
                }

                closed.Add(u);

                if (u == end)
                {
                    int count = 0;

                    foreach (var item in EnumeratePath(prev, u).Reverse())
                    {
                        Console.CursorLeft = item.X;
                        Console.CursorTop = item.Y;
                        Console.Write("0");
                        count++;
                    }

                    Console.WriteLine("Total steps {0}", count);
                    break;
                }

                foreach (var v in u.Adjacent().Where(o => o.X >= 0 && o.Y >= 0 &&
                    new BitArray(new[] { (o.X * o.X + 3 * o.X + 2 * o.X * o.Y + o.Y + o.Y * o.Y) + 1358 }).OfType<bool>().Count(b => b) % 2 == 0))
                {
                    q.Enqueue(v);
                    var alt = dist[u] + 1;
                    int newDist;

                    if (alt < (dist.TryGetValue(v, out newDist) ? newDist : int.MaxValue))
                    {
                        dist[v] = alt;
                        prev[v] = u;
                    }
                }
            }

            q.Clear();
            closed.Clear();
            dist.Clear();
            dist[start] = 0;
            q.Enqueue(start);

            while (q.Count > 0)
            {
                var u = q.Dequeue();
                closed.Add(u);

                if (dist[u] == 50)
                {
                    continue;
                }

                foreach (var v in u.Adjacent().Where(o => o.X >= 0 && o.Y >= 0 &&
                    new BitArray(new[] { (o.X * o.X + 3 * o.X + 2 * o.X * o.Y + o.Y + o.Y * o.Y) + 1358 }).OfType<bool>().Count(b => b) % 2 == 0))
                {
                    var alt = dist[u] + 1;
                    int prevDist;

                    if (alt < (dist.TryGetValue(v, out prevDist) ? prevDist : int.MaxValue))
                    {
                        q.Enqueue(v);
                        dist[v] = alt;
                    }
                }
            }
            ConsoleColor[] color = {
                /*ConsoleColor.DarkRed,
                ConsoleColor.DarkYellow,
                ConsoleColor.DarkGreen,
                ConsoleColor.DarkCyan,
                ConsoleColor.DarkBlue,
                ConsoleColor.DarkMagenta,*/
                ConsoleColor.Red,
                ConsoleColor.Yellow,
                ConsoleColor.Green,
                ConsoleColor.Cyan,
                ConsoleColor.Blue,
                ConsoleColor.Magenta };
            //R Y G C B M R Y G C B M
            foreach (var item in closed)
            {
                Console.CursorLeft = item.X;
                Console.CursorTop = item.Y;
                var distance = dist[item];
                if (distance < 50)
                {
                    Console.ForegroundColor = color[distance % color.Length];
                }
                else
                {
                    Console.ForegroundColor = ConsoleColor.White;
                }
                Console.Write((char)('A' + (distance % 26)));
            }

            Console.WriteLine("Spots in 50 steps: " + dist.Count(o => o.Value <= 50));
        }

        private static void Load(ILGenerator generator, string target)
        {
            int value;
            if (int.TryParse(target, out value))
            {
                generator.Emit(OpCodes.Ldc_I4, value);
            }
            else
            {
                int source = target[0] - 'a';
                EmitLdloc(generator, source);
            }
        }

        private static void Day12()
        {
            Regex parser = new Regex("^(?<Op>[a-z]{3})(?: (?<Arg>-?[a-z0-9]+))+");
            Type[] methodArgs = { };
            DynamicMethod run = new DynamicMethod("Run", typeof(int), methodArgs, typeof(Program).Module);
            ILGenerator generator = run.GetILGenerator();
            for (int i = 0; i < 4; i++)
            {
                generator.DeclareLocal(typeof(int));
                generator.Emit(OpCodes.Ldc_I4, (i == 2) ? 1 : 0);
            }
            generator.Emit(OpCodes.Stloc_3);
            generator.Emit(OpCodes.Stloc_2);
            generator.Emit(OpCodes.Stloc_1);
            generator.Emit(OpCodes.Stloc_0);
            List<Label> labels = Resources.Day12.Split('\n').TakeWhile(o => o != "").Select(o => generator.DefineLabel()).ToList();
            labels.Add(generator.DefineLabel());
            int instruction = 0;

            foreach (var line in Resources.Day12.Split('\n'))
            {
                Match m = parser.Match(line);
                if (!m.Success)
                {
                    break;
                }

                var args = m.Groups["Arg"];
                string firstArg = args.Captures[0].Value;
                int target;
                generator.MarkLabel(labels[instruction]);
                Load(generator, firstArg);

                switch (m.Groups["Op"].Value)
                {
                    case "cpy":
                        target = args.Captures[1].Value[0] - 'a';
                        EmitStloc(generator, target);
                        break;
                    case "inc":
                        generator.Emit(OpCodes.Ldc_I4_1);
                        generator.Emit(OpCodes.Add);
                        EmitStloc(generator, firstArg[0] - 'a');
                        break;
                    case "dec":
                        generator.Emit(OpCodes.Ldc_I4_1);
                        generator.Emit(OpCodes.Sub);
                        EmitStloc(generator, firstArg[0] - 'a');
                        break;
                    case "jnz":
                        int jump = int.Parse(args.Captures[1].Value);
                        generator.Emit(OpCodes.Brtrue, labels[instruction + jump]);
                        break;
                }

                instruction++;
            }

            generator.MarkLabel(labels[instruction]);
            generator.Emit(OpCodes.Ldloc_0);
            generator.Emit(OpCodes.Ret);

            Func<int> func = (Func<int>)run.CreateDelegate(typeof(Func<int>));
            Console.WriteLine(func());
        }

        private static void EmitLdloc(ILGenerator generator, int source)
        {
            switch (source)
            {
                case 0:
                    generator.Emit(OpCodes.Ldloc_0);
                    break;
                case 1:
                    generator.Emit(OpCodes.Ldloc_1);
                    break;
                case 2:
                    generator.Emit(OpCodes.Ldloc_2);
                    break;
                case 3:
                    generator.Emit(OpCodes.Ldloc_3);
                    break;
            }
        }

        private static void EmitStloc(ILGenerator generator, int target)
        {
            switch (target)
            {
                case 0:
                    generator.Emit(OpCodes.Stloc_0);
                    break;
                case 1:
                    generator.Emit(OpCodes.Stloc_1);
                    break;
                case 2:
                    generator.Emit(OpCodes.Stloc_2);
                    break;
                case 3:
                    generator.Emit(OpCodes.Stloc_3);
                    break;
            }
        }

        private static void Day11()
        {
            var state = new ElevatorState();

            //The first floor contains a thulium generator, a thulium-compatible microchip, a plutonium generator, and a strontium generator.
            //The second floor contains a plutonium-compatible microchip and a strontium-compatible microchip.
            //The third floor contains a promethium generator, a promethium-compatible microchip, a ruthenium generator, and a ruthenium - compatible microchip.
            //The fourth floor contains nothing relevant.
            state[Elements.Human] = 0;
            state[Elements.TmGen] = 0;
            state[Elements.Tm] = 0;
            state[Elements.PuGen] = 0;
            state[Elements.SrGen] = 0;

            state[Elements.Pu] = 1;
            state[Elements.Sr] = 1;

            state[Elements.PmGen] = 2;
            state[Elements.Pm] = 2;
            state[Elements.RuGen] = 2;
            state[Elements.Ru] = 2;

            state[Elements.ElGen] = 0;
            state[Elements.El] = 0;
            state[Elements.LiGen] = 0;
            state[Elements.Li] = 0;

            SolveElevator3(state);
            //Console.WriteLine("Min moves {0}", SolveElevator(history));
        }

        private static void SolveElevator3(ElevatorState start)
        {
            Queue<ElevatorState> q = new Queue<ElevatorState>();
            HashSet<ElevatorState> closed = new HashSet<ElevatorState>();
            Dictionary<ElevatorState, int> dist = new Dictionary<ElevatorState, int>();
            Dictionary<ElevatorState, ElevatorState> prev = new Dictionary<ElevatorState, ElevatorState>();
            dist[start] = 0;
            q.Enqueue(start);
            int checks = 0;
            Stopwatch timer = Stopwatch.StartNew();

            while (true)
            {
                checks++;
                var u = q.Dequeue();
                if (closed.Contains(u))
                {
                    continue;
                }

                if (timer.Elapsed.TotalSeconds > 1)
                {
                    Console.CursorTop = 0;
                    Console.WriteLine("{0} checks/s", checks);
                    checks = 0;
                    timer = Stopwatch.StartNew();
                    PrintState(u);
                }

                closed.Add(u);

                if (u.IsDone())
                {
                    int count = 0;

                    foreach (var item in EnumeratePath(prev, u).Reverse())
                    {
                        count++;
                        PrintState(item);
                    }

                    Console.WriteLine("Total steps {0}", count);
                    return;
                }

                foreach (var v in EnumerateValidMoves(u))
                {
                    q.Enqueue(v);
                    var alt = dist[u] + 1;
                    int newDist;

                    if (alt < (dist.TryGetValue(v, out newDist) ? newDist : int.MaxValue))
                    {
                        dist[v] = alt;
                        prev[v] = u;
                    }
                }
            }
        }

        private static IEnumerable<T> EnumeratePath<T>(IDictionary<T, T> cameFrom, T current)
        {
            yield return current;

            while (cameFrom.TryGetValue(current, out current))
            {
                yield return current;
            }
        }

        static Stopwatch timer;
        static int frameLimit = 0;
        private static void PrintState(ElevatorState state)
        {
            if (frameLimit++ % 1000 != 0)
            {
                //return;
            }

            if (timer == null)
            {
                timer = Stopwatch.StartNew();
            }

            //Console.CursorTop = 20;
            for (int i = 3; i >= 0; i--)
            {
                Console.Write("Fl{0}|", i);
                foreach (var item in state.Keys)
                {
                    Console.Write("{0,5}", (state[item] == i) ? item.ToString().Substring(0, Math.Min(item.ToString().Length, 3)) : string.Empty);
                }

                Console.WriteLine();
            }

            timer.Restart();
        }

        private static IEnumerable<ElevatorState> EnumerateValidMoves(ElevatorState currentState)
        {
            int myFloor = currentState[Elements.Human];

            if (myFloor < 3)
            {
                // Going up
                var newFloor = myFloor + 1;
                foreach (var newState in currentState.EnumerateMoves(myFloor, newFloor))
                {
                    yield return newState;
                }
            }

            if (myFloor > 0)
            {
                // Going down
                var newFloor = myFloor - 1;
                foreach (var newState in currentState.EnumerateMoves(myFloor, newFloor))
                {
                    yield return newState;
                }
            }
        }

        private static async Task Day10()
        {
            Dictionary<int, Robot> robots = new Dictionary<int, Robot>();
            Dictionary<int, Robot> outputs = new Dictionary<int, Robot>();

            for (int i = 0; i < 210; i++)
            {
                robots.Add(i, new Robot(i, robots, outputs));
            }

            for (int i = 0; i < 21; i++)
            {
                outputs.Add(i, new Robot(i, robots, outputs));
            }

            Regex itemParser = new Regex(@"value (?<Item>\d+) goes to bot (?<Target>\d+)");

            foreach (var line in Resources.Day10.Split('\n'))
            {
                RobotInstruction instruction;
                Match itemInstruction = itemParser.Match(line);
                if (itemInstruction.Success)
                {
                    int item = int.Parse(itemInstruction.Groups["Item"].Value);
                    int target = int.Parse(itemInstruction.Groups["Target"].Value);
                    robots[target].GiveValue(item);
                }
                else if (RobotInstruction.TryParse(line, out instruction))
                {
                    robots[instruction.RobotId].QueueInstruction(instruction);
                }
                else if (line == "")
                {
                    break;
                }
                else
                {
                    throw new InvalidOperationException();
                }
            }

            while (Interlocked.Read(ref Robot.RunningBots) > 0)
            {
                await Task.Delay(100);
            }
        }

        private static void Day9()
        {
            Console.Clear();

            Test("ADVENT\n", Decompress);
            Test("A(1x5)BC\n", Decompress);
            Test("(3x3)XYZ\n", Decompress);
            Test("A(2x2)BCD(2x2)EFG\n", Decompress);
            Test("(6x1)(1x3)A\n", Decompress);
            Test("X(8x2)(3x3)ABCY\n", Decompress);

            Console.WriteLine("Data size for {0}: {1}", "(3x3)XYZ", Decompress2("(3x3)XYZ"));
            Console.WriteLine("Data size for {0}: {1}", "X(8x2)(3x3)ABCY", Decompress2("X(8x2)(3x3)ABCY"));
            Console.WriteLine("Data size for {0}: {1}", "(27x12)(20x12)(13x14)(7x10)(1x12)A", Decompress2("(27x12)(20x12)(13x14)(7x10)(1x12)A"));
            Console.WriteLine("Data size for {0}: {1}", "(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN", Decompress2("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"));

            Console.WriteLine("Day 9 decompressed length: " + Decompress(Resources.Day9).Count());
            Console.WriteLine("Day 9 decompressed length: " + Decompress2(Resources.Day9));
        }

        private static void Test(string data, Func<string, IEnumerable<char>> func)
        {
            Console.WriteLine("Decompressed length: " + func(data).LongCount());
        }

        private static long Decompress2(IEnumerable<char> dataStream)
        {
            //Console.WriteLine("Stack depth: {0}", new StackTrace().FrameCount);
            long charCount = 0;

            var dataEnumerator = dataStream.GetEnumerator();
            while (dataEnumerator.MoveNext())
            {
                if (dataEnumerator.Current == '(')
                {
                    int length = ReadInt(dataEnumerator);
                    Debug.Assert(dataEnumerator.Current == 'x');
                    int count = ReadInt(dataEnumerator);
                    Debug.Assert(dataEnumerator.Current == ')');
                    var data = Decompress2(ReadString(dataEnumerator, length));

                    charCount += data * count;
                }
                else if ('A' <= dataEnumerator.Current && dataEnumerator.Current <= 'Z')
                {
                    charCount++;
                }
                else if (dataEnumerator.Current == '\n')
                {
                    break;
                }
                else
                {
                    throw new InvalidOperationException();
                }
            }

            return charCount;
        }

        private static IEnumerable<char> Decompress(IEnumerable<char> dataStream)
        {
            var dataEnumerator = dataStream.GetEnumerator();
            while (dataEnumerator.MoveNext())
            {
                if (dataEnumerator.Current == '(')
                {
                    int length = ReadInt(dataEnumerator);
                    Debug.Assert(dataEnumerator.Current == 'x');
                    int count = ReadInt(dataEnumerator);
                    Debug.Assert(dataEnumerator.Current == ')');
                    string data = ReadString(dataEnumerator, length);

                    for (int i = 0; i < count; i++)
                    {
                        foreach (char c in data)
                        {
                            yield return c;
                        }
                    }
                }
                else if ('A' <= dataEnumerator.Current && dataEnumerator.Current <= 'Z')
                {
                    yield return dataEnumerator.Current;
                }
                else if (dataEnumerator.Current == '\n')
                {
                    yield break;
                }
                else
                {
                    throw new InvalidOperationException();
                }
            }
        }

        private static StringBuilder buffer = new StringBuilder();

        private static string ReadString(IEnumerator<char> dataEnumerator, int length)
        {
            buffer.Clear();

            for (int i = 0; i < length && dataEnumerator.MoveNext(); i++)
            {
                buffer.Append(dataEnumerator.Current);
            }

            return buffer.ToString();
        }

        private static int ReadInt(IEnumerator<char> dataEnumerator)
        {
            buffer.Clear();

            while (dataEnumerator.MoveNext())
            {
                var c = dataEnumerator.Current;
                if ('0' <= c && c <= '9')
                {
                    buffer.Append(c);
                }
                else
                {
                    break;
                }
            }

            return int.Parse(buffer.ToString());
        }

        private static void Day8()
        {
            var instructionSet = new Dictionary<string, Action<bool[,], int, int>> {
                {
                    "rect", (g, a, b)=> {
                        for(int y = 0; y < b; y++)
                        {
                            for(int x = 0; x < a; x++)
                            {
                                g[y,x]=true;
                            }
                        }
                    }
                },
                {
                    "rotate row", (g, y, o)=> {
                        var offset = (from x in Enumerable.Range(0, g.GetLength(1))
                                     select g[y,x]).ToList();

                        for(int x= 0; x < g.GetLength(1); x++)
                        {
                            g[y, (x + o) % g.GetLength(1)] = offset[x];
                        }
                    }
                },
                {
                    "rotate column", (g, x, o)=> {
                        var offset = (from y in Enumerable.Range(0, g.GetLength(0))
                                     select g[y, x]).ToList();

                        for(int y= 0; y < g.GetLength(0); y++)
                        {
                            g[(y + o) % g.GetLength(0), x] = offset[y];
                        }
                    }
                }
            };
            Regex parser = new Regex(@"((?<Op>rect)|(?<Op>rotate (?:row|column)) (?:x|y)\s?=)\s?(?<A>\d+)(?:x| by )(?<B>\d+)");
            bool[,] grid = new bool[6, 50];

            foreach (var command in Resources.Day8.Split(new[] { '\n' }, StringSplitOptions.RemoveEmptyEntries))
            {
                Match tokens = parser.Match(command);
                string op = tokens.Groups["Op"].Value;
                int a = int.Parse(tokens.Groups["A"].Value);
                int b = int.Parse(tokens.Groups["B"].Value);

                instructionSet[op](grid, a, b);
                PrintGrid(grid);
            }

            Console.WriteLine("On lights: {0}", grid.OfType<bool>().Count(o => o));
        }

        private static void PrintGrid(bool[,] grid)
        {
            Console.CursorTop = 0;
            Console.CursorLeft = 0;

            for (int y = 0; y < grid.GetLength(0); y++)
            {
                for (int x = 0; x < grid.GetLength(1); x++)
                {
                    Console.Write(grid[y, x] ? "#" : ".");
                }
                Console.WriteLine();
            }
        }

        private static void Day7()
        {
            Regex abba = new Regex(@"([a-z])(?!\1)([a-z])\2\1");
            var tls = from line in Resources.Day7.Split('\n')
                      let tokens = line.Split('[', ']')
                      where tokens.Where((o, i) => i % 2 == 0 && abba.IsMatch(o)).Any()
                      where !tokens.Where((o, i) => i % 2 == 1 && abba.IsMatch(o)).Any()
                      select line;

            Console.WriteLine(tls.Count());

            Regex abaParser = new Regex(@"([a-z])(?!\1)([a-z])\1");
            var ssl = from line in Resources.Day7.Split(new[] { '\n' }, StringSplitOptions.RemoveEmptyEntries)
                      let tokens = line.Split('[', ']')
                      let supernet = tokens.Where((o, i) => i % 2 == 0)
                      let aba = from net in supernet
                                from aba in ParseAba(net)
                                select aba
                      let hypernet = tokens.Where((o, i) => i % 2 == 1)
                      where aba.Any(bab => hypernet.Any(net => net.Contains(bab)))
                      select line;

            Console.WriteLine(ssl.Count());
        }

        private static IEnumerable<string> ParseAba(string net)
        {
            char a = net[0];
            char b = net[1];

            for (int i = 2; i < net.Length; i++)
            {
                char c = net[i];
                if (c == a)
                {
                    yield return $"{b}{a}{b}";
                }

                a = b;
                b = c;
            }
        }

        private static void Day6()
        {
            var frequent = from index in Enumerable.Range(0, 8)
                           from entry in Resources.Day6.Split('\n')
                           where entry.Length > 0
                           let c = entry[index]
                           group c by new { c, index } into g
                           group g by g.Key.index into g2
                           orderby g2.Key
                           from g in g2
                           where g.Count() == g2.Max(o => o.Count())
                           select g.Key;

            foreach (var f in frequent)
            {
                Console.WriteLine(f);
            }

            var infrequent = from index in Enumerable.Range(0, 8)
                             from entry in Resources.Day6.Split('\n')
                             where entry.Length > 0
                             let c = entry[index]
                             group c by new { c, index } into g
                             group g by g.Key.index into g2
                             orderby g2.Key
                             from g in g2
                             where g.Count() == g2.Min(o => o.Count())
                             select g.Key;

            foreach (var f in infrequent)
            {
                Console.WriteLine(f);
            }
        }

        private static void Day5()
        {
            string doorId = "reyedfim";

            MD5 algo = MD5.Create();
            var hacking = from index in Enumerable.Range(0, int.MaxValue)
                          let hashInput = Encoding.ASCII.GetBytes(doorId + index)
                          let hash = algo.ComputeHash(hashInput)
                          where hash[0] == 0 && hash[1] == 0 && (hash[2] & 0xf0) == 0
                          select new
                          {
                              SimpleDigit = hash[2].ToString("x")[0],
                              Position = hash[2],
                              Digit = (hash[3] >> 4).ToString("x")[0]
                          };

            Console.WriteLine("HACKING!!");
            int top = Console.CursorTop;
            int charCount = 0;
            StringBuilder secondPass = new StringBuilder("        ");

            foreach (var thing in hacking)
            {
                if (charCount < 8)
                {
                    Console.CursorTop = top;
                    Console.CursorLeft = charCount++;
                    Console.Write(thing.SimpleDigit);
                }

                if (thing.Position < 8 && secondPass[thing.Position] == ' ')
                {
                    secondPass[thing.Position] = thing.Digit;
                    Console.CursorTop = top + 1;
                    Console.CursorLeft = 0;
                    Console.WriteLine(secondPass.ToString());

                    if (secondPass.ToString().All(o => o != ' '))
                    {
                        break;
                    }
                }
            }

            Console.WriteLine();
        }

        private static void Day4()
        {
            var codes = from line in Resources.Day4.Split(new[] { '\n' }, StringSplitOptions.RemoveEmptyEntries)
                        let tokens = line.Split(new[] { '[', ']', '-' }, StringSplitOptions.RemoveEmptyEntries)
                        let phraseChars = from token in tokens.Take(tokens.Count() - 2)
                                          from c in token
                                          group c by c into g
                                          orderby g.Key
                                          select g
                        let sector = int.Parse(tokens.ElementAt(tokens.Count() - 2))
                        let checksum = tokens.Last()
                        let checksumcheck = new string((from charGroup in phraseChars
                                                        orderby charGroup.Count() descending, charGroup.Key
                                                        select charGroup.Key).Take(5).ToArray())
                        let decoded = from token in tokens.Take(tokens.Count() - 2)
                                      from c in token + '-'
                                      select (c != '-') ? (char)((c - 'a' + sector) % 26 + 'a') : c
                        where checksum == checksumcheck
                        select new
                        {
                            sector,
                            Decoded = new string(decoded.ToArray())
                        };

            Console.WriteLine("Sector sum: {0}", codes.Sum(o => o.sector));
            Console.WriteLine(codes.Single(o => o.Decoded.Contains("northpole")));
        }

        private static void Day3()
        {
            var foo = from line in Resources.Day3.Split('\n')
                      where line.Length > 0
                      let lengths = from length in line.Split(new[] { ' ' }, StringSplitOptions.RemoveEmptyEntries)
                                    select int.Parse(length)
                      let totalLength = lengths.Sum()
                      let maxLength = lengths.Max()
                      where totalLength - maxLength > maxLength
                      select lengths;

            Console.WriteLine("Valid trianges: {0}", foo.Count());

            int rank = 0;
            var foo2 = from line in Resources.Day3.Split('\n')
                       where line.Length > 0
                       from length in line.Split(new[] { ' ' }, StringSplitOptions.RemoveEmptyEntries)
                       let entry = new { Rank = rank++, Length = int.Parse(length) }
                       group entry.Length by new { Row = entry.Rank % 3, Col = entry.Rank / 9 } into tris
                       let totalLength = tris.Sum()
                       let maxLength = tris.Max()
                       where totalLength - maxLength > maxLength
                       select tris;

            Console.WriteLine("Valid trianges: {0}", foo2.Count());
        }

        private static void Day1()
        {
            Point lastPoint = Point.Zero;
            byte orientation = 0;

            var segments =
                (from instruction in Resources.Day1.Split(',').Select(o => o.Trim())
                 let o = (Direction)(((instruction[0] == 'L') ? ++orientation : --orientation) % 4)
                 let distance = int.Parse(instruction.Substring(1))
                 let allPoints = lastPoint.Interpolate(o, distance)
                 from point in allPoints
                 let LastPoint = lastPoint = point
                 select point).ToList();

            Console.WriteLine(Math.Abs(segments.Last().X) + Math.Abs(segments.Last().Y));

            for (int i = 0; i < segments.Count - 1; i++)
            {
                if (segments.Skip(i + 1).Contains(segments[i]))
                {
                    Console.WriteLine(Math.Abs(segments[i].X) + Math.Abs(segments[i].Y));
                    break;
                }
            }
        }

        private static void Day2()
        {
            char[,] panel = new char[5, 5];
            panel[1, 1] = '1';
            panel[1, 2] = '2';
            panel[1, 3] = '3';
            panel[2, 1] = '4';
            panel[2, 2] = '5';
            panel[2, 3] = '6';
            panel[3, 1] = '7';
            panel[3, 2] = '8';
            panel[3, 3] = '9';
            SolvePanel(panel);
            panel = new char[7, 7];
            panel[1, 3] = '1';
            panel[2, 2] = '2';
            panel[2, 3] = '3';
            panel[2, 4] = '4';
            panel[3, 1] = '5';
            panel[3, 2] = '6';
            panel[3, 3] = '7';
            panel[3, 4] = '8';
            panel[3, 5] = '9';
            panel[4, 2] = 'A';
            panel[4, 3] = 'B';
            panel[4, 4] = 'C';
            panel[5, 3] = 'D';
            SolvePanel(panel);
        }

        private static void SolvePanel(char[,] panel)
        {
            Func<char, Direction> parse = c =>
            {
                switch (c)
                {
                    case 'U':
                        return Direction.North;
                    case 'L':
                        return Direction.West;
                    case 'R':
                        return Direction.East;
                    case 'D':
                        return Direction.South;
                    default: throw new InvalidOperationException();
                }
            };

            var instructions = from line in Resources.Day2.Split('\n')
                               from c in line
                               let instruction = parse(c)
                               group instruction by line;

            int x = 0;
            int y = 0;

            for (y = 0; y < panel.GetLength(1); y++)
            {
                for (x = 0; x < panel.GetLength(0); x++)
                {
                    if (panel[y, x] == '5')
                    {
                        goto Done;
                    }
                }
            }

        Done:

            Console.WriteLine("Code: ");
            foreach (var set in instructions)
            {
                foreach (var i in set)
                {
                    switch (i)
                    {
                        case Direction.North:
                            if (panel[y - 1, x] != 0)
                            {
                                y--;
                            }
                            break;
                        case Direction.East:
                            if (panel[y, x + 1] != 0)
                            {
                                x++;
                            }
                            break;
                        case Direction.South:
                            if (panel[y + 1, x] != 0)
                            {
                                y++;
                            }
                            break;
                        case Direction.West:
                            if (panel[y, x - 1] != 0)
                            {
                                x--;
                            }
                            break;
                    }
                }

                Console.WriteLine($"{x}, {y} : {panel[y, x]}");
            }
        }
    }
}
