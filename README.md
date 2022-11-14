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
