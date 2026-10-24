/* Test-only wall-clock control for differential command sequences.
 * less suppresses repeated EOF bells in the same wall-clock second. Sequential
 * reference/candidate runs otherwise straddle second boundaries nondeterministically.
 * Monotonic clocks and signal timers deliberately remain real.
 */
#include <time.h>

time_t time(time_t *result)
{
    time_t value = (time_t)1700000000;
    if (result != 0)
        *result = value;
    return value;
}
