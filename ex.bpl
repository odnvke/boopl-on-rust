CALL a;

P N;

P S; P S; P S; P S; P S; 
CALL b;




FUNC a;
    P F; P F; P F; P T; P S;

    CALL b;

    P F; P F; P F; P T; P S;
RET E;


FUNC b;
    P F; P F; P T; P F; P S;

    CALL c;

    P F; P F; P T; P F; P S;
RET E;


FUNC c;
    P F; P T; P F; P F; P S;
    
    CALL d;

    P F; P T; P F; P F; P S;
RET E;


FUNC d;
    P T; P F; P F; P F; P S;
    
    CALL spliter;

    P T; P F; P F; P F; P S;
RET E;


FUNC spliter;
    P S;

    CALL print_colon;

    P S; P S;
RET E;


FUNC print_colon;
    sb_0 F;  sb_1 F;  sb_2 T;  sb_3 T;  sb_4 T;  sb_5 F;  sb_6 T;  sb_7 F;
    P U sb_0;
RET E;