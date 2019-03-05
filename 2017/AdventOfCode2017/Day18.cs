using System;
using System.Collections.Concurrent;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Linq;
using System.Linq.Expressions;
using System.Numerics;
using System.Reactive.Linq;
using System.Reactive.Subjects;
using System.Threading;
using System.Threading.Tasks;

namespace AdventOfCode2017
{
    public class Day18 : AdventDay<IEnumerable<(string Op, string A, string B)>>
    {
        private class Context
        {
            public int WaitingCount = 0;
        }

        protected override IEnumerable<(string Op, string A, string B)> TransformInput(string input) =>
            from line in input.EnumerateLines()
            let tokens = line.Split(' ')
            let op = tokens[0]
            let a = tokens[1]
            let b = tokens.Length > 2 ? tokens[2] : "0"
            select (op, a, b);

        public override string RunPart1(IEnumerable<(string Op, string A, string B)> input)
        {
            var channel = new ConcurrentQueue<long>();
            var program = new Program(0, input.ToArray(), new Context(), channel, channel, shouldReturnOnRcv: true);
            program.Run();
            return channel.Last().ToString();
        }

        public override string RunPart2(IEnumerable<(string Op, string A, string B)> input)
        {
            var instructions = input.ToArray();
            var channelA = new ConcurrentQueue<long>();
            var channelB = new ConcurrentQueue<long>();
            var context = new Context();
            var pa = new Program(0, instructions, context, channelA, channelB);
            var pb = new Program(1, instructions, context, channelB, channelA);

            Parallel.Invoke(pa.Run, pb.Run);

            return pb.SendCount.ToString();
        }

        private class Program
        {
            private Dictionary<string, long> registers = new Dictionary<string, long>();
            private (string Op, string A, string B)[] instructions;
            private ConcurrentQueue<long> inQueue;
            private ConcurrentQueue<long> outQueue;
            private Context context;
            private int progId;
            public int SendCount { get; private set; }
            private bool shouldReturnOnRcv;

            public Program(
                int progId,
                (string Op, string A, string B)[] instructions,
                Context context,
                ConcurrentQueue<long> outQueue,
                ConcurrentQueue<long> inQueue,
                bool shouldReturnOnRcv = false)
            {
                this.context = context;
                this.progId = progId;
                this.instructions = instructions;
                this.inQueue = inQueue;
                this.outQueue = outQueue;
                this.registers["p"] = progId;
                this.shouldReturnOnRcv = shouldReturnOnRcv;
            }

            public void Run()
            {
                for (int ip = 0; ip < this.instructions.Length;)
                {
                    int whichInstruction = ip;
                    var (op, regA, regB) = this.instructions[ip];
                    //Console.WriteLine($"#{progId}:{whichInstruction}:{op} {regA} {regB} => {GetReferenceOrValue(regA)}, {GetReferenceOrValue(regB)} ");

                    switch (op)
                    {
                        case "snd":
                            var sendValue = GetReferenceOrValue(regA);
                            this.outQueue.Enqueue(sendValue);
                            SendCount++;
                            //Console.WriteLine($"{progId}:{sendCount} {sendValue} =>");
                            //Console.Beep((int)((freq) % (32767 - 37)) + 37, 50); // uncomment for mad beats
                            break;
                        case "set":
                            this.registers[regA] = GetReferenceOrValue(regB);
                            break;
                        case "add":
                            this.registers[regA] = GetReferenceOrValue(regA) + GetReferenceOrValue(regB);
                            break;
                        case "mul":
                            this.registers[regA] = GetReferenceOrValue(regA) * GetReferenceOrValue(regB);
                            break;
                        case "mod":
                            this.registers[regA] = GetReferenceOrValue(regA) % GetReferenceOrValue(regB);
                            break;
                        case "rcv":
                            if (shouldReturnOnRcv)
                            {
                                return;
                            }

                            Interlocked.Increment(ref context.WaitingCount);
                            SpinWait.SpinUntil(() =>
                                inQueue.Count > 0 || (context.WaitingCount > 1 && inQueue.Count + outQueue.Count == 0));

                            if (context.WaitingCount > 1 && inQueue.Count + outQueue.Count == 0)
                            {
                                //Console.WriteLine($"#{progId}:{Thread.CurrentThread.ManagedThreadId} DEADLOCK!");
                                return;
                            }

                            Interlocked.Decrement(ref context.WaitingCount);
                            long recieveValue;
                            while (!inQueue.TryDequeue(out recieveValue))
                            {
                                //throw new InvalidOperationException();
                            }

                            //Console.WriteLine($"{progId}: {recieveValue} <=");
                            this.registers[regA] = recieveValue;
                            break;
                        case "jgz":
                            if (GetReferenceOrValue(regA) > 0)
                            {
                                ip += (int)GetReferenceOrValue(regB);
                                continue;
                            }
                            break;
                        default:
                            throw new InvalidOperationException();
                    }

                    ip++;
                }

                Interlocked.Increment(ref context.WaitingCount);
            }

            private long GetReferenceOrValue(string token)
            {
                if (int.TryParse(token, out int literal))
                {
                    return literal;
                }
                else
                {
                    if (registers.TryGetValue(token, out long register))
                    {
                        return register;
                    }
                    else
                    {
                        registers.Add(token, 0);
                        return 0;
                    }
                }
            }
        }
    }
}