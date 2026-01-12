PD.R;

i_0 F;  i_1 F;  i_2 F;  i_3 F;  
i_4 F;  i_5 F;  i_6 F;  i_7 F;  

i2_0 F;  i2_1 F;  i2_2 F;  i2_3 F;  
i2_4 F;  i2_5 F;  i2_6 F;  i2_7 F;  

c_in F;
c_out F;


o_0 F; o_1 F; o_2 F; o_3 F;
o_4 F; o_5 F; o_6 F; o_7 F;

G P.math_lib_close;
E;


P.add;
    tc c_in;

    t   X i_0 i2_0;
    t2  A i_0 i2_0;
    o_0 X t   c_in;
    t   A t   c_in; 
    tc  O t   t2;

    t   X i_1 i2_1;
    t2  A i_1 i2_1;
    o_1 X t   c_in;
    t   A t   c_in; 
    tc  O t   t2;

    t   X i_2 i2_2;
    t2  A i_2 i2_2;
    o_2 X t   c_in;
    t   A t   c_in; 
    tc  O t   t2;

    t   X i_3 i2_3;
    t2  A i_3 i2_3;
    o_3 X t   c_in;
    t   A t   c_in; 
    tc  O t   t2;

    t   X i_4 i2_4;
    t2  A i_4 i2_4;
    o_4 X t   c_in;
    t   A t   c_in; 
    tc  O t   t2;

    t   X i_5 i2_5;
    t2  A i_5 i2_5;
    o_5 X t   c_in;
    t   A t   c_in; 
    tc  O t   t2;

    t   X i_6 i2_6;
    t2  A i_6 i2_6;
    o_6 X t   c_in;
    t   A t   c_in; 
    tc  O t   t2;

    t   X i_7 i2_7;
    t2  A i_7 i2_7;
    o_7 X t   c_in;
    t   A t   c_in; 
    tc  O t   t2;

    G PD.R;

P.sub;
    i2_0 N i2_0; i2_1 N i2_1;
    i2_2 N i2_2; i2_3 N i2_3;
    i2_4 N i2_4; i2_5 N i2_5;
    i2_6 N i2_6; i2_7 N i2_7;

    c_in N c_in;

    PD._lib_R_1 PD.R;
    PD.R P._math_0;

    G P.add;

    P._math_0;

    G PD._lib_R_1;






P.math_lib_close;