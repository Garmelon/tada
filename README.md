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

In the following sections,
- `a`, `b`, ... are arbitrary expressions,
- `foo`, `bar`, ... are identifiers, and
- `{..}` represents an arbitrary list constructor.

### Function calls

| Sugar   | Desugared              |
|---------|------------------------|
| `a(b)`  | `'{ call: a, arg: b }` |
| `a()`   | `a(nil)`               |
| `a{..}` | `a({..})`              |

### Field access

| Sugar       | Desugared       |
|-------------|-----------------|
| `a[b]`      | `'get{a, b}`    |
| `a[b] = c`  | `'set{a, b, c}` |
| `a.foo`     | `a["foo"]`      |
| `a.foo = b` | `a["foo"] = b`  |

### Variable access

| Sugar           | Desugared                 |
|-----------------|---------------------------|
| `[a]`           | `'scope()[a]`             |
| `[a] = b`       | `'scope()[a] = b`         |
| `local [a] = b` | `'setraw('scope(), a, b)` |
| `foo`           | `["foo"]`                 |
| `foo = a`       | `["foo"] = a`             |
| `local foo = a` | `local ["foo"] = a`       |

### Table constructors

| Sugar              | Desugared                     |
|--------------------|-------------------------------|
| `{ a, b, foo: c }` | `'{ raw: '{ a, b, foo: c } }` |
| `{ .., [a] = b }`  | `'set({..}, a, b)`            |

### Table destructuring

`{ foo, bar: baz } = a` is converted to
```
'destructure{
    'scope(),
    { "foo", bar: "baz" },
    a,
}
```

`local { foo, bar: baz } = a` is converted to
```
'destructure{
    'scope(),
    { "foo", bar: "baz" },
    a,
    local: true,
}
```

### Function definitions

`function() a` is converted to
```
{
    '{ quote: a },
    scope: 'scope(),
}
```

`function(foo) a` is converted to
```
function() '{
    local foo = 'arg(),
    a
}
```

`function{..} a` is converted to
```
function() '{
    local {..} = 'arg(),
    a
}
```

| Sugar                  | Desugared                       |
|------------------------|---------------------------------|
| `function foo() a`     | `foo = function() a`            |
| `function foo(a) b`    | `foo = function(a) b`           |
| `function foo{..} a`   | `foo = function{..} a`          |
