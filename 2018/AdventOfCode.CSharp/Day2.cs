using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;

namespace AdventOfCode.CSharp
{
    public class Day2 : IDay
    {
        public string RunPartA(IEnumerable<string> input)
        {
            int doubles = 0;
            int triples = 0;

            foreach (var id in input)
            {
                int[] letterCount = new int[26];
                for (int i = 0; i < id.Length; i++)
                {
                    int alphaIndex = id[i] - 'a';
                    letterCount[alphaIndex]++;
                }

                bool hasDouble = false;
                bool hasTriple = false;

                for (int i = 0; i < letterCount.Length; i++)
                {
                    if (letterCount[i] == 2)
                    {
                        hasDouble = true;
                    }
                    else if (letterCount[i] == 3)
                    {
                        hasTriple = true;
                    }
                }
                if (hasDouble)
                {
                    doubles++;
                }
                if (hasTriple)
                {
                    triples++;
                }
            }

            return (doubles * triples).ToString();
        }

        public string RunPartB(IEnumerable<string> input)
        {
            var boxes = input.ToArray();
            var answerBuilder = new StringBuilder();
            for (int i = 0; i < boxes.Length - 1; i++)
            {
                for (int j = i + 1; j < boxes.Length; j++)
                {
                    var leftBox = boxes[i];
                    var rightBox = boxes[j];
                    answerBuilder.Clear();

                    int charDifference = 0;

                    for (int c = 0; c < leftBox.Length; c++)
                    {
                        if (leftBox[c] != rightBox[c])
                        {
                            charDifference++;
                        }
                        else
                        {
                            answerBuilder.Append(leftBox[c]);
                        }

                        if (charDifference > 1)
                        {
                            break;
                        }
                    }

                    if (charDifference == 1)
                    {
                        return answerBuilder.ToString();
                    }
                }
            }

            throw new InvalidOperationException();
        }
    }
}
