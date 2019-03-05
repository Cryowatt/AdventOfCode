module internal AdventOfCode.FSharp.Day1

let ParseInput input = Seq.map int input

let PartA (input:seq<string>) = 
    Seq.sum (ParseInput input)

let rec repeat items =
    seq {
        yield! items 
        yield! repeat items
        }
        
let addSet ((freq, dupe, state): (int * bool * Set<int>)) item =
    let currentFreq = freq + item
    match currentFreq with
    | i when state.Contains(i) -> (i, true, state)
    | i -> (i, false, state.Add i)

let PartB (input:seq<string>) =
    let input = ParseInput input
    let startState = (0, false, set [0])
    let gen = Seq.scan addSet startState (repeat input)
    let firstDupe = Seq.find (fun (_, dupe, _) -> dupe) gen
    match firstDupe with
    | (freq, _, _) -> freq
