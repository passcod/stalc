# Stalc

_Stack-based CLI calculator._

In the manner of [dc] but with advanced features and friendlier syntax.

- [ ] First-class support for dates/times and durations
- [ ] Unicode and non-Western number systems
- [ ] Unit and currency conversion
- [ ] Arbitrary-precision arithmetic
- [ ] Complex numbers
- [ ] Bases from 2 to 64
- [ ] History and lookup
- [ ] Simple equation solver
- [ ] Multiple registers
- [ ] Persistent registers
- [ ] Variables for simpler usage than registers
- [ ] Reverse-polish by default, but with Polish and Infix modes as options
- [ ] Linux, Mac, Windows, and more
- [ ] Optional [dc]-compatible terse syntax

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
significant and ignored, whether it be spaces, tabs, newlines, etc.

Stalc is stack-based, so inputs are pushed on top of a stack, and commands pop
one or more inputs, perform some computation and/or side-effects, and push zero
or more inputs back onto the stack.

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

Non-Western numerals (can be substituted in all forms above):

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

(The rules for what can be a symbol and what can't are TBC, but Unicode is
supported.)

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
