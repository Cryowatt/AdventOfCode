using System;
using System.Collections.Concurrent;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Diagnostics;
using System.Linq;
using System.Linq.Expressions;
using System.Numerics;
using System.Reactive.Linq;
using System.Reactive.Subjects;
using System.Text;
using System.Text.RegularExpressions;
using System.Threading;
using System.Threading.Tasks;

namespace AdventOfCode2017
{
    public class Particle
    {
        public Particle(int id)
        {
            this.Position = default;
            this.Velocity = default;
            this.Acceleration = default;
            this.Id = id;
        }

        public Vector3 Position;
        public Vector3 Velocity;
        public Vector3 Acceleration;

        public int Id { get; }
    }

    public class Day20 : AdventDay<IEnumerable<Particle>>
    {
        private static Vector3 ParseValues(string input)
        {
            var values = input.Split(',');
            return new Vector3
            {
                X = int.Parse(values[0]),
                Y = int.Parse(values[1]),
                Z = int.Parse(values[2]),
            };
        }

        //p=<-3770,-455,1749>, v=<-4,-77,53>, a=<11,7,-9>
        protected override IEnumerable<Particle> TransformInput(string input) =>
            from particleInput in input.EnumerateLines().Select((line, i) => (line, i))
            let match = Regex.Match(particleInput.line, @"p=<(?<Position>-?\d+(,-?\d+){2})>, v=<(?<Velocity>-?\d+(,-?\d+){2})>, a=<(?<Acceleration>-?\d+(,-?\d+){2})>")
            let Position = ParseValues(match.Groups["Position"].Value)
            let Velocity = ParseValues(match.Groups["Velocity"].Value)
            let Acceleration = ParseValues(match.Groups["Acceleration"].Value)
            select new Particle(particleInput.i)
            {
                Position = Position,
                Velocity = Velocity,
                Acceleration = Acceleration
            };

        public override string RunPart1(IEnumerable<Particle> input) =>
            (from particle in input
             orderby particle.Acceleration.Length(), particle.Velocity.Length(), particle.Position.Length()
             select particle.Id).First().ToString();

        public override string RunPart2(IEnumerable<Particle> input)
        {
            int time = 0;
            var particles = input.ToArray();
            bool possibleCollision;

            do
            {
                possibleCollision = false;
                for (int i = 0; i < particles.Length - 1; i++)
                {
                    if (particles[i] == null)
                    {
                        continue;
                    }

                    bool hadCollision = false;

                    for (int j = i + 1; j < particles.Length; j++)
                    {
                        if (particles[j] == null)
                        {
                            continue;
                        }

                        // Can collide?
                        if (CanCollide(particles[i], particles[j]))
                        {
                            possibleCollision = true;
                        }

                        if (particles[j].Position == particles[i].Position)
                        {
                            Console.WriteLine("Destroyed " + particles[j].Id);
                            hadCollision = true;
                            particles[j] = null;
                        }
                    }

                    if (hadCollision)
                    {
                        Console.WriteLine("Destroyed " + particles[i].Id);
                        particles[i] = null;
                    }
                }

                particles.Where(o => o != null).ForEach(TimeStep);
                time++;
            } while (possibleCollision);

            return particles.Count(o => o != null).ToString();
        }

        private bool CanCollide(Particle particle1, Particle particle2)
        {
            var a = particle1.Acceleration;
            var b = particle2.Acceleration;
            var p = particle1.Position;
            var q = particle2.Position;
            var v = particle1.Velocity;
            var u = particle2.Velocity;

            // Corrected maths
            //t = -(sqrt((a - b - 2 u + 2 v)^2 - 8 (a - b) (p - q - u + v)) + a - b - 2 u + 2 v)/(2 (a - b)) and a!=b
            //t = (sqrt((a - b - 2 u + 2 v)^2 - 8 (a - b) (p - q - u + v)) - a + b + 2 u - 2 v)/(2 (a - b)) and a!=b
            //t = (p - q - u + v)/(u - v) and a = b and u!=v
            var u2 = 2 * u;
            var v2 = 2 * v;
            var squaredBit = a - b - u2 + v2;
            squaredBit *= squaredBit;
            var rootedBit = Vector3.SquareRoot(squaredBit - (8 * (a - b) * (p - q - u + v)));
            var t0 = -(rootedBit + a - b - u2 + v2) / (2 * (a - b));
            var t1 = (rootedBit - a + b + u2 - v2) / (2 * (a - b));
            var t2 = (p - q - u + v) / (u - v);

            float tx = Waffles(0f, a.X, b.X, v.X, u.X, p.X, q.X, t0.X, t1.X, t2.X);
            float ty = Waffles(0f, a.Y, b.Y, v.Y, u.Y, p.Y, q.Y, t0.Y, t1.Y, t2.Y);
            float tz = Waffles(0f, a.Z, b.Z, v.Z, u.Z, p.Z, q.Z, t0.Z, t1.Z, t2.Z);

            var tc = Math.Min(Math.Min(tx, ty), tz);

            if (!(tc >= 0))
            {
                return false;
            }

            if ((tx == float.PositiveInfinity || tx == tc) &&
                (ty == float.PositiveInfinity || ty == tc) &&
                (tz == float.PositiveInfinity || tz == tc))
            {
                return true;
            }
            else
            {
                return false;
            }
        }

        private static float Waffles(float t, float a, float b, float v, float u, float p, float q, float t0, float t1, float t2)
        {
            if (p == q)
            {
                return float.PositiveInfinity;
            }

            if (a == b && u != v && t2 % 1 == 0)
            {
                return t2;
            }

            if (a != b)
            {
                if (t0 > t && t0 % 1 == 0)
                {
                    if (t1 > t && t1 % 1 == 0)
                    {
                        return Math.Min(t0, t1);
                    }
                    else
                    {
                        return t0;
                    }
                }
                if (t1 > t && t1 % 1 == 0)
                {
                    return t1;
                }
            }

            return -1;
        }

        private void TimeStep(Particle particle)
        {
            particle.Velocity += particle.Acceleration;
            particle.Position += particle.Velocity;
        }
    }
}