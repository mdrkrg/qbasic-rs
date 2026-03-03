10 REM Test relational operators
20 LET X = 10
30 LET Y = 20
40 LET Z = 10
50 IF X < Y THEN 80
60 PRINT "X < Y is false"
70 GOTO 90
80 PRINT "X < Y is true"
90 IF X <= Z THEN 120
100 PRINT "X <= Z is false"
110 GOTO 130
120 PRINT "X <= Z is true"
130 IF Y > X THEN 160
140 PRINT "Y > X is false"
150 GOTO 170
160 PRINT "Y > X is true"
170 IF Y >= X THEN 200
180 PRINT "Y >= X is false"
190 GOTO 210
200 PRINT "Y >= X is true"
210 IF X <> Y THEN 240
220 PRINT "X <> Y is false"
230 GOTO 250
240 PRINT "X <> Y is true"
250 END
