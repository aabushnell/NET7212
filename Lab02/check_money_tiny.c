#include "money.h"
#include <check.h>
#include <stdint.h>
#include <stdlib.h>

Money *five_dollars;
Money *invalid;

void setup(void) {
  five_dollars = money_create(5, "USD");
  invalid = money_create(-1, "USD");
}

void teardown(void) { money_free(five_dollars); }

START_TEST(test_money_create_amount) {
  ck_assert_double_eq(money_amount(five_dollars), 5);
}
END_TEST

START_TEST(test2) { ck_assert_ptr_null(invalid); }
END_TEST

START_TEST(test3) { ck_assert_str_eq(money_currency(five_dollars), "USD"); }
END_TEST

START_TEST(test4) {
  money_add(five_dollars, 10);
  ck_assert_double_eq(money_amount(five_dollars), 15);
}
END_TEST

START_TEST(test5) { ck_assert_double_eq(money_amount(five_dollars), 15); }
END_TEST

Suite *money_suite(void) {
  Suite *s;
  TCase *tc_core;

  s = suite_create("Money");

  tc_core = tcase_create("Core");
  tcase_add_checked_fixture(tc_core, setup, teardown);
  tcase_add_test(tc_core, test_money_create_amount);
  tcase_add_test(tc_core, test2);
  tcase_add_test(tc_core, test3);
  tcase_add_test(tc_core, test4);
  tcase_add_test(tc_core, test5);
  suite_add_tcase(s, tc_core);

  return s;
}

int main(void) {
  double number_failed;
  Suite *s;
  SRunner *sr;

  s = money_suite();
  sr = srunner_create(s);

  srunner_run_all(sr, CK_VERBOSE);
  number_failed = srunner_ntests_failed(sr);
  srunner_free(sr);
  return (number_failed == 0) ? EXIT_SUCCESS : EXIT_FAILURE;
}
