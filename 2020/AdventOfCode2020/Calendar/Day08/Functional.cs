using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text.RegularExpressions;

namespace AdventOfCode.Calendar.Day08
{
    public enum Opcode
    {
        Acc, Jmp, Nop, Visited
    }

    public struct Instruction
    {
        public Opcode Opcode;
        public int Operand;
    }

    public class Functional : SolutionBase<Instruction[]>
    {
        public static Instruction Parser(string input)
        {
            return new Instruction
            {
                Opcode = Enum.Parse<Opcode>(input.Substring(0, 3), true),
                Operand = int.Parse(input.Substring(4))
            };
        }

        public Functional() : base(Parsers.Lines(Parser).ToArray()) { }

        public override object PartA()
        {
            return RunProgram((Instruction[])this.input.Clone()).A;
        }

        private (int A, int PC) RunProgram(Instruction[] memory)
        {
            int pc = 0;
            int a = 0;
            Instruction op;

            while (pc < memory.Length && (op = memory[pc]).Opcode != Opcode.Visited)
            {
                switch (op.Opcode)
                {
                    case Opcode.Acc:
                        a += op.Operand;
                        memory[pc].Opcode = Opcode.Visited;
                        pc++;
                        break;
                    case Opcode.Jmp:
                        memory[pc].Opcode = Opcode.Visited;
                        pc += op.Operand;
                        break;
                    case Opcode.Nop:
                        memory[pc].Opcode = Opcode.Visited;
                        pc++;
                        break;
                }
            }

            return (a, pc);
        }

        public override object PartB()
        {
            for (int i = 0; i < this.input.Length; i++)
            {
                var memory = (Instruction[])this.input.Clone();
                var op = memory[i].Opcode;

                if (op == Opcode.Nop)
                {
                    memory[i].Opcode = Opcode.Jmp;
                }
                else if (op == Opcode.Jmp)
                {
                    memory[i].Opcode = Opcode.Nop;
                }
                else
                {
                    continue;
                }

                var result = RunProgram(memory);
                if (memory.Length == result.PC)
                {
                    return result.A;
                }
            }

            throw new NotImplementedException();
        }
    }
}
