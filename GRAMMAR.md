# Lexical grammar
The lexical grammar is currently identical to [lox](https://craftinginterpreters.com/appendix-i.html).

```
    <number> ::= <digit>+ [ "." <digit>+ ]
    <string> ::= "\"" <any char except "\"">* "\""
<identifier> ::= <alpha> ( <alpha> | <digit> )*
     <alpha> ::= "a" ... "z" | "A" ... "Z" | "_"
     <digit> ::= "0" ... "9"
```

# Backus-Naur form primer
The above grammar is in [Backus-Naur form](https://en.wikipedia.org/wiki/Backus%E2%80%93Naur_form)
(specifically, Extended Backus-Naur form, or EBNF). This form is the typical 
way to specify how a programming language is lexed / parsed -- you'll find most 
languages will describe their grammar using this form. It looks and feels a bit
like regular expressions (though BNF grammars need not be regular). It consists
of a number of *symbol*s, and their definitions in terms of other symbols.

Anything enclosed in `""` is literal text -- this is referred to as a 
*terminal* symbol.

A symbol represented with angle-brackets like `<alpha>` is a referred to as a 
*non-terminal* symbol. This means that it can be expanded into some larger
expression.

The `|` operator means "or": the grammar will accept either the left or the 
right symbol.

The "extended" part of EBNF is the use of `*` and `+`. These are used much the 
same as they are for regular expressions:
- `<symbol>*`: 0 or more `<symbol>`s
- `<symbol>+`: 1 or more `<symbol>`s

Another common extension present in EBNF is the use of square brackets for 
optional items:
- `[<symbol>]`: 0 or 1 `<symbol>`

While the above grammar is very simple, these grammars can get pretty complex
fast -- luckily, the use of EBNF simplifies things (though, believe it or not, 
any grammar specifiable in EBNF can be specified in BNF -- it's just a lot 
harder to read and may require more recursive rules.)
