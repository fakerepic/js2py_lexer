# js2py_lexer

## Description

This is a lexer for the javascript language. It is implemented in rust to tokenize the input. The lexer is able to tokenize the following javascript code:

```js
// test.js
var a = 0.e1;
var b = 2e001;
var c = a + b;
console.log(a === b);

let d = () => {;
/* 
 * multiline comment 
 * */
  console.log('d\
    \A \
    n');
};
a=0
++a
// comment

```

to the following tokens:

```bash
# cargo run test.js
Token { typ: Var, start: 0, end: 3 }
Token { typ: Identifier, start: 4, end: 5 }
Token { typ: Eq, start: 6, end: 7 }
Token { typ: Decimal, start: 8, end: 12 }
Token { typ: Semicolon, start: 12, end: 13 }
Token { typ: LineTerminator, start: 13, end: 14 }
Token { typ: Var, start: 14, end: 17 }
Token { typ: Identifier, start: 18, end: 19 }
Token { typ: Eq, start: 20, end: 21 }
Token { typ: Decimal, start: 22, end: 27 }
Token { typ: Semicolon, start: 27, end: 28 }
Token { typ: LineTerminator, start: 28, end: 29 }
Token { typ: Var, start: 29, end: 32 }
Token { typ: Identifier, start: 33, end: 34 }
Token { typ: Eq, start: 35, end: 36 }
Token { typ: Identifier, start: 37, end: 38 }
Token { typ: Plus, start: 39, end: 40 }
Token { typ: Identifier, start: 41, end: 42 }
Token { typ: Semicolon, start: 42, end: 43 }
Token { typ: LineTerminator, start: 43, end: 44 }
Token { typ: Identifier, start: 44, end: 51 }
Token { typ: Dot, start: 51, end: 52 }
Token { typ: Identifier, start: 52, end: 55 }
Token { typ: LParen, start: 55, end: 56 }
Token { typ: Identifier, start: 56, end: 57 }
Token { typ: Eq3, start: 58, end: 61 }
Token { typ: Identifier, start: 62, end: 63 }
Token { typ: RParen, start: 63, end: 64 }
Token { typ: Semicolon, start: 64, end: 65 }
Token { typ: LineTerminator, start: 65, end: 66 }
Token { typ: LineTerminator, start: 66, end: 67 }
Token { typ: Let, start: 67, end: 70 }
Token { typ: Identifier, start: 71, end: 72 }
Token { typ: Eq, start: 73, end: 74 }
Token { typ: LParen, start: 75, end: 76 }
Token { typ: RParen, start: 76, end: 77 }
Token { typ: Eq, start: 78, end: 79 }
Token { typ: RAngle, start: 79, end: 80 }
Token { typ: LCurly, start: 81, end: 82 }
Token { typ: Semicolon, start: 82, end: 83 }
Token { typ: LineTerminator, start: 83, end: 84 }
Token { typ: LineTerminator, start: 84, end: 116 }
Token { typ: Identifier, start: 118, end: 125 }
Token { typ: Dot, start: 125, end: 126 }
Token { typ: Identifier, start: 126, end: 129 }
Token { typ: LParen, start: 129, end: 130 }
Token { typ: Str, start: 130, end: 149 }
Token { typ: RParen, start: 149, end: 150 }
Token { typ: Semicolon, start: 150, end: 151 }
Token { typ: LineTerminator, start: 151, end: 152 }
Token { typ: RCurly, start: 152, end: 153 }
Token { typ: Semicolon, start: 153, end: 154 }
Token { typ: LineTerminator, start: 154, end: 155 }
Token { typ: Identifier, start: 155, end: 156 }
Token { typ: Eq, start: 156, end: 157 }
Token { typ: Decimal, start: 157, end: 158 }
Token { typ: LineTerminator, start: 158, end: 159 }
Token { typ: Plus2, start: 159, end: 161 }
Token { typ: Identifier, start: 161, end: 162 }
Token { typ: LineTerminator, start: 162, end: 163 }
Token { typ: LineTerminator, start: 163, end: 174 }

