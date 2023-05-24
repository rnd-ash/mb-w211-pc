#include "can.h"
#include "driver/twai.h"
#include "esp_log.h"
#include "freertos/task.h"
#include "esp_timer.h"
#include "lighting.h"

const twai_timing_config_t CAN_TIMING_83_3bps = {
    .brp = 48,
    .tseg_1 = 15,
    .tseg_2 = 4,
    .sjw = 3,
    .triple_sampling = false
};

Can::Can() {
    this->can_active = false;
    this->hazards_pressed = false;
}

void Can::setup_tasks() {
    xTaskCreate(Can::start_tx_task, "CAN_TX", 8192, this, 5, nullptr);
    xTaskCreate(Can::start_rx_task, "CAN_RX", 8192, this, 5, nullptr);
}

void Can::setup() {
    twai_general_config_t gen_config = TWAI_GENERAL_CONFIG_DEFAULT(CAN_TX_PIN, CAN_RX_PIN, TWAI_MODE_NORMAL);
    //gen_config.intr_flags = ESP_INTR_FLAG_IRAM;
    gen_config.rx_queue_len = 32;
    gen_config.tx_queue_len = 32;
    twai_filter_config_t filter_config = TWAI_FILTER_CONFIG_ACCEPT_ALL();
    esp_err_t err = twai_driver_install(&gen_config, &CAN_TIMING_83_3bps, &filter_config);
    twai_start();
    this->can_active = true;
}

void Can::shutdown() {
    this->can_active = false;
    twai_stop();
    twai_driver_uninstall();
}

void Can::start_tx_task(void* params) {
    static_cast<Can*>(params)->tx_task();
}

void Can::start_rx_task(void* params) {
    static_cast<Can*>(params)->rx_task();
}

void to_tx_frame(twai_message_t* tx, uint16_t id, uint8_t dlc, uint64_t data) {
    tx->identifier = id;
    tx->data_length_code = dlc;
    tx->flags = 0;
    for(uint8_t i = 0; i < 8; i++) {
        tx->data[7-i] = data & 0xFF;
        data >>= 8;
    }
}

uint8_t Can::get_light_level_target_display() {
    uint64_t now = esp_timer_get_time()/1000;
    DBE_A1 dbea1 = {};
    if (dbe.get_DBE_A1(now, 500, &dbea1)) {
        if (dbea1.get_TUNNEL()) {
            return LIGHT_PWM_TUNNEL;
        } else if (dbea1.get_NACHT()) { // Night mode
            return LIGHT_PWM_NIGHT;
        } else {
            return LIGHT_PWM_DAY;
        }
    } else {
        return 0xFF; // Max brightness if error (So the display is readable)
    }
}

void Can::tx_task() {
    uint64_t last_obf_tx_time = 0;
    twai_message_t tx;
    while(1) {
        if (this->can_active) {
            uint64_t now = esp_timer_get_time()/1000;
            if (now > last_obf_tx_time + 100) {
                // Check if ignition or if hazards is on / just turned off
                if (this->can_passive_send_data() || this->hazards_pressed || this->hazards_state_change) {
                    if (this->hazards_state_change) {
                        this->hazards_pressed = !this->hazards_pressed;
                        this->hazards_state_change = false;
                    }
                    tx_frame_obf.set_WBL_EIN(this->hazards_pressed);
                    // Check signals from PC
                    PC_CTRL_PANEL ctrl = {};
                    if (pc.get_PC_CTRL_PANEL(now, 500, &ctrl)) {
                        tx_frame_obf.set_ESP_BET(ctrl.get_ESPOFF()); // ESP Button
                        tx_frame_obf.set_SHZ_VL_NS(ctrl.get_PASS_HEATER_PRESSED()); // Passenger heater
                        tx_frame_obf.set_SHZ_VR_NS(ctrl.get_DRIVER_HEATER_PRESSED()); // Driver heater
                        tx_frame_obf.set_SBL_VL_NS(ctrl.get_PASS_COOLER_PRESSED()); // Passenger cooler
                        tx_frame_obf.set_SBL_VR_NS(ctrl.get_DRIVER_COOLER_PRESSED()); // Driver cooler
                        tx_frame_obf.set_FKS_BET(ctrl.get_HEADREST()); // Headrests
                        tx_frame_obf.set_ZV_VERRI_IS(ctrl.get_LOCK()); // Lock
                        tx_frame_obf.set_ZV_ENTRI_IS(ctrl.get_UNLOCK()); // Unlock
                        tx_frame_obf.set_HR_BET(ctrl.get_BLIND()); // Blind
                    }


                    to_tx_frame(&tx, OBF_A1_CAN_ID, 4, tx_frame_obf.raw);
                    twai_transmit(&tx, 0);
                }
            }
        }
        vTaskDelay(100);
    }
}

// Get ignition status. Check if key is present in EZS.
// If key is not present, then we must not passively send CAN data
// (instead, we can only send Hazard data if the button is pressed)
bool Can::can_passive_send_data() {
    uint64_t now = esp_timer_get_time()/1000;
    EZS_A1 ezsa1;
    // Signal times out after 1 second. If no data is received
    // after 1 second, we assume key off
    if (ezs.get_EZS_A1(now, 1000, &ezsa1)) {
        return ezsa1.get_KL_15R_EIN(); // Terminal 15R is active if key is in pos 1 or higher
    } else {
        return false;
    }
}

uint64_t tmp = 0;
inline uint64_t frame_to_uint64(twai_message_t* msg) {
    tmp = 0;
    for (int i = 0; i < msg->data_length_code; i++) {
        tmp |= (uint64_t)msg->data[i] << (8*(7-i));
    }
    return tmp;
}

void Can::rx_task() {
    twai_message_t rx = {};
    twai_status_info_t status;
    uint64_t last_rx_time = 0;
    while(1) {
        if (this->can_active) {
            uint64_t now = esp_timer_get_time()/1000;
            twai_get_status_info(&status);
            if (status.msgs_to_rx != 0) {
                last_rx_time = now;
                for (int f = 0; f < status.msgs_to_rx; f++) {
                    if (twai_receive(&rx, 0) == ESP_OK && rx.data_length_code != 0 && rx.flags == 0) { // Ignore non data frames
                        // Ignition state, import into EZS
                        if (rx.identifier == EZS_A1_CAN_ID) {
                            ezs.import_frames(frame_to_uint64(&rx), rx.identifier, now);
                        } else if (rx.identifier == SAM_H_A3_CAN_ID) {
                            sam.import_frames(frame_to_uint64(&rx), rx.identifier, now);
                            // Set the expire time now!
                            SAM_H_A3 sama3;
                            if (this->sam.get_SAM_H_A3(now, 2550, &sama3)) {
                                if (sama3.get_WARN_AKT()) { // Hazards are active
                                    this->led_on_expire_time = now + (sama3.get_HELL_BLINK()*10);
                                } else { // Not the hazards active, so don't register it
                                    this->led_on_expire_time = 0;
                                }
                            } else {
                                this->led_on_expire_time = 0; // No blink
                            }
                        } else if (rx.identifier == DBE_A1_CAN_ID) {
                            dbe.import_frames(frame_to_uint64(&rx), rx.identifier, now);
                        } else if (rx.identifier == PC_CTRL_PANEL_CAN_ID) {
                            pc.import_frames(frame_to_uint64(&rx), rx.identifier, now);
                        }
                    }
                }
            }
        } else {
        
        }
        vTaskDelay(20);
    }
}
