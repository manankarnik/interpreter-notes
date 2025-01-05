#set heading(numbering: "1.")
#import "@preview/cetz:0.3.1": canvas, draw

= Definition
An interpreter is a computer program that directly executes instructions written in a programming or scripting language, without requiring them previously to have been compiled into a machine language program.

= Architecture
Source code is tansformed into tokens using a lexer and AST using a parser.

#canvas({
    import draw: *
    let elements = ((0, "Source Code"), (5, "Tokens"), (4, "AST"));
    for (d, name) in elements {
        content(
            (rel: (d, 0)),
            name: name,
            frame: "rect",
            anchor: "west",
            padding: 1em,
            [#name]
        )
    }
    set-style(mark: (end: (symbol: ">", fill: black)))
    line("Source Code", "Tokens", name: "l1")
    content(
        ("l1.start", 50%, "l1.end"),
        angle: "l1.end",
        padding: 0.4,
        anchor: "north",
        [Lexer]
    )
    line("Tokens", "AST", name: "l2")
    content(
        ("l2.start", 50%, "l2.end"),
        angle: "l2.end",
        padding: 0.4,
        anchor: "north",
        [Parser]
    )
})

== Source Code
Plain text program written in a specific programming language

== Tokens
#canvas({
    import draw: *
    let tokens = ("let", "x", "=", "5", "+", "5", ";");
    for token in tokens {
        content(
            (rel: (1.5, 0)),
            frame: "rect",
            padding: 1em,
            [#token],
        )
    }
})

- _Identifiers_: Identify variables, functions, classes, or other entities in code $=>$ `x, y, z`
- _Literals_: Fixed values that represent data directly $=>$ ```c 5, "Hello", false```
- _Keywords_: Reserved words in a that have special meanings $=>$ ```rust if, let, for```

==  AST
Data structure used to represent the structure of a program or code snippet.

#canvas({
    import draw: *
    content(
        (1, 3),
        frame: "rect",
        padding: 1em,
        name: "x",
        [x],
    )
    content(
        (1, 1.5),
        frame: "circle",
        padding: .5em,
        name: "+",
        [+],
    )
    content(
        (0, 0),
        frame: "rect",
        padding: 1em,
        name: "five_one",
        [5],
    )
    content(
        (2, 0),
        frame: "rect",
        padding: 1em,
        name: "five_two",
        [5],
    )
    set-style(mark: (end: (symbol: ">", fill: black)))
    line("x", "+")
    line("+", "five_one")
    line("+", "five_two")
})

= Glossary
/ AST: Abstract Syntax Tree

