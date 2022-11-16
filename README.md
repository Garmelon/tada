# Tada

Tada is an interpreted language inspired by Lua.

It started with the idea of making Lua more consistent. For example, why not
return multiple values by returning and then destructuring a table? That would
also allow accessing return values besides the first in expressions. Since we
need destructuring anyways, why not use a table to pass arguments? Positional
and keyword arguments would directly fall out of that approach.

After some discussion, this turned into using tables as answer for pretty much
every design decision. Functions? Tables. Code blocks? Tables, their curly
braces fit pretty well already. Scopes? Tables. Function calls? Tables. Source
code? Tables, with most "normal" syntax being simple syntactic sugar.

## Syntax

At its core, a program consists only of literals. Any further syntax desugars to
literals. A program file contains a single literal, usually a table literal, at
its root.

The types of literal are:
- Boolean: `true`, `false`
- Integer: `1`, `-3`, `0x45`, `0b11001`
- String: `"Hello world\n"`
- Builtin: `'get`, `'scope`
- Table: `'{ 1, 2, 3 }`, `'{ foo: bar }`

A program's source code can be represented entirely with those literals and
nothing else. However, that becomes cumbersome quickly. It doesn't look like an
imperative language either. For that reason there is a lot of syntactic sugar.

## Syntactic sugar

Builtins, function calls and similar operations are magic. On the other hand,
syntactic sugar is the opposite of magic: It is transparent even for those not
familiar with the implementation of the language. When it is applied, the result
is just code again.

One design principle of this language is to limit the scope of magic and rely
mostly on syntactic sugar. This should also make understanding things you don't
know yet easier: It is either syntactic sugar and can be applied, or it is magic
with side effects limited to a local area, making it easy to find and look up.
