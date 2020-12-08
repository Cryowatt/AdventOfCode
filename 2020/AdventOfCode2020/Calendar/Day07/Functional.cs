using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text.RegularExpressions;

namespace AdventOfCode.Calendar.Day07
{
    public record Bag(string Modifier, string Colour)
    {
        public string Fullname => $"{this.Modifier} {this.Colour}";
        public IList<ChildBag> Children { get; set; } = new List<ChildBag>();
    }

    public record ChildBag(Bag Bag, int Count);

    public class Functional : SolutionBase<IReadOnlyDictionary<string, Bag>>
    {
        private static IReadOnlyDictionary<string, Bag> Parser(string input)
        {
            var bagPattern = new Regex(@"(?:(?<Count>\d+) |^)(?<Fullname>(?<Modifier>\w+) (?<Colour>\w+)) bags?(?:,|.)");
            var dictionary = new Dictionary<string, Bag>();

            using (var reader = new StringReader(input))
            {
                string line;
                while ((line = reader.ReadLine()) != null)
                {
                    var segments = line.Split(" contain ");
                    var parentSegment = segments[0];
                    var childrenSegment = segments[1];

                    var parentMatch = bagPattern.Match(parentSegment);
                    Bag parent;

                    if (!dictionary.TryGetValue(parentMatch.Groups["Fullname"].Value, out parent))
                    {
                        parent = new Bag(parentMatch.Groups["Modifier"].Value, parentMatch.Groups["Colour"].Value);
                        dictionary.Add(parent.Fullname, parent);
                    }

                    if (childrenSegment != "no other bags.")
                    {
                        var childrenMatches = bagPattern.Matches(childrenSegment);

                        foreach (var match in childrenMatches.OfType<Match>())
                        {
                            Bag bag;

                            if (!dictionary.TryGetValue(match.Groups["Fullname"].Value, out bag))
                            {
                                bag = new Bag(match.Groups["Modifier"].Value, match.Groups["Colour"].Value);
                                dictionary.Add(bag.Fullname, bag);
                            }

                            var child = new ChildBag(bag, int.Parse(match.Groups["Count"].Value));

                            parent.Children.Add(child);
                        }
                    }
                }
            }

            return dictionary;
        }

        public Functional() : base(Parser) { }

        public override object PartA()
        {
            Dictionary<Bag, bool> cache = new Dictionary<Bag, bool>();

            bool AnyChild(Bag bag, string fullname)
            {
                if (cache.TryGetValue(bag, out bool hasChild))
                {
                    return hasChild;
                }

                foreach (var child in bag.Children)
                {
                    if (child.Bag.Fullname == fullname)
                    {
                        hasChild = true;
                        break;
                    }
                    else if(AnyChild(child.Bag, fullname))
                    {
                        hasChild = true;
                        break;
                    }
                }

                cache[bag] = hasChild;
                return hasChild;
            }

            return this.input.Values.Count(bag => AnyChild(bag, "shiny gold"));
        }

        public override object PartB()
        {
            Dictionary<Bag, int> cache = new Dictionary<Bag, int>();

            int CountDescendants(Bag bag)
            {
                if (cache.TryGetValue(bag, out int descendantCount))
                {
                    return descendantCount;
                }

                descendantCount = bag.Children.Sum(child => child.Count * CountDescendants(child.Bag)) + 1;
                cache[bag] = descendantCount;
                return descendantCount;
            }

            return CountDescendants(this.input["shiny gold"]) - 1; // -1 to skip counting itself
        }
    }
}
