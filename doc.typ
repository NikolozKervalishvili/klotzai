#align(center)[#smallcaps("Klotzai Docs") #line(length: 100%)]

#set heading(numbering: "1.1.")
#let code(s) = {
  h(0.5em)
  box(outset: 3pt, stroke: black + 0.25pt, fill: gray.lighten(75%))[#text(
    blue.darken(40%),
    size: 10pt,
  )[#s]]
}

= Lexer
== Keywords
+ Function declaration: #code("def")
+ Variable binding: #code("let")
+ Control flow: #code("if") #code("else") #code("while") #code("return")
+ Boolean/Null literals: #code("true") #code("false") #code("nil")

== Identifiers
Must start with an alphabetical character or underscore, followed by alphanumeric characters or underscores.

== Literals
The raw data values hardcoded into the scripts.
+ Integer: Whole number e.g. #code("42")
+ Float: Decimal number e.g. #code("3.14")
+ String: Text enclosed in double quotes. We should support basic escape sequences (like newline or tab).

== Operators
+ Arithmetic: #code("+") #code("-") #code("*") #code("/") #code("%")
+ Comparison: #code("==") #code("!=") #code("<") #code(">") #code("<=") #code(">=")
+ Logical: #code("&&") #code("||") #code("!")
+ Assignment: #code("=")

== Punctuation / Delimiters
+ Grouping/Calls: #code("(") #code(")")
+ Block Scoping: #code("{") #code("}")
+ Seperator: #code(",")
+ End of statement: #code(";")

== Meta Tokens
+ EOF
+ Illegal/Error: A token used gracefully to catch and report unknown character without instantly crashing lexer loop
