// Power manager header
// You will need to implement these functions!


#ifndef __POWER_MANAGER_H_
#define __POWER_MANAGER_H_

namespace PowerManager {
    /**
     * @brief Set the amplifier circuit state
     * 
     * @param power If the amplifier circuit should be powered
     */
    void set_amp_power_state(bool power);

    /**
     * @brief Set the PC circuit state
     * 
     * @param power If the PC circuit should be powered
     */
    void set_pc_power_state(bool power);

    /**
     * @brief Set the amp standby
     * 
     * @param standby If the amplifier standby pin should be enabled
     */
    void set_amp_standby(bool standby);

    /**
     * @brief Set the amp mute state
     * 
     * @param standby If the amplifier mute pin should be set
     */
    void set_amp_mute(bool muted);

}

#endif