PD.__ret;

// final_comprehensive_test.bpl






// Основной код с разными паттернами вызовов
PD.ret1 P.start1;
PD.ret2 P.start2;

P.start1;
// CALL calculate
PD.__ret P.__ret_0;
G P.__func_calculate_body;
P.__ret_0;
// CALL setup
PD.__ret P.__ret_1;
G P.__func_setup_body;
P.__ret_1;
G PD.ret2;

P.start2;
// CALL process
PD.__ret P.__ret_2;
G P.__func_process_body;
P.__ret_2;
// CALL cleanup
PD.__ret P.__ret_3;
G P.__func_cleanup_body;
P.__ret_3;
E;

// === Тела функций ===

P.__func_helper2_body;
  PD.__local_ret_helper2 PD.__ret;
  203 T;
  204 T;
  G PD.__local_ret_helper2;

P.__func_cleanup_body;
  PD.__local_ret_cleanup PD.__ret;
  300 T;
  301 T;
  G PD.__local_ret_cleanup;

P.__func_calculate_body;
  PD.__local_ret_calculate PD.__ret;
  // CALL setup
  PD.__ret P.__ret_calculate_0;
  G P.__func_setup_body;
  P.__ret_calculate_0;
  // CALL process
  PD.__ret P.__ret_calculate_1;
  G P.__func_process_body;
  P.__ret_calculate_1;
  // CALL cleanup
  PD.__ret P.__ret_calculate_2;
  G P.__func_cleanup_body;
  P.__ret_calculate_2;
  999 T;
  G PD.__local_ret_calculate;

P.__func_helper1_body;
  PD.__local_ret_helper1 PD.__ret;
  201 T;
  202 T;
  G PD.__local_ret_helper1;

P.__func_process_body;
  PD.__local_ret_process PD.__ret;
  // CALL helper1
  PD.__ret P.__ret_process_0;
  G P.__func_helper1_body;
  P.__ret_process_0;
  // CALL helper2
  PD.__ret P.__ret_process_1;
  G P.__func_helper2_body;
  P.__ret_process_1;
  200 T;
  G PD.__local_ret_process;

P.__func_setup_body;
  PD.__local_ret_setup PD.__ret;
  100 T;
  101 T;
  102 T;
  G PD.__local_ret_setup;
