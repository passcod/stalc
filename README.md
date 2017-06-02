# Stalc

_Stack-based terminal calculator._

In the manner of [dc] but with advanced features and friendlier syntax.

- [ ] First-class support for dates/times and durations
- [ ] Unit and currency conversion
- [ ] Arbitrary-precision arithmetic
- [ ] Complex numbers
- [ ] Bases from 2 to 64
- [ ] Variables for simpler usage than registers
- [ ] History and lookup

Future features:

- [ ] Reverse-polish by default, but with Polish and Infix modes as options
- [ ] Multiple registers
- [ ] Persistent registers
- [ ] Optional [dc]-compatible terse syntax
- [ ] Linux, Mac, Windows, and more
- [ ] Unicode and non-Western number systems
- [ ] Simple equation solver

[dc]: https://en.wikipedia.org/wiki/dc_(computer_program)

## Install

```
$ cargo install stalc
```

### Upgrade

```
$ cargo install --force stalc
```

## Invocation

Default interactive mode:

```
$ stalc
```

Executing from a script, argument, pipe:

```
$ stalc -c script.stalc
$ stalc -e '123 45 + print'
$ echo '"x + 2 = 6" solve(x)' | stalc
```

Setting syntax mode:

```
$ stalc -s reverse # (default)
$ stalc -s infix
$ stalc -s polish
$ stalc -s terse
```

More options:

```
$ stalc --help
```

## Usage

Stalc is a whitespace-separated language, so most whitespace is not
significant and ignored, whether it be spaces, tabs, newlines, etc. Stalc is
stack-based, so inputs are pushed on top of a stack, and commands pop one or
more inputs, perform some computation and/or side-effects, and push zero or
more inputs back onto the stack.

### Inputs

Simple numbers:

```
123
123.4
.1234

-123
+123
```

Scientific and exponential notation:

```
1.23e4
87.38e6
.1927e2
```

Complex numbers:

```
12+3i
45-6i
+7i
-8i
9i
```

Dates and times:

```
12:34:56
2017-08-19
2004-09-23T04:18:21
2013-03-28T15:07:54+12:00

19:03:48.182

2017-W22
2017-W49-3
2017-152

15:37 // Assumed to be 00:15:37
37:15 // Assumed to be 00:37:15
99:23 // Only valid as a duration
```

Durations:

```
P5D
P3Y6M4DT12H30M5S
P0003-06-04T12:30:05

// Bare times are also considered durations:
4:13
38:23
99:42 // Resolves to 1:39:42

// Durations are much nicer to build using these commands:
12 days
827 milliseconds
6.5 years
2 hours 40 minutes +
```

Bases:

```
1010_2
774_8
1dea_16
1DEA_16
8a3X_64

1111_1 // Non-standard. Uses 1 as a tally mark.
```

Strings:

```
"A string."
"A
multiline
string."
"A string \"containing\" quotes."
"Another with \\ a slash."
```

Booleans:

```
true
false
```

Non-Western numerals (can be substituted in all forms above):

(Most of these are not likely to be implemented early on due to syntax and
semantics of RTL reading order and various features of numeral systems not
being defined yet.)

```
1409 // For reference
١٤٠٩
๑๔๐๙
၁၄၀၉
𝍡𝍤𝍠𝍩
௧௪௦௯
१४0९
一四〇九
חהטא
```

Roman numerals via commands:

```
"MMXI" roman // 2011
2009   roman // "MMIX"
```

Comments:

```
// Ignores everything until the next newline
```

For multiline comments, either add // to every line, or use multiline strings
and the `pop` command:

```
"Comment
text.

Spanning multiple
lines." pop
```

## Commands

Simple command words:

(The rules for what can be a symbol and what can't are not formally defined
yet, but as a general guideline: anything that can be interpreted as an
**input** cannot be a command, which includes all digits; anything else except
non-printable characters and whitespace can be.)

```
print
stack
pop
+
-
/
*
Σ
```

Commands have a fixed arity (number of values they pop off the stack):

```
123 print // valid
print // error (not enough values to pop)
123 456 print // valid, only prints 456

1 2 + // valid
1 + // error (not enough values to pop)
1 2 3 + // valid, only adds 2 and 3 together
```

Commands can call themselves, and can use that to simulate variable arity, e.g.
for the `sum` command.

```
1 2 3 4 5 6 7 8 9
9 sum
=> 45
```

The `sum` commands works like this: (pseudocode)

```
Arity: 2

// Pop arguments
N = first pop
Input = second pop

// Accumulator
internalval ||= 0
internalval += Input

if N < 2
  // Finish
  push(internalval)
else
  // Push back
  push(N - 1)
  sum
end
```

So step by step:

```
1 2 3                           Stack: [3 2 1]
3                               Stack: [3 3 2 1]

sum (pop arguments)             Stack: [2 1]       N: 3   Input: 3   Internalval: 0
sum (accumulator)               Stack: [2 1]       N: 3   Input: 3   Internalval: 3
sum (push back)                 Stack: [2 2 1]     N: 3   Input: 3   Internalval: 3

sum (pop arguments)             Stack: [2 2 1]     N: 2   Input: 2   Internalval: 3
sum (accumulator)               Stack: [1]         N: 2   Input: 2   Internalval: 5
sum (push back)                 Stack: [1 1]       N: 2   Input: 2   Internalval: 5

sum (pop arguments)             Stack: [1 1]       N: 1   Input: 1   Internalval: 5
sum (accumulator)               Stack: []          N: 1   Input: 1   Internalval: 6
sum (finish)                    Stack: [6]         N: 1   Input: 1   Internalval: 6
```

### Argument syntax

For convenience, if a command word is immediately (without whitespace
separator) followed by `([arguments])`, where `[arguments]` is a set of zero or
more values separated by whitespace, these arguments are passed to the command,
taking priority over the stack. The behaviour can be _thought of_ as pushing
the arguments onto the stack in reverse and then calling the command, but keep
in mind that the syntax does not _actually_ affect the stack directly (i.e. the
values are always passed directly to the command).

For example, the `sum` command can be called like this:

```
1 1 2 3 5 8 13
sum(7)
=> 34

// Or even:
1 2 sum(3 17)
=> 20
```

The first invocation makes it a bit easier to see that `7` is different from
the numbers to be summed, so it adds clarity. However, the second form confuses
the invocation and is much harder to understand. Thus, while there are many
ways to invoke commands using combinations of those two syntaxes, the
documentation will generally prefer the one which makes most sense.

Note that a command cannot be invoked with more arguments than it has arity,
but it can be invoked with less (and will then take the remainder from the
stack).

With this, `sum` can be reimplemented like thus, so that the N value is never
pushed back to the stack, and only the final result is:

```
N = first pop
Input = second pop

internalval ||= 0
internalval += Input

if N < 2
  push(internalval)
else
  sum(N - 1)
end
```

### Prefix (polish) and "infix" syntax modes

The way these work is by enabling different modes of delayed application to
commands. Given commands have a fixed arity, Stalc can take a subset of the
command's inputs from the stack, and then "wait" for more before applying the
command.

In polish mode, Stalc takes _none_ from the stack and then "waits" for more. In
infix mode, Stalc takes _one_ from the stack and then "waits". Argument syntax
still takes precedence.

Polish / Prefix mode:

```
                                Stack: []        Commands: (arity | inputs so far): []
10                              Stack: [10]      Commands: []
+                               Stack: [10]      Commands: [+ (2 | 0)]
40                              Stack: [10]      Commands: [+ (2 | 1)]
50   (before + applies)         Stack: [10]      Commands: [+ (2 | 2)]
     (after + applies)          Stack: [90 10]   Commands: []

// With argument syntax:

+(5)                            Stack: []        Commands: [+ (2 | 1)]
10   (before + applies)         Stack: []        Commands: [+ (2 | 2)]
     (after + applies)          Stack: [15]      Commands: []

// With argument syntax and a recursive function (not showing command stack/state):
// Remember that sum() has arity = 2.

sum(4)                          Stack: []      (sum() is waiting on a value)
10                              Stack: []      (sum(4 10) applies, calls sum(3), which waits)
20                              Stack: []      (sum(3 20) applies, calls sum(2), which waits)
30                              Stack: []      (sum(2 30) applies, calls sum(1), which waits)
40                              Stack: [100]   (sum(1 40) applies, pushes result to the stack)
```

"Infix" mode:

```
                                Stack: []        Commands: (arity | inputs so far): []
10                              Stack: [10]      Commands: []
+                               Stack: []        Commands: [+ (2 | 1)]
50   (before + applies)         Stack: []        Commands: [+ (2 | 2)]
     (after + applies)          Stack: [60]      Commands: []

// With argument syntax:

10                              Stack: [10]      Commands: []
+(5) (before + applies)         Stack: []        Commands: [+ (2 | 2)]
     (after + applies)          Stack: [15]      Commands: []

// With argument syntax and a recursive function (not showing command stack/state):
// Remember that sum() has arity = 2.

10                              Stack: [10]
sum(4)                          Stack: []      (sum(4 10) applies, calls sum(3), which waits)
20                              Stack: []      (sum(3 20) applies, calls sum(2), which waits)
30                              Stack: []      (sum(2 30) applies, calls sum(1), which waits)
40                              Stack: [100]   (sum(1 40) applies, pushes result to the stack)
```

In most documentation, the default Reverse Polish mode is used, unless explicitely specified.

### Variables

```
// Storing a value in a variable
3.14159 =:("pi")
-1 sqrt set("i")

// Retrieving the value of a variable
:>("i") print
7 ^(2) get("pi") *

// Some more tools:
isset("pi") //=> true
unset("pi")
isset("pi") //=> false
```

## Progress and plans

### Basics (0.0)

#### Spec progress

- Big picture:
  + [x] Separator: Unicode whitespace class
  + [ ] Commands: valid names
  + [x] Commands: arguments
  + [ ] Commands: definitions
  + [x] Modes: reverse
  + [x] Run: as interactive shell
  + [ ] Runtime: interpreter
- Inputs:
  + [x] Integers: natural
  + [x] Integers: signed
  + [x] Decimals: natural + signed
  + [x] Decimals: without leading zero
  + [x] Exponential notation
  + [x] Complex numbers
  + [x] Complex numbers: short form, signed
  + [x] Complex numbers: short form, unsigned
  + [x] Dates: YYYY-MM-DD
  + [x] Times: HH:MM:SS
  + [x] Times: MM:SS
  + [x] Times: MM:SS (with MM > 59)
  + [x] Times: HH:MM:SS (with HH > 23)
  + [ ] Times: signed
  + [x] Times: with decimal seconds
  + [x] Times: are also durations
  + [x] Datetimes: (any date format)T(any time format within time bounds)
  + [x] Datetimes: with numeric timezone
  + [x] ISO Durations: full form
  + [x] ISO Durations: short form
  + [ ] ISO Durations: signed
  + [x] ISO Durations: datetime form
  + [ ] ISO Durations: datetime form, signed
  + [x] Bases: NNN_B
  + [x] Bases: bases under 37 are case-insensitive
  + [x] Bases: bases 37 and over are case-sensitive
  + [x] Bases: maximum radix is 64
  + [x] Bases: unofficial radix-1 using `1` as the single digit
  + [x] Numerals: Western (0 1 2 3 4 5 6 7 8 9)
  + [x] Strings: on one line
  + [x] Strings: multiline
  + [x] Strings: quote escape
  + [x] Strings: backslash escape
  + [x] Comments: whole line
  + [x] Comments: partial line
  + [x] Comments: multiline (through strings)
  + [x] Booleans
- Commands:
  + [ ] Core (native impl)
  + [ ] Stdlib (written in Stalc)
  + [ ] Types
  + [x] Variables: global
- Core:
  + [x] `push<any> -> any` (does nothing when used on a stack directly)
  + [x] `pop<any> -> nil` (discards the value)
  + [x] `plus<num, num> -> num` (alias `+`)
  + [x] `minus<num, num> -> num` (alias `-`)
  + [x] `divise<num, num> -> num` (alias `/`)
  + [x] `multiply<num, num> -> num` (alias `*`, `×`)
  + [x] `power<num, num> -> num` (alias `^`)
  + [x] `sqrt<num, num> -> num` (alias `√`)
  + [ ] `concat<string, string> -> string`
  + [ ] `format<string, any> -> string`
  + [ ] `if<bool, any, any> -> any` (alias `?`)
  + [x] `print<any> -> nil`
  + [ ] `equal<any, any> -> bool` (alias `==`)
  + [ ] `not<bool> -> bool` (alias `!`)
  + [ ] `lessthan<num, num> -> bool` (alias `<`)
  + [ ] `and<bool, bool> -> bool` (alias `&&`)
  + [ ] `or<bool, bool> -> bool` (alias `||`)
  + [ ] `alias<string, string>`
  + [x] `set<any, string>` (alias `=:`)
  + [x] `get<string> -> any` (alias `:>`)
  + [x] `isset<string> -> bool`
  + [x] `unset<string>`
- Stdlib:
  + [x] `sum<num, num> -> num`
- Shell features:
  + [ ] Inspect: stack
  + [ ] Inspect: waiting commands
  + [ ] Inspect: trace
- Precision:
  + [ ] Set/get
- Platforms:
  + [x] First-tier (dev): Linux 64-bit

#### Implementation

- Parser:
  + [x] Integers: natural
  + [x] Integers: signed
  + [x] Decimals: natural + signed
  + [ ] Decimals: without leading zero
  + [ ] Exponential notation
  + [ ] Complex numbers
  + [ ] Complex numbers: short form, signed
  + [ ] Complex numbers: short form, unsigned
  + [ ] Dates: YYYY-MM-DD
  + [ ] Times: HH:MM:SS
  + [ ] Times: MM:SS
  + [ ] Times: MM:SS (with MM > 59)
  + [ ] Times: HH:MM:SS (with HH > 23)
  + [ ] Times: signed
  + [ ] Times: with decimal seconds
  + [ ] Times: are also durations
  + [ ] Datetimes: (any date format)T(any time format within time bounds)
  + [ ] Datetimes: with numeric timezone
  + [ ] ISO Durations: full form
  + [ ] ISO Durations: short form
  + [ ] ISO Durations: signed
  + [ ] ISO Durations: datetime form
  + [ ] ISO Durations: datetime form, signed
  + [ ] Bases: NNN_B
  + [ ] Bases: bases under 37 are case-insensitive
  + [ ] Bases: bases 37 and over are case-sensitive
  + [ ] Bases: maximum radix is 64
  + [ ] Bases: unofficial radix-1 using `1` as the single digit
  + [ ] Numerals: Western (0 1 2 3 4 5 6 7 8 9)
  + [ ] Strings: on one line
  + [ ] Strings: multiline
  + [ ] Strings: quote escape
  + [ ] Strings: backslash escape
  + [ ] Comments: whole line
  + [ ] Comments: partial line
  + [ ] Comments: multiline (through strings)
  + [ ] Booleans
  + [ ] Commands: bare
  + [ ] Commands: with arguments
  + [ ] Commands: with empty argument list
  + [ ] Commands: bare nested within arguments
  + [ ] Commands: argumented nested within arguments
  + [ ] Commands: test with ~5 levels of nesting

### Useful extras (0.1)

State: Predraft

- Big picture:
  + [x] Modes: polish
  + [x] Modes: infix
  + [ ] Modes: terse
  + [x] Run: from file
  + [x] Run: from line
  + [x] Run: from pipe
- Inputs:
  + [ ] Integers: comma as decimal separator
  + [x] Dates: YYYY-WNN-N
  + [x] Dates: YYYY-WNN
  + [x] Dates: YYYY-OOO
  + [ ] Times: seconds short form (:SS)
  + [ ] Times: seconds short form with decimals
  + [ ] Times: seconds short form with SS > 59
  + [ ] Bases: as part of composite inputs e.g. complex numbers, dates/times
  + [ ] Numerals: Roman (specced as only supported through string parsing)
  + [ ] Strings: Unicode escapes
- Commands:
  + [ ] Variables: command-internal
- Stdlib:
  + [x] `roman<num | string> -> num | string`
- Shell features:
  + [ ] History: whole line lookup

### Advanced Types (0.2)

State: Predraft

- Commands:
  + [ ] Types: enforced
  + [ ] Types: derived
  + [ ] Types: hierarchy
- Core:
  + [ ] `cast<any, string> -> any` (alias `::`)
  + [ ] `typeof<any> -> string`
- Stdlib:
  + [ ] `to_num`
  + [ ] `to_string`
  + [ ] `to_bool`
  + [ ] `to_date`
  + [ ] `to_time`

### Advanced History (0.3)

State: Predraft

- Shell features:
  + [ ] History: prefix lookup
  + [ ] History: partial lookup
  + [ ] History: persistent

### Advanced Numerals, part I (0.4)

State: Predraft

- Inputs:
  + [ ] Bases: with non-Western numerals
  + [ ] Numerals: Devanagari (0 १ २ ३ ४ ५ ६ ७ ८ ९)
  + [ ] Numerals: Burmese (၀ ၁ ၂ ၃ ၄ ၅ ၆ ၇ ၈ ၉)

### Advanced Bases (0.5)

State: Predraft

- Inputs:
  + [ ] Bases: higher than 64
  + [ ] Bases: negative
  + [ ] Bases: non-integer
  + [ ] Bases: complex

### Import/Export (0.6)

State: Predraft

- Big picture:
  + [ ] Ecosystem: import
  + [ ] Ecosystem: export

### Further along

- Big picture:
  + [ ] Modes: terse, dc-compatible
  + [ ] Multiple stacks (registers)
  + [ ] Persistent stacks
  + [ ] Runtime: JIT
  + [ ] Runtime: compiler ?
- Inputs:
  + [ ] Dates: YY-MM-DD ?
  + [ ] Dates: Non-ISO ?
  + [ ] Dates: ISO short form ?
  + [ ] Datetimes: with lettered timezone ?
  + [ ] Numerals: Arabic (٩ ٨ ٧ ٦ ٥ ٤ ٣ ٢ ١ ٠) (RTL)
  + [ ] Numerals: Hebrew (א ב ג ד ה ו ז ח ט) (RTL, non-zero, quasi-decimal)
  + [ ] Numerals: Tamil (௦ ௧ ௨ ௩ ௪ ௫ ௬ ௭ ௮ ௯) (quasi-decimal)
  + [ ] Numerals: Greek (ō α β γ δ ε ϝ ζ η θ ι) (quasi-decimal, possible conflicts with commands)
  + [ ] Numerals: Sino-Korean (〇 一 二 三 四 五 六 七 八 九 十) (quasi-decimal, alternate zero 零, RTL?)
  + [ ] Numerals: Others (Thai, Khmer, Abjad, ...)
- Commands:
  + [ ] Stdlib (optimised versions)
  + [ ] Variables: scoped
- Core:
  + [ ] I/O: files
  + [ ] I/O: stdin
  + [ ] I/O: net
- Stdlib:
  + [ ] convert
  + [ ] solve
- Precision:
  + [ ] Auto
- Platforms:
  + [ ] Second-tier (CI): OS X, Windows 64-bit
  + [ ] Third-tier: BSD? ARM? 32-bit?
  + [ ] Fourth-tier: untested

