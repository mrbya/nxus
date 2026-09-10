#include <nuttx/board.h>
#include <nuttx/compiler.h>
#include <nuttx/config.h>
#include <testing/unity.h>

namespace {

void smoke() {
    TEST_ASSERT_EQUAL_UINT32(1, 1);
    TEST_ASSERT_EQUAL_UINT32(500, 500);
}

}  // namespace

extern "C" void setUp() {}

extern "C" void tearDown() {}

extern "C" int main(int argc, FAR char* argv[]) {
    (void)argc;
    (void)argv;

    UNITY_BEGIN();

    RUN_TEST(smoke);

    const int result = UNITY_END();

#ifdef CONFIG_BOARDCTL_POWEROFF
    board_power_off(result);
#endif

    return result;
}
