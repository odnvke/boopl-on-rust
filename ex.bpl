// Тестовые числа: 5 (00000101) и 3 (00000011)
a0 T; a1 F; a2 T; a3 F; a4 F; a5 F; a6 F; a7 F;
b0 T; b1 T; b2 F; b3 F; b4 F; b5 F; b6 F; b7 F;

// Результат
r0 F; r1 F; r2 F; r3 F; r4 F; r5 F; r6 F; r7 F; c F;

// Временные
t1 F; t2 F; t3 F;

// Вывод a
P a7; P a6; P a5; P a4; P a3; P a2; P a1; P a0; P N;

// Вывод b  
P b7; P b6; P b5; P b4; P b3; P b2; P b1; P b0; P N;

// Сложение - исправленная версия
// Бит 0
t1 X a0 b0;     // a XOR b
r0 X t1 c;      // (a XOR b) XOR c (c изначально 0)
t1 A a0 b0;     // a AND b
t2 A a0 c;      // a AND c  
t3 A b0 c;      // b AND c
t2 O t1 t2;     // (a AND b) OR (a AND c)
c O t2 t3;      // (a AND b) OR (a AND c) OR (b AND c)

// Бит 1
t1 X a1 b1;
r1 X t1 c;
t1 A a1 b1;
t2 A a1 c;
t3 A b1 c;
t2 O t1 t2;
c O t2 t3;

// Бит 2
t1 X a2 b2;
r2 X t1 c;
t1 A a2 b2;
t2 A a2 c;
t3 A b2 c;
t2 O t1 t2;
c O t2 t3;

// Бит 3
t1 X a3 b3;
r3 X t1 c;
t1 A a3 b3;
t2 A a3 c;
t3 A b3 c;
t2 O t1 t2;
c O t2 t3;

// Бит 4
t1 X a4 b4;
r4 X t1 c;
t1 A a4 b4;
t2 A a4 c;
t3 A b4 c;
t2 O t1 t2;
c O t2 t3;

// Бит 5
t1 X a5 b5;
r5 X t1 c;
t1 A a5 b5;
t2 A a5 c;
t3 A b5 c;
t2 O t1 t2;
c O t2 t3;

// Бит 6
t1 X a6 b6;
r6 X t1 c;
t1 A a6 b6;
t2 A a6 c;
t3 A b6 c;
t2 O t1 t2;
c O t2 t3;

// Бит 7
t1 X a7 b7;
r7 X t1 c;
t1 A a7 b7;
t2 A a7 c;
t3 A b7 c;
t2 O t1 t2;
c O t2 t3;

// Вывод результата
P r7; P r6; P r5; P r4; P r3; P r2; P r1; P r0; P N;
P S; P c;

E;