#
# Converts the can_data.txt files into C++ headers for this project!
#
# This program takes 2 arguments:
# 1. Input can_data.txt file
# 2. Output directory for header files
# 3. Optional - Global #ifdef guard for file

import os
import sys


input_file=open(sys.argv[1], 'r')
output_dir=sys.argv[2]

output_guard = False
global_guard=""
if len(sys.argv) > 3:
    global_guard=sys.argv[3]
    output_guard = True

def clear_bit(mask, bit):
    return mask & ~(1<<bit)

def remove_useless_from_line(l: str) -> str:
    return l.replace("\t", "").replace("\n", "")

class EnumEntry:
    def __init__(self, name: str, raw: int, desc: str):
        self.name = name
        self.raw = raw
        self.desc = desc

class Signal:
    def __init__(self, name: str, desc: str, length: int, offset: int, unit: str):
        self.name = name
        self.desc = desc
        self.length = length
        self.offset = offset
        self.is_iso_tp = False
        self.is_number = False
        self.is_bool = False
        self.is_char = False
        self.number_data = (0.0, 0.0) # Multiplier, offset
        self.is_enum = True
        self.unit = unit
        self.enum_table = []

    def get_return_data_type(self, frame_name: str) -> str:
        if self.is_number:
            if self.length <= 8: # 8 bit wide data
                return "u8"
            elif self.length <= 16: # 16 bit wide data
                return "u16"
            elif self.length <= 32: # 32 bit wide data
                return "u32"
            else:
                return "u64" # 64 bit wide data
        elif self.is_bool: # Boolean data type
            return "bool"
        elif self.is_enum: # Enum data type
            return "{}_{}{}".format(frame_name, self.name, global_guard)
        elif self.is_char:
            return "char"
        else:
            return"" # ??

    def get_setter_and_getter(self, frame_name: str) -> str:
        mask = 0xFFFFFFFFFFFFFFFF

        start_mask = 63-self.offset
        for bit in range(0,self.length):
            mask = clear_bit(mask, start_mask-bit)

        f_mask = 0x0
        for bit in range(0,self.length):
            f_mask = (f_mask | 0x01 << bit)

        conv_to = ""
        conv_from=""
        unit_str=""
        if self.unit:
            unit_str=" (Unit: {})".format(self.unit)
        if self.is_number:

            conv_to = ". Conversion formula (To raw from real): y=(x{1:+})/{0:.2f}".format(self.number_data[0], self.number_data[1]*-1)
            conv_from = ". Conversion formula (To real from raw): y=({0:.2f}x){1:+}".format(self.number_data[0], self.number_data[1])

        if self.is_enum:
            return """
    /// Sets {0}{6}{8}

    pub fn set_{1}(&mut self, value: {2}){{ self.0 = (self.0 & 0x{5:{fill}16x}) | ((value as u64) & 0x{4:x}) << {3}; }}

    /// Gets {0}{7}{8}
    pub fn get_{1}(&self) -> std::result::Result<{2}, ()> {{ return {2}::try_from((self.0 >> {3} & 0x{4:x}) as u8) }}
        """.format(self.desc, self.name, self.get_return_data_type(frame_name), 64-self.length-self.offset, f_mask, mask, conv_to, conv_from, unit_str, fill='0')
        elif self.is_bool:
            return """
    /// Sets {0}{6}{8}

    pub fn set_{1}(&mut self, value: {2}){{ self.0 = (self.0 & 0x{5:{fill}16x}) | ((value as u64) & 0x{4:x}) << {3}; }}

    /// Gets {0}{7}{8}
    pub fn get_{1}(&self) -> {2} {{ (self.0 >> {3} & 0x{4:x}) != 0 }}
        """.format(self.desc, self.name, self.get_return_data_type(frame_name), 64-self.length-self.offset, f_mask, mask, conv_to, conv_from, unit_str, fill='0')
        else:
            return """
    /// Sets {0}{6}{8}

    pub fn set_{1}(&mut self, value: {2}){{ self.0 = (self.0 & 0x{5:{fill}16x}) | ((value as u64) & 0x{4:x}) << {3}; }}

    /// Gets {0}{7}{8}
    pub fn get_{1}(&self) -> {2} {{ (self.0 >> {3} & 0x{4:x}) as {2} }}
        """.format(self.desc, self.name, self.get_return_data_type(frame_name), 64-self.length-self.offset, f_mask, mask, conv_to, conv_from, unit_str, fill='0')

    def add_enum(self, e: EnumEntry):
        self.enum_table.append(e)

    def add_data_str(self, dt: str):
        self.is_bool = False
        self.is_enum = False
        self.is_iso_tp = False
        self.is_number = False

        if dt.strip() == "ISO_TP":
            self.is_iso_tp = True
        elif dt.strip() == "CHAR":
            self.is_char = True
        elif dt.strip() == "BOOL":
            self.is_bool = True
        elif "ENUM" in dt:
            self.is_enum = True
        elif "NUMBER" in dt:
            self.is_number = True
            multiplier = float(dt.split("_MULTIPLIER_: ")[1].split(",")[0])
            offset = float(dt.split("_OFFSET_: ")[1].split(")")[0])
            self.number_data = (multiplier, offset)
        else:
            print(dt)

class Frame:
    def __init__(self, name: str, id: int):
        self.name = name
        self.can_id = id
        self.signals=[]
    
    def add_signal(self, s: Signal):
        self.signals.append(s)

class ECU:
    def __init__(self, name: str):
        self.name = name
        self.frames=[]

    def add_frame(self, f: Frame):
        self.frames.append(f)

    def filter_frames(self):
        cloned = self.frames.copy()
        self.frames.clear()
        for x in cloned:
            if len(x.signals) > 1:
                self.frames.append(x)

    def make_output_str(self) -> str:
        self.filter_frames()
        # Create output header string
        tmp = """
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
/**
* AUTOGENERATED BY convert.py
* DO NOT EDIT THIS FILE!
*
* IF MODIFICATIONS NEED TO BE MADE, MODIFY can_data.txt!
*
* CAN Defintiion for ECU '{0}'
*/
    """.format(self.name)

        for f in self.frames:
            tmp += "\npub const {}{}_CAN_ID: u16 = 0x{:04X};".format(f.name.strip().removesuffix("h"), global_guard, f.can_id)

        tmp += "\n\n"
        # Now iterate over all enums of the ECU
        for x in self.frames:
            for s in x.signals:
                if s.is_enum:
                    tmp += "/// {}".format(s.desc)
                    tmp += "\n#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]"
                    tmp += "\n#[repr(C)]"
                    tmp += "\npub enum {}_{}{} {{".format(x.name, s.name, global_guard)
                    for e in s.enum_table:
                        tmp += "\n\t{} = {}, // {}".format(e.name.replace(" ", "_").upper(), e.raw, e.desc)
                    tmp += "\n}\n"
                    tmp += "\nimpl TryFrom<u8> for {}_{} {{".format(x.name, s.name)
                    tmp += "\n\ttype Error = ();"
                    tmp += "\n\tfn try_from(value: u8) -> Result<Self, Self::Error> {"
                    tmp += "\n\t\tmatch value {"
                    for e in s.enum_table:
                        tmp += "\n\t\t\t{} => Ok(Self::{}),".format(e.raw, e.name.replace(" ", "_").upper())
                    tmp += "\n\t\t\t_ => Err(())"
                    tmp +="\n\t\t}"
                    tmp +="\n\t}"
                    tmp += "\n}\n"

        # Now create our type unions for CAN Frames!
        for x in self.frames:
            struct_name = x.name.strip().removesuffix("h")
            tmp += "\npub struct {}(u64);".format(struct_name) # Struct name
            tmp += "\n\nimpl {} {{".format(struct_name)
            tmp += "\n\n\t/// Gets CAN ID of {}{}".format(struct_name, global_guard)
            tmp += "\n\tpub fn get_canid() -> u16 {{ {}{}_CAN_ID }}".format(struct_name, global_guard)
            
            # Setters and getters!
            for s in x.signals:
                tmp += s.get_setter_and_getter(x.name)

            tmp += "\n}"

        
        # Now magic to create the class ;)

        #num_frames = len(self.frames)
#
        #tmp += "\n\nclass ECU_{} {{".format(self.name)
        #tmp += "\n\tpublic:"
        # Setter function to import supported frames to the ECU
        #tmp += """
        #/**
        # * @brief Imports the CAN frame given the CAN ID, CAN Contents, and current timestamp
        # *
        # * Returns true if the frame was imported successfully, and false if import failed (Due to non-matching CAN ID).
        # *
        # * NOTE: The endianness of the value cannot be guaranteed. It is up to the caller to correct the byte order!
        # */
        #bool import_frames(uint64_t value, uint32_t can_id, uint64_t timestamp_now) {
        #    switch(can_id) {"""
        #for idx, frame in enumerate(self.frames):
        #    tmp += """
        #        case {0}{2}_CAN_ID:
        #            LAST_FRAME_TIMES[{1}] = timestamp_now;
        #            FRAME_DATA[{1}] = value;
        #            return true;""".format(frame.name.strip().removesuffix("h"), idx, global_guard)
        #tmp += """
        #        default:
        #            return false;
        #    }
        #}
        #"""
        # Now do getters!
        #for idx, frame in enumerate(self.frames):
        #    tmp += """
        #/** Sets data in pointer to {0}
        #  * 
        #  * If this function returns false, then the CAN Frame is invalid or has not been seen
        #  * on the CANBUS network yet. Meaning it's data cannot be used.
        #  *
        #  * If the function returns true, then the pointer to 'dest' has been updated with the new CAN data
        #  */
        #bool get_{0}(uint64_t now, uint64_t max_expire_time, {0}{2}* dest) const {{
        #    if (LAST_FRAME_TIMES[{1}] == 0 || dest == nullptr) {{ // CAN Frame has not been seen on bus yet / NULL pointer
        #        return false;
        #    }} else if (now > LAST_FRAME_TIMES[{1}] && now - LAST_FRAME_TIMES[{1}] > max_expire_time) {{ // CAN Frame has not refreshed in valid interval
        #        return false;
        #    }} else {{ // CAN Frame is valid! return it
        #        dest->raw = FRAME_DATA[{1}];
        #        return true;
        #    }}
        #}}
        #    """.format(frame.name.strip().removesuffix("h"), idx, global_guard)
        #tmp += "\n\tprivate:"
        #tmp += "\n\t\tuint64_t FRAME_DATA[{0}];".format(num_frames)
        #tmp += "\n\t\tuint64_t LAST_FRAME_TIMES[{0}];".format(num_frames)
        #tmp += "\n};"

        return tmp


current_ecu: ECU = None
current_frame: Frame = None
current_signal: Signal = None

ecus=[]

for line in input_file:
    print(line)
    l = remove_useless_from_line(line)
    if not l.startswith("#"): # Ignore comments
        if l.startswith("ECU "):
            ecu = l.split("ECU ")[1].strip()
            ecus.append(ecu)
            if getattr(current_ecu, 'name', 'nan') != ecu and current_ecu != None:
                # Check if frame / signal is none
                if current_signal and current_frame:
                    if len(current_frame.signals) > 1: # Ignore ISO-TP endpoints
                        current_frame.add_signal(current_signal)
                    current_signal = None
                if current_frame and current_ecu:
                    current_ecu.add_frame(current_frame)
                    current_frame = None
                open("{}/{}.rs".format(output_dir, current_ecu.name), 'w').write(current_ecu.make_output_str()) # Write tmp output str to file
            current_ecu = ECU(ecu)
        elif l.startswith("FRAME"):
            frame_name = l.split("FRAME ")[1].split("(")[0].strip()
            frame_id = int(l.split("(")[1].split(")")[0], 0)
            if current_signal and current_frame:
                current_frame.add_signal(current_signal)
            if current_frame:
                if len(current_frame.signals) > 1: # Ignore ISO-TP endpoints
                    # Its a new frame
                    current_ecu.add_frame(current_frame)
            current_frame = Frame(frame_name, frame_id)
            current_signal = None
        elif l.startswith("SIGNAL"):
            if current_signal:
                current_frame.add_signal(current_signal)
            signal_name = l.split("SIGNAL ")[1].split(", ")[0].strip()
            signal_offset = int(l.split("OFFSET: ")[1].split(",")[0], 10)
            signal_length = int(l.split("LEN: ")[1].split(",")[0], 10)
            signal_desc = l.split("DESC: ")[1].split(", DATA TYPE")[0].strip()
            try:
                signal_dt = l.split(", DATA TYPE ")[1]
            except Exception as e:
                signal_dt = "RAW"
            unit=""
            if "UNIT: " in l:
                unit = l.split("UNIT: ")[1]
            current_signal = Signal(signal_name, signal_desc, signal_length, signal_offset, unit)
            current_signal.add_data_str(signal_dt)
        elif l.startswith("ENUM"):
            if current_signal:
                enum_name = l.split("ENUM ")[1].split(", ")[0].strip()
                enum_raw = int(l.split("RAW: ")[1].split(", ")[0])
                enum_desc = l.split("DESC: ")[1].strip()
                current_signal.add_enum(EnumEntry(enum_name, enum_raw, enum_desc))


if current_signal and current_frame:
    current_frame.add_signal(current_signal)
    current_signal = None
if current_frame and current_ecu:
    current_ecu.add_frame(current_frame)
    current_frame = None
# Write last ECU in DB
open("{}/{}.rs".format(output_dir, current_ecu.name), 'w').write(current_ecu.make_output_str()) # Write tmp output str to file

mod_str=""

for x in ecus:
    mod_str += "\nmod {};".format(x)
    mod_str += "\npub use {}::*;".format(x)

open("{}/mod.rs".format(output_dir), 'w').write(mod_str)