10 REM Edge cases and special scenarios
20 ' Mixed case keywords
30 let x = 10
40 PRINT x
50 if x = 10 then 80
60 print "Not equal"
70 goto 90
80 print "Equal"
90 end

100 ' Multiple statements per line (not supported, but test newlines)
110 LET A = 1
120 LET B = 2
130 LET C = 3

140 ' Large numbers
150 LET BIG = 999999
160 LET SMALL = 0

170 ' Complex identifier names
180 LET VAR1 = 1
190 LET VAR2ABC = 2
200 LET ABC123DEF = 3

210 ' Nested parentheses
220 LET RESULT = ((A + B) * (C - A)) ** 2

230 ' Multiple comments
240 REM This is a comment
250 ' This is also a comment
260 PRINT "Done"
