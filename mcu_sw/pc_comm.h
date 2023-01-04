// Adapted for Arduino framework

#ifndef __PC_COMM_H_
#define __PC_COMM_H_

#include <stdint.h>

typedef struct {
    uint32_t id;
    uint8_t data[8];
    uint8_t dlc;
    bool ack;
    bool rtr;
} MCUCanFrame;

typedef struct {
  char can_id;
  uint16_t id;
  uint8_t dlc;
  uint8_t data[8];
}  __attribute__ ((packed)) PcFrame;

/**
 * @brief Prints the CAN Frame on Serial in an encoded string
 * 
 * @param bus Bus tag (Source of the CAN Frame)
 * @param f Frame to print
 */
void print_frame(char bus, MCUCanFrame* f);

/**
 * @brief Since MCU works in Little Endian, but W211 CAN is
 * big ENdian, this helper function flips all the bytes before
 * returning a uint64_t of the data which can be used in a union
 * 
 * @param f Frame input
 * @return uint64_t long of frame data
 */
inline uint64_t frame_to_int(MCUCanFrame* f);

/**
 * @brief Tries to read a PC CAN Frame from Serial
 * 
 * @param dest Destination PC Frame to read to
 * @return true If a PC frame was read from serial
 * @return false If no PC frame was read from serial
 */
bool read_frame(PcFrame* dest);

/**
 * @brief Converts a PCFrame to a MCUCanFrame, which can be sent on
 * CAN
 * 
 * @param pc_frame Pointer to input PC frame
 * @param can_frame Pointer to output CAN frame
 */
void pc_frame_to_can_frame(PcFrame* pc_frame, MCUCanFrame* can_frame);

#endif