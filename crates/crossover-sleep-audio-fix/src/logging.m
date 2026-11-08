#include <dispatch/dispatch.h>
#include <os/log.h>
#include <stdint.h>

void sleep_audio_log(uint8_t level, const char *message) {
    static os_log_t logger;
    static dispatch_once_t once;
    dispatch_once(&once, ^{ logger = os_log_create("com.github.scryner.crosspuck", "audio"); });
    os_log_type_t type = level == 1 ? OS_LOG_TYPE_ERROR :
                         level <= 3 ? OS_LOG_TYPE_DEFAULT : OS_LOG_TYPE_DEBUG;
    os_log_with_type(logger, type, "%{public}s", message);
}
