def firstDigit (line: String) : Char :=
  let index := line.find (·.isDigit)
  line.get index

def lastDigit (line: String) : Char :=
  match (line.revFind (·.isDigit)) with
  | some index => line.get index
  | _ => 'A'

def partOne (input: String) :=
  let lines := (input.crlfToLf).split (·.isWhitespace)
  let firstDigits := lines.map (firstDigit ·)
  let lastDigits := lines.map (lastDigit ·)
  let zipped := List.zip firstDigits lastDigits
  (zipped.map (fun (a, b) => s!"{a}{b}".toNat!)).sum

def firstDigit2 (chars : List Char) : Char :=
  match chars with
  | [] => 'A'
  | c :: cs =>
      if c.isDigit then
        c
      else
        let str := String.mk (c :: cs)
        if str.startsWith "one"        then '1'
        else if str.startsWith "two"   then '2'
        else if str.startsWith "three" then '3'
        else if str.startsWith "four"  then '4'
        else if str.startsWith "five"  then '5'
        else if str.startsWith "six"   then '6'
        else if str.startsWith "seven" then '7'
        else if str.startsWith "eight" then '8'
        else if str.startsWith "nine"  then '9'
        else firstDigit2 cs


def lastDigit2 (chars : List Char) : Char :=
  let rec findLast (remainingChars : List Char) (currentBest : Char) : Char :=
    match remainingChars with
    | [] => currentBest
    | c :: cs =>
        let digitInTail := findLast cs currentBest

        if digitInTail != 'A' then
          digitInTail
        else
          if c.isDigit then
            c
          else
            let str := String.mk (c :: cs)
            if str.startsWith "one"   then '1'
            else if str.startsWith "two"   then '2'
            else if str.startsWith "three" then '3'
            else if str.startsWith "four"  then '4'
            else if str.startsWith "five"  then '5'
            else if str.startsWith "six"   then '6'
            else if str.startsWith "seven" then '7'
            else if str.startsWith "eight" then '8'
            else if str.startsWith "nine"  then '9'
            else currentBest
  findLast chars 'A'

def partTwo (input: String) :=
  let lines := (input.crlfToLf).split (·.isWhitespace)
  let firstDigits := lines.map (firstDigit2 ·.toList)
  let lastDigits := lines.map (lastDigit2 ·.toList)
  let zipped := List.zip firstDigits lastDigits
  (zipped.map (fun (a, b) => s!"{a}{b}".toNat!)).sum

def main : IO Unit := do
  let input ← (IO.FS.readFile "input.txt")
  IO.println (partOne input)
  IO.println (partTwo input)
