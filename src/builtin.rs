// Possible future built-ins
//
// #get - Get table's value at a key/path
// #set - Set table's value at a key/path
// #raw - Interpret values literally (either recursively or only one layer)
// #path - Construct a path from a table
// #scope - Execute a command in a new sub-scope
// #loop - Repeat a command infinitely
// #break - Break out of an infinite loop (with a value?)
// #if - Conditionally execute one of two commands
// #print - Print a string to stdout
// #input - Read a line from stdin
// #read - Load the contents of a file as string
// #write - Store a string into a file
// Arithmetic: #add #sub #neg #shiftr #shiftl
// Booleans: #and #or #not #xor #andb #orb #notb #xorb
// Comparisons: #eq #neq #lt #le #gt #ge

/// Built-in operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Builtin {}
