#ifndef __CAN_H_
#define __CAN_H_

#include "driver/twai.h"

#include "../lib/w211_canb/src/EZS.h"
#include "../lib/w211_canb/src/OBF.h"
#include "../lib/w211_canb/src/SAM_H.h"
#include "../lib/w211_canb/src/DBE.h"
#include "../lib/w211_canb/src/PC.h"

#define CAN_TX_PIN gpio_num_t::GPIO_NUM_5
#define CAN_RX_PIN gpio_num_t::GPIO_NUM_18

class Can {
public:
    Can();
    // Called ONCE on CPU startup
    void setup_tasks();
    void setup();
    void shutdown();
    bool can_passive_send_data();
    uint8_t get_light_level_target_display();
    static void start_tx_task(void* params);
    static void start_rx_task(void* params);
    bool hazards_state_change = false;
    bool hazards_pressed = false;
    uint64_t led_on_expire_time = 0;
    uint64_t last_rx_time = 0;
private:
    [[noreturn]]
    void tx_task();

    [[no_return]]
    void rx_task();

private:
    bool can_active = false;
    ECU_EZS ezs = ECU_EZS();
    ECU_DBE dbe = ECU_DBE();

    OBF_A1 tx_frame_obf = {};
    ECU_SAM_H sam = ECU_SAM_H(); // For blinking lights duration
    ECU_PC pc = ECU_PC();
};

#endif