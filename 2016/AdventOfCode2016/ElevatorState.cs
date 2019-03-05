using System;
using System.Collections;
using System.Collections.Generic;
using System.Collections.Specialized;
using System.Diagnostics;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace AdventOfCode
{
    public enum Elements
    {
        Sr, //strontium
        SrGen, //strontium
        Ru, //ruthenium 
        RuGen, //ruthenium
        Pm, //promethium
        PmGen, //promethium
        Tm, //thulium
        TmGen, //thulium
        Pu, //plutonium
        PuGen, //plutonium
        El,
        ElGen,
        Li,
        LiGen,
        Human
    }

    public struct ElevatorState : IEquatable<ElevatorState>
    {
        private const int ElementCount = 7;
        private const int TopFloor = 3;
        private const int GeneratorOffset = 1;
        private static BitVector32.Section[] sections;
        public static ICollection<Elements> AllKeys;
        private int data;
        private static int DoneState;
        private static int[] FloorCheck = new int[4];

        static ElevatorState()
        {
            AllKeys = (Elements[])Enum.GetValues(typeof(Elements));
            int itemCount = AllKeys.Count;
            sections = new BitVector32.Section[itemCount];
            BitVector32.Section last;
            sections[0] = last = BitVector32.CreateSection(TopFloor);
            var doneState = new BitVector32();

            for (int i = 1; i < itemCount; i++)
            {
                sections[i] = last = BitVector32.CreateSection(TopFloor, last);
            }

            for (int i = 0; i < ElementCount * 2; i++)
            {
                doneState[sections[i]] = TopFloor;
            }

            doneState[sections[(int)Elements.Human]] = TopFloor;
            DoneState = doneState.Data;

            for (int floor = 0; floor < 4; floor++)
            {
                var floorState = new BitVector32();

                for (int i = 0; i < ElementCount; i++)
                {
                    floorState[sections[(i << 1) + GeneratorOffset]] = floor;
                }

                FloorCheck[floor] = floorState.Data;
            }
        }

        public ElevatorState(int data)
        {
            this.data = data;
        }

        public int this[Elements key] {
            get {
                return new BitVector32(this.data)[sections[(int)key]];
            }

            set {
                if (key != Elements.Human && (int)key >= ElementCount * 2)
                {
                    return;
                }

                var state = new BitVector32(this.data);
                state[sections[(int)key]] = value;
                this.data = state.Data;
            }
        }

        public bool IsDone()
        {
            return data == DoneState;
        }

        public ICollection<Elements> Keys {
            get { return AllKeys; }
        }

        public int GetHeuristic()
        {
            var state = new BitVector32(this.data);
            int score = 0;
            for (int i = 0; i < ElementCount * 2; i++)
            {
                score += state[sections[i]];
            }

            return score / 2;
        }

        public bool IsValidFloor(int floor)
        {
            var state = new BitVector32(this.data);
            bool hasGenerator = false;
            bool hasMismatch = false;
            
            for (int i = 0; i < ElementCount; i++)
            {
                var item = i << 1;
                var gen = item + GeneratorOffset;
                var genState = state[sections[gen]];

                if (genState == floor)
                {
                    hasGenerator = true;
                }

                if (state[sections[item]] == floor && genState != floor)
                {
                    hasMismatch = true;
                }

                if(hasGenerator && hasMismatch)
                {
                    return false;
                }
            }

            return !hasGenerator || !hasMismatch;

            //if (!hasGenerator)
            //{
            //    // No generators, we cool
            //    return true;
            //}
            //w = Stopwatch.StartNew();
            //for (int i = 0; i < ElementCount; i++)
            //{
            //    var item = i << 1;
            //    if (state[sections[item]] == floor && state[sections[item + GeneratorOffset]] != floor)
            //    {
            //        return false;
            //    }
            //}
            //w.Stop();

            //return true;
        }

        public IEnumerable<ElevatorState> EnumerateMoves(int fromFloor, int newFloor)
        {
            var state = new BitVector32(this.data);
            for (int i = 0; i < ElementCount * 2; i++)
            {
                if (state[sections[i]] != fromFloor)
                {
                    continue;
                }

                for (int j = i + 1; j < ElementCount * 2; j++)
                {
                    if (state[sections[j]] != fromFloor)
                    {
                        continue;
                    }

                    var newState = state;
                    newState[sections[i]] = newFloor;
                    newState[sections[j]] = newFloor;
                    newState[sections[(int)Elements.Human]] = newFloor;
                    var newState2 = new ElevatorState(newState.Data);

                    if (IsValidMove(newState2, fromFloor, newFloor))
                    {
                        yield return newState2;
                    }
                }

                {
                    var newState = state;
                    newState[sections[i]] = newFloor;
                    newState[sections[(int)Elements.Human]] = newFloor;
                    var newState2 = new ElevatorState(newState.Data);

                    if (IsValidMove(newState2, fromFloor, newFloor))
                    {
                        yield return newState2;
                    }
                }
            }
        }
        private static bool IsValidMove(ElevatorState state, int fromFloor, int toFloor)
        {
            return (state.IsValidFloor(fromFloor) && state.IsValidFloor(toFloor));
        }

        public override bool Equals(object obj)
        {
            if (object.ReferenceEquals(this, obj))
            {
                return true;
            }

            if (obj == null || !(obj is ElevatorState))
            {
                return false;
            }

            ElevatorState other = (ElevatorState)obj;
            return this.data == other.data;
        }

        public override int GetHashCode()
        {
            return this.data.GetHashCode();
        }

        public bool Equals(ElevatorState other)
        {
            return other.data.Equals(this.data);
        }
    }
}
