IMPORT bit_input;
IMPORT math;

IMPORT messages;


P.vvod;


PD.R P.1;
G P.first_num_print;
P.1;

PD.R P.0;
G P.input_8;
P.0;

i_0 ib_0;  i_1 ib_1;  i_2 ib_2;  i_3 ib_3;
i_4 ib_4;  i_5 ib_5;  i_6 ib_6;  i_7 ib_7;


P N;

P.vvod_2;

PD.R P.2;
G P.second_num_print;
P.2;

PD.R P.3;
G P.input_8;
P.3;

i2_0 ib_0;  i2_1 ib_1;  i2_2 ib_2;  i2_3 ib_3;
i2_4 ib_4;  i2_5 ib_5;  i2_6 ib_6;  i2_7 ib_7;

P N;

PD.R P.9;
G P.deistvie;
P.9;

INBC;
IN;
minus INB;

I minus;
    PD.R P.10;
    G P.raznica;
    P.10;

    PD.R P.11;
    G P.sub;
    P.11;
E;

plus N minus;

I plus;
    PD.R P.4;
    G P.summa;
    P.4;
        
    PD.R P.5;
    G P.add;
    P.5;
E;

P N;
P o_7; P o_6; P o_5; P o_4; P o_3; P o_2; P o_1; P o_0; P N;

PD.R P.6;
G P.prodoljit;
P.6;

INBC;
IN;
pdol INB;

IG pdol P.end;

PD.R P.7;
G P.prodoljit_2;
P.7;

INBC;
IN;
pdol_2 INB;

I pdol_2;
    i_0 o_0;  i_1 o_1;  i_2 o_2;  i_3 o_3;
    i_4 o_4;  i_5 o_5;  i_6 o_6;  i_7 o_7;
    
    PD.R P.8;
    G P.first_num_print;
    P.8;

    P N;
    P o_7; P o_6; P o_5; P o_4; P o_3; P o_2; P o_1; P o_0; P N;

    G P.vvod_2;
E;

G P.vvod;

E;

P.end;
