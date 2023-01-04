#include "pc_comm.h"

#include <string.h>

char buf[100];
void print_frame(char bus, MCUCanFrame* f) {
    int pos = 0;
    pos += sprintf(buf+pos, "%c%04X ", bus, f->id);
    for (uint8_t x = 0; x < f->dlc; x++) {
        pos += sprintf(buf+pos, "%02X",f->data[x]);
    }
    buf[pos] = '\n';
    Serial.write((const char*)buf, pos+1);
}

bool read_frame(PcFrame* dest) {
  if (Serial.available() >= sizeof(PcFrame)) {
    Serial.readBytes((uint8_t*)dest, sizeof(PcFrame));
    return true;
  }
  return false;
}

inline uint64_t frame_to_int(MCUCanFrame* f) {
  uint64_t tmp = 0;
  for(uint8_t i = 0; i < f->dlc; i++) {
      tmp |= (uint64_t)f->data[i] << (8*(7-i));
  }
  return tmp;
}

void pc_frame_to_can_frame(PcFrame* pc_frame, MCUCanFrame* can_frame) {
  can_frame->ack = 0;
  can_frame->rtr = 0;
  can_frame->id = pc_frame->id;
  can_frame->dlc = pc_frame->dlc;
  memcpy(can_frame->data, pc_frame->data, 8);
}