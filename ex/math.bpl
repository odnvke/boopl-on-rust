IMPORT bit_input;
IMPORT math;
IMPORT print;


P.vvod;

CALL first_num_print;

CALL input_8;

i_0 ib_0;  i_1 ib_1;  i_2 ib_2;  i_3 ib_3;
i_4 ib_4;  i_5 ib_5;  i_6 ib_6;  i_7 ib_7;


P N;

P.vvod_2;

CALL second_num_print;

CALL input_8;

i2_0 ib_0;  i2_1 ib_1;  i2_2 ib_2;  i2_3 ib_3;
i2_4 ib_4;  i2_5 ib_5;  i2_6 ib_6;  i2_7 ib_7;

P N;

CALL deistvie;

INBC;
IN;
minus INB;

I minus;
    CALL raznica;

    CALL sub;
E;

plus N minus;

I plus;
    CALL summa;
        
    CALL add;
E;

P N;
P o_7; P o_6; P o_5; P o_4; P o_3; P o_2; P o_1; P o_0; P N;

CALL prodoljit;

INBC;
IN;
pdol INB;

IG pdol P.end;

CALL prodoljit_2;

INBC;
IN;
pdol_2 INB;

I pdol_2;
    i_0 o_0;  i_1 o_1;  i_2 o_2;  i_3 o_3;
    i_4 o_4;  i_5 o_5;  i_6 o_6;  i_7 o_7;
    
    CALL first_num_print;

    P N;
    P o_7; P o_6; P o_5; P o_4; P o_3; P o_2; P o_1; P o_0; P N;

    G P.vvod_2;
E;

G P.vvod;

E;

P.end;





















FUNC first_num_print;
    CALL print_ru_P;
    CALL print_ru_e;
    CALL print_ru_r;
    CALL print_ru_v;
    CALL print_ru_e;

    P S;

    CALL print_ru_b;
    CALL print_ru_i;
    CALL print_ru_t;
    CALL print_ru_y;
    CALL print_colon;

    RET E;



FUNC second_num_print;
    P N;

    CALL print_ru_V;
    CALL print_ru_t;
    CALL print_ru_o;
    CALL print_ru_r;
    CALL print_ru_y;
    CALL print_ru_e;

    P S;

    CALL print_ru_b;
    CALL print_ru_i;
    CALL print_ru_t;
    CALL print_ru_y;
    CALL print_colon;

    RET E;


FUNC summa;
    P N;

    CALL print_ru_S;
    CALL print_ru_u;
    CALL print_ru_m;
    CALL print_ru_m;
    CALL print_ru_a;
    CALL print_colon;

    P N;

    RET E;



FUNC prodoljit;
    P N;

    CALL print_0;
    P S; P S;

    CALL print_ru_P;
    CALL print_ru_r;
    CALL print_ru_o;
    CALL print_ru_d;
    CALL print_ru_o;
    CALL print_ru_l;
    CALL print_ru_zh;
    CALL print_ru_i;
    CALL print_ru_t;
    CALL print_ru_magkiy;

    P N;

    CALL print_1;
    P S; P S;

    CALL print_ru_V;
    CALL print_ru_y;
    CALL print_ru_h;
    CALL print_ru_o;
    CALL print_ru_d;

    P N;

    RET E;




FUNC prodoljit_2;
    P N;

    CALL print_0;
    P S; P S;

    CALL print_ru_N;
    CALL print_ru_o;
    CALL print_ru_v;
    CALL print_ru_y;
    CALL print_ru_e;

    P S;

    CALL print_ru_b;
    CALL print_ru_i;
    CALL print_ru_t;
    CALL print_ru_y;

    P N;

    CALL print_1;
    P S; P S;

    CALL print_ru_P;
    CALL print_ru_r;
    CALL print_ru_o;
    CALL print_ru_d;
    CALL print_ru_l;
    CALL print_ru_zh;
    CALL print_ru_i;
    CALL print_ru_t;
    CALL print_ru_magkiy;

    P N;

    RET E;

FUNC deistvie;
    P N;

    CALL print_0;
    P S; P S;

    CALL print_ru_P;
    CALL print_ru_l;
    CALL print_ru_yu;
    CALL print_ru_s;

    P N;

    CALL print_1;
    P S; P S;

    CALL print_ru_M;
    CALL print_ru_i;
    CALL print_ru_n;
    CALL print_ru_u;
    CALL print_ru_s;

    P N;

    RET E;



FUNC raznica;
    P N;

    CALL print_ru_R;
    CALL print_ru_a;
    CALL print_ru_z;
    CALL print_ru_n;
    CALL print_ru_i;
    CALL print_ru_ts;
    CALL print_ru_a;
    CALL print_colon;

    P N;

    RET E;
