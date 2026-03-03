10 REM String operations and INPUT
20 PRINT "What is your name?"
30 INPUT NAME
40 PRINT "Hello "
50 PRINT NAME
60 PRINT "!"
70 PRINT "How old are you?"
80 INPUT AGE
90 IF AGE < 18 THEN 120
100 PRINT "You are an adult."
110 GOTO 130
120 PRINT "You are a minor."
130 LET MESSAGE = "Thank you for using this program!"
140 PRINT MESSAGE
150 PRINT "String with escaped quotes: \"Hello\""
160 PRINT "Another string: 'This is not a comment'"
170 END
