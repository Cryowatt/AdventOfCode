using AdventOfCode;
using System;
using System.Reflection;

namespace AdventOfCode2020.CSharp
{
    public class CSharpDayFactory : IDayFactory
    {
        public IDay GetDay(int dayNumber)
        {
            var type = Assembly.GetAssembly(this.GetType()).GetType("AdventOfCode.CSharp.Day" + dayNumber, false, true);

            if (type == null)
            {
                return null;
            }

            return (IDay)Activator.CreateInstance(type);
        }
    }
}
