10 F; 11 F; 12 F;

IF 10;
    P F; P F; P F; P T;   P N;
ELSE IF 11;
    P F; P F; P T; P F;   P N;
ELSE IF 12;
    P F; P T; P F; P F;   P N;
ELSE;
    P F; P T; P F; P F;   P N;
E;

P N; P N;

20 F; 21 T;
30 F; 31 F;

IF 20;
    CALL print_a;

    IF 30;
        CALL print_a;
    ELSE IF 31;
        CALL print_b;
    ELSE;
        CALL print_else;
    E;

ELSE IF 21;
    CALL print_b;

    IF 30;
        CALL print_a;
    ELSE IF 31;
        CALL print_b;
    ELSE;
        CALL print_else;
    E;

ELSE;
    CALL print_else;

    IF 30;
        CALL print_a;
    ELSE IF 31;
        CALL print_b;
    ELSE;
        CALL print_else;
    E;
E;

FUNC print_a;
    sb_0 F;  sb_1 T;  sb_2 T;  sb_3 F;  sb_4 F;  sb_5 F;  sb_6 F;  sb_7 T;
    P U sb_0; P S;
    RET E;

FUNC print_b;
    sb_0 F;  sb_1 T;  sb_2 T;  sb_3 F;  sb_4 F;  sb_5 F;  sb_6 T;  sb_7 F;
    P U sb_0; P S;
    RET E;

FUNC print_else;
    sb_0 F;  sb_1 T;  sb_2 T;  sb_3 F;  sb_4 F;  sb_5 T;  sb_6 F;  sb_7 T;
    P U sb_0;

    sb_0 F;  sb_1 T;  sb_2 T;  sb_3 F;  sb_4 T;  sb_5 T;  sb_6 F;  sb_7 F;
    P U sb_0;
    
    sb_0 F;  sb_1 T;  sb_2 T;  sb_3 T;  sb_4 F;  sb_5 F;  sb_6 T;  sb_7 T;
    P U sb_0;

    sb_0 F;  sb_1 T;  sb_2 T;  sb_3 F;  sb_4 F;  sb_5 T;  sb_6 F;  sb_7 T;
    P U sb_0;

    P S;

    RET E;

