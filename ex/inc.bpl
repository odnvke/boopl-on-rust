IMPORT math;
IMPORT cmp;

var_{0..15} F;

var2_{0..15} T;

i_{0..7} var_{0..7};
o_{0..7} F;

P.loop;
    i_{0..7} var_{0..7};

    CALL inc;
    
    var_{0..7} o_{0..7};

    IF c_out;
        i_{0..7} var_{8..15};

        CALL inc;
        
        var_{8..15} o_{0..7};
    E;

    i_{0..7} var_{8..15};
    i2_{0..7} var2_{8..15};

    CALL cmp_eq;
    
    IF eq_flag;        
        i_{0..7} var_{0..7};
        i2_{0..7} var2_{0..7};
        
        CALL cmp_eq;
    
        IFG eq_flag P.end;
    E;

    P var_{15..0}; P N;

G P.loop;

P.end;

