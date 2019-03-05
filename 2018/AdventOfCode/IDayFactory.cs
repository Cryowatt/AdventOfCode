using System;
using System.Reflection;

namespace AdventOfCode
{
    public interface IDayFactory
    {
        IDay GetDay(int dayNumber);
    }
}
