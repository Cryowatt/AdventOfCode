using System;
using System.Collections.Concurrent;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Text;
using System.Text.RegularExpressions;
using System.Threading;
using System.Threading.Tasks;

namespace AdventOfCode
{
    public struct RobotInstruction
    {
        ////bot 12 gives low to output 4 and high to bot 125
        private static readonly Regex parser =
            new Regex(@"bot (?<RobotId>\d+) gives low to (?<LowType>bot|output) (?<LowTarget>\d+) and high to (?<HighType>bot|output) (?<HighTarget>\d+)");
        public bool LowTargetIsOutput;
        public int LowTarget;
        public bool HighTargetIsOutput;
        public int HighTarget;
        public int RobotId;

        internal static bool TryParse(string input, out RobotInstruction instruction)
        {
            Match match = parser.Match(input);
            if (!match.Success)
            {
                instruction = new RobotInstruction();
                return false;
            }

            instruction = new RobotInstruction
            {
                RobotId = int.Parse(match.Groups["RobotId"].Value),
                LowTarget = int.Parse(match.Groups["LowTarget"].Value),
                LowTargetIsOutput = match.Groups["LowType"].Value == "output",
                HighTarget = int.Parse(match.Groups["HighTarget"].Value),
                HighTargetIsOutput = match.Groups["HighType"].Value == "output"
            };

            return true;
        }
    }

    public class Robot
    {
        public ConcurrentQueue<RobotInstruction> InstructionQueue = new ConcurrentQueue<RobotInstruction>();
        public int RobotId;
        private IDictionary<int, Robot> otherRobots;
        private ConcurrentQueue<int> heldMicrochips = new ConcurrentQueue<int>();
        private SemaphoreSlim itemEvent = new SemaphoreSlim(0, 1);
        public Task InstructionTaskQueue;
        private IDictionary<int, Robot> outputs;
        internal static long RunningBots;

        public Robot(int robotId, IDictionary<int, Robot> otherRobots, IDictionary<int, Robot> outputs)
        {
            this.RobotId = robotId;
            this.otherRobots = otherRobots;
            this.outputs = outputs;
        }

        public void GiveValue(int value)
        {
            heldMicrochips.Enqueue(value);

            if (heldMicrochips.Count == 2)
            {
                itemEvent.Release();
            }
        }

        public void QueueInstruction(RobotInstruction instruction)
        {
            // Console.WriteLine("{0} queued instruction", this.RobotId);
            if (this.InstructionTaskQueue == null)
            {
                this.InstructionTaskQueue = Run(instruction);
            }
            else
            {
                this.InstructionTaskQueue = this.InstructionTaskQueue.ContinueWith(_ => Run(instruction));
            }
        }

        public async Task Run(RobotInstruction instruction)
        {
            Debug.Assert(this.RobotId == instruction.RobotId);
            await itemEvent.WaitAsync();
            Interlocked.Increment(ref RunningBots);
            SortedSet<int> items = new SortedSet<int>();

            for (int i = 0; i < 2; i++)
            {
                int item;
                bool hasItem = this.heldMicrochips.TryDequeue(out item);
                Debug.Assert(hasItem);
                items.Add(item);
            }

            Debug.Assert(items.Count == 2);

            int low = items.First();
            int high = items.Last();

            TransferItem(low, instruction.LowTarget, instruction.LowTargetIsOutput);
            TransferItem(high, instruction.HighTarget, instruction.HighTargetIsOutput);
            Interlocked.Decrement(ref RunningBots);
        }

        private void TransferItem(int item, int target, bool targetIsOutput)
        {
            //Console.WriteLine("{0} gives {1} to {2} {3}", this.RobotId, item, targetIsOutput ? "output" : "bot", target);
            Robot output = (targetIsOutput) ? this.outputs[target] : this.otherRobots[target];
            output.GiveValue(item);
        }
    }
}
