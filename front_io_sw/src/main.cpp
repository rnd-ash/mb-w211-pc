#include "driver/gpio.h"
#include <freertos/FreeRTOS.h>
#include <freertos/task.h>
#include "esp_log.h"
#include "esp_timer.h"
#include "driver/ledc.h"
#include "driver/dac.h"
#include "esp_sleep.h"
#include "can/can.h"
#include "esp_task_wdt.h"



#define PIN_5V_SHUTOFF GPIO_NUM_12
#define HZARD_PIN GPIO_NUM_26
#define HZARD_BACKLIGHT_PIN GPIO_NUM_14
#define HZARD_LIGHT_PIN GPIO_NUM_27
#define LED_PWM_PIN GPIO_NUM_15
#define PIN_COOLER_FAN GPIO_NUM_13

bool hazards_on = false;

uint64_t last_btn_time = 0;
uint64_t last_can_time = 0;

uint16_t fan_pwm = 0;

typedef struct {
    uint32_t freq;
    gpio_num_t pin;
    ledc_channel_t channel;
    ledc_timer_t timer;
} PwmIo;

// Fan

// Backlight
const PwmIo BackLightPwmConfig = {
    .freq = 360,
    .pin = LED_PWM_PIN,
    .channel = ledc_channel_t::LEDC_CHANNEL_0,
    .timer = ledc_timer_t::LEDC_TIMER_0
};

const PwmIo FanPwmConfig = {
    .freq = 5000,
    .pin = PIN_COOLER_FAN,
    .channel = ledc_channel_t::LEDC_CHANNEL_1,
    .timer = ledc_timer_t::LEDC_TIMER_1
};

void turn_off_pwm_channel(PwmIo pwm_cfg) {
    //ledc_stop(ledc_mode_t::LEDC_HIGH_SPEED_MODE, pwm_cfg.channel, 0);
    gpio_set_direction(pwm_cfg.pin, gpio_mode_t::GPIO_MODE_OUTPUT);
    gpio_set_level(pwm_cfg.pin, 0);
}

void configure_pwm_channel(PwmIo pwm_cfg, uint16_t initial_freq) {
    gpio_set_direction(pwm_cfg.pin, gpio_mode_t::GPIO_MODE_OUTPUT);
    ledc_timer_config_t timer_cfg = {
        .speed_mode = ledc_mode_t::LEDC_HIGH_SPEED_MODE, // Low speed timer mode
        .duty_resolution = LEDC_TIMER_8_BIT,
        .timer_num = pwm_cfg.timer,
        .freq_hz = pwm_cfg.freq,
        .clk_cfg = LEDC_AUTO_CLK
    };
    ledc_channel_config_t channel_cfg = {
        .gpio_num = pwm_cfg.pin,
        .speed_mode = ledc_mode_t::LEDC_HIGH_SPEED_MODE,
        .channel = pwm_cfg.channel,
        .intr_type = LEDC_INTR_DISABLE, // Disable fade interrupt
        .timer_sel = pwm_cfg.timer,
        .duty = 0,
        .hpoint = 0
    };
    ledc_timer_config(&timer_cfg);
    ledc_channel_config(&channel_cfg);
    ledc_set_duty(ledc_mode_t::LEDC_HIGH_SPEED_MODE, pwm_cfg.channel, initial_freq);
    ledc_update_duty(ledc_mode_t::LEDC_HIGH_SPEED_MODE, pwm_cfg.channel);
}

Can* can;
static void IRAM_ATTR on_hazards_pressed(void *args) {
    uint64_t now = esp_timer_get_time()/ 1000;
    if (now > last_btn_time + 100) { // Debounce check
        can->hazards_state_change = true;
        last_btn_time = now;
    }
}

void sleep_and_wakeup() {
    ESP_LOGI("S&W", "Going to sleep!");
    gpio_set_level(PIN_5V_SHUTOFF, 1); // Shut down 5V rail for the HDMI board
    gpio_set_level(HZARD_LIGHT_PIN, 0); // Turn off hazards light pin
    gpio_set_level(HZARD_BACKLIGHT_PIN, 0); // Turn off hazard backlight pin
    gpio_set_level(PIN_COOLER_FAN, 0); // Turn off fan
    //turn_off_pwm_channel(FanPwmConfig);
    turn_off_pwm_channel(BackLightPwmConfig);
    gpio_isr_handler_remove(HZARD_PIN);
    
    can->shutdown(); // Disable the CAN transeiver on ESP, reconfigure CAN Rx pin as generic GPIO
    gpio_wakeup_enable(CAN_RX_PIN, GPIO_INTR_LOW_LEVEL); // Wakeup on CAN activity (Pull low to wake)
    gpio_wakeup_enable(HZARD_PIN, GPIO_INTR_LOW_LEVEL); // Wakeup on Hazards button press (Pull low to wake)
    esp_sleep_enable_gpio_wakeup();
    esp_light_sleep_start();

    /**
     * SLEEPING HERE
    */
    ESP_LOGI("S&W", "Woken up!"); // So now set everything back up!
    can->setup();
    gpio_set_direction(HZARD_PIN, gpio_mode_t::GPIO_MODE_INPUT);
    gpio_set_intr_type(HZARD_PIN, GPIO_INTR_POSEDGE);
    gpio_isr_handler_add(HZARD_PIN, on_hazards_pressed, nullptr);
    gpio_set_level(PIN_COOLER_FAN, 1);
    gpio_set_level(HZARD_LIGHT_PIN, 1);
    //configure_pwm_channel(FanPwmConfig, 192); // Configure fan
    configure_pwm_channel(BackLightPwmConfig, 128); // Configure backlight
    gpio_set_level(PIN_5V_SHUTOFF, 0);
    gpio_set_level(HZARD_BACKLIGHT_PIN, 0);                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       
}

void backlight_demo(void*) {
    uint8_t pwm = 0xFF;
    uint8_t pwm_target = 0xFF;
    while(1) {  
        pwm_target = can->get_light_level_target_display();
        ledc_set_duty(ledc_mode_t::LEDC_HIGH_SPEED_MODE, BackLightPwmConfig.channel, 0xFF-pwm);
        ledc_update_duty(ledc_mode_t::LEDC_HIGH_SPEED_MODE, BackLightPwmConfig.channel);
        if (pwm_target > pwm) {
            pwm += 1;
        } else if (pwm_target < pwm) {
            pwm -= 1;
        }
        vTaskDelay(25); // Slow backlight fading
    }
}

extern "C" void app_main(void) {
    gpio_reset_pin(PIN_5V_SHUTOFF);
    gpio_reset_pin(PIN_COOLER_FAN);
    gpio_reset_pin(LED_PWM_PIN);

    gpio_reset_pin(HZARD_BACKLIGHT_PIN);
    gpio_reset_pin(HZARD_LIGHT_PIN);
    gpio_reset_pin(HZARD_PIN);
    gpio_reset_pin(CAN_RX_PIN);
    gpio_reset_pin(CAN_TX_PIN);

    gpio_set_direction(PIN_5V_SHUTOFF, gpio_mode_t::GPIO_MODE_OUTPUT);
    gpio_set_direction(PIN_COOLER_FAN, gpio_mode_t::GPIO_MODE_OUTPUT);
    gpio_set_direction(HZARD_LIGHT_PIN, gpio_mode_t::GPIO_MODE_OUTPUT);
    gpio_set_direction(HZARD_PIN, gpio_mode_t::GPIO_MODE_INPUT);
    gpio_pullup_en(HZARD_PIN);
    gpio_install_isr_service(0);
    gpio_set_intr_type(HZARD_PIN, GPIO_INTR_POSEDGE);
    gpio_isr_handler_add(HZARD_PIN, on_hazards_pressed, nullptr);
    gpio_set_level(HZARD_LIGHT_PIN, 0);
    gpio_set_level(PIN_5V_SHUTOFF, 0);
    gpio_set_level(PIN_COOLER_FAN, 1);
    gpio_set_level(HZARD_BACKLIGHT_PIN, 0);
    can = new Can();
    can->setup_tasks();
    can->setup();
    configure_pwm_channel(BackLightPwmConfig, 128);
    //configure_pwm_channel(FanPwmConfig, 192);
    twai_message_t read;
    uint64_t last_wake_time = 0;
    xTaskCreate(backlight_demo, "BDEMO", 8192, nullptr, 5, nullptr);
    while(1) {
        //ESP_LOGI("T", "Running");
        uint64_t now = esp_timer_get_time()/1000;
        //ESP_LOGI("MAIN", "Hazards is %d", can->hazards_pressed);
        gpio_set_level(HZARD_LIGHT_PIN, can->led_on_expire_time >= now);
        if (can->can_passive_send_data() || now < can->last_rx_time + 1000) {
            last_can_time = esp_timer_get_time()/1000;
            last_wake_time = esp_timer_get_time()/1000;
        }
        if (can->hazards_pressed) {
            last_wake_time = now;
        }
        if (now > last_wake_time + 5000) {
            sleep_and_wakeup();
            // After wakeup
            last_wake_time = esp_timer_get_time()/1000;
        }
        vTaskDelay(10/portTICK_PERIOD_MS);
    }
}