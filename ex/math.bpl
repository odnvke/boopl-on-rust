IMPORT bit_input;
IMPORT math;
IMPORT print;


P.input1;
    P N;  // выводим 1 сообщение 
    CALL first_num_print;
    
    // вызываем input_8 из bit_input
    CALL input_8;
    
    // переносим с ib_0-7 в i_0-7
    i_{0..8} ib_{0..8};
    // ib_N вывод bit_input
    // i_N вход math


P.input2;

    P N;
    CALL second_num_print;

    CALL input_8;

    i2_{0..8} ib_{0..8};
    // i2_N второй вход math


P.loop_deistvie;
    P N;
    CALL deistvie;

    INBC;  // очищаем буфер ввода
    IN;    // запрашиваем ввод
    t INBC;// t = (буфер ввода?)
    
    // если да переходим в начало
    IG t P.loop_deistvie;
    
    // minus = первый знак из буфера
    minus INB;

    I minus;
        CALL raznica;
        // если 1(true) выводим сообщение и вызываем sub из math
        CALL sub;
    E;

    plus N minus;

    I plus;
        CALL summa;
        // если 0(false) выводим сообщение и вызываем add из math
        CALL add;
    E;  

    P N;
    P o_{0..8};
    // выводим выход math


P.loop2;
    CALL prodoljit;

    INBC;
    IN;
    t INBC;
    
    IG t P.loop2;

    exit INB;

    IG exit P.end;
    // 0 продолжить; 1 выход

P.loop3;
    CALL prodoljit_2;

    INBC;
    IN;
    t INBC;
    
    IG t P.loop3

    pdol_2 INB;
    
    // 1 задать первому входу math выход

    I pdol_2;
         i_{0..8} o_{0..8};
    
         CALL first_num_print;

         P N;
         P i_{0..8};

         G P.input2;
         // выводим и переходим к вводу второго входа
    E;

    G P.input1;
    // переходим к вводу двух 



// метка конца (необязательна)
P.end;
E;



















// ФУНК имя метки без P.
FUNC first_num_print;
    // вызовы принта букв
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
    // возврат
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

    CALL print_1;
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
