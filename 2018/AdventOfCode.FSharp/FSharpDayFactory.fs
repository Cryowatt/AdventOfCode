namespace AdventOfCode.FSharp

open AdventOfCode

type DayFixture(partA, partB) =
    interface IDay with
        member this.RunPartA(input:seq<string>):string = string (partA input)
        member this.RunPartB(input:seq<string>):string = string (partB input)

type public FSharpDayFactory() =
    interface IDayFactory with
        member this.GetDay(dayNumber:int):IDay =
            match dayNumber with
                | 1 -> DayFixture(Day1.PartA, Day1.PartB) :> IDay
                | _ -> null