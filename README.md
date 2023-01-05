# Rustython

A simple python interpreter meant as a way of learning how python works

Since I dont want to bother with generating the python AST (Abstract Syntax Tree), im going to be using the `tree-sitter` and `tree-sitter-python` crates to do the parsing.

Goals:

-   [ ] Learning how the python interpreter works
-   [ ] Make a working "Hello World" program
-   [ ] Adding loops, functions, conditions etc...
-   [ ] (This may not happen) ability to import other files or packages

Sources

-   https://unpyc.sourceforge.net/Opcodes.html
-   https://aosabook.org/en/500L/a-python-interpreter-written-in-python.html
-   https://leanpub.com/insidethepythonvirtualmachine/read
-   https://github.com/python/cpython
    -   https://github.com/python/cpython/blob/main/Lib/opcode.py
-   https://eli.thegreenplace.net/tag/python-internals
