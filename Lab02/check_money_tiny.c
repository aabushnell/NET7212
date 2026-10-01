#include <stdlib.h>
#include <stdint.h>
#include <check.h>
#include "money.h"

Money *five_dollars;

void setup(void) {
    five_dollars = money_create(5, "USD");
}

void teardown(void) {
    money_free(five_dollars);
}

START_TEST(test_money_create_amount) {
    ck_assert_double_eq(money_amount(five_dollars), 5);
}
END_TEST

Suite * money_suite(void) {
    Suite *s;
    TCase *tc_core;

    s = suite_create("Money");

    tc_core = tcase_create("Core");
    tcase_add_checked_fixture(tc_core, setup, teardown);
    tcase_add_test(tc_core, test_money_create_amount);
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
