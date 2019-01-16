fn u8ify(b: bool) -> u8 {
    if b {1} else {0}
}

pub const DISCS: usize = 8;
pub const REGISTERS: usize = DISCS; //because they have to be accessable in the same number of bytes

pub struct Machine {
    mem: [[u8; 256]; DISCS], //four port memory: port 1 is RAM and input/output, port 2 is the removable disc, the rest is the hard drive
    registers: [u8; REGISTERS],
    prgcount: u8, //index on disc
    dsccount: u8, //current disc
    isactive: bool, //is it on or not?
}

fn run(program: Vec<String>) ->Result<(), &'static str> {
    let mut vm = Machine { mem: [[0; 256]; DISCS], registers: [0; REGISTERS], prgcount: 0, dsccount: 0, isactive: true, };
    fn parse_byte(x: &str) -> u8 {
        u8::from_str_radix(x, 2).unwrap()
    }
    fn make_len(mut v: Vec<u8>, l: usize) -> Vec<u8> { //code for formatting program
        if {v.len() == l} {v}
        else if {v.len() > l} {v[0..l].to_vec()}
        else {
            for i in 0..l-v.len() {
                v.push(0);
            }
            v
        }
    }
    fn format(bytes: Vec<u8>) -> [u8; 256] { //more code for formatting raw program
        let mut array = [0; 256];
        array.copy_from_slice(&bytes[..256]);
        array
    }
    for (i, v) in program.iter().enumerate() {
        vm.loaddsc(i as u8, format(make_len(v.split(" ").map(parse_byte).collect(), 256)))?;
        if {i > 2} {
            break
        }
    }
    while {vm.isactive} {
        let ins: [u8; 3] = [vm.mem(vm.dsccount, vm.prgcount)?, vm.mem(vm.dsccount, vm.prgcount+1)?, vm.mem(vm.dsccount, vm.prgcount+2)?];
        match ins[0] {
            0b00000000 => vm.end_END()?,
            0b00001000 ... 0b00001111 => vm.ram_SAV_DIV(ins[1], ins[2], ins[0]&0b00000111)?,
            0b00010000 ... 0b00010111 => vm.ram_SAV_DIR(ins[1], ins[2], ins[0]&0b00000111)?,
            0b00011000 ... 0b00011111 => vm.ram_SAV_DRV(ins[1], ins[2], ins[0]&0b00000111)?,
            0b00100000 ... 0b00100111 => vm.ram_SAV_DRR(ins[1], ins[2], ins[0]&0b00000111)?,
            0b00101000 ... 0b00101111 => vm.ram_SAV_RIV(ins[1], ins[2], ins[0]&0b00000111)?,
            0b00110000 ... 0b00110111 => vm.ram_SAV_RIR(ins[1], ins[2], ins[0]&0b00000111)?,
            0b00111000 ... 0b00111111 => vm.ram_SAV_RRV(ins[1], ins[2], ins[0]&0b00000111)?,
            0b01000000 ... 0b01000111 => vm.ram_SAV_RRR(ins[1], ins[2], ins[0]&0b00000111)?,
            0b01001000 ... 0b01001111 => vm.ram_LOAD_DIR(ins[1], ins[2], ins[0]&0b00000111)?,
            0b01010000 ... 0b01010111 => vm.ram_LOAD_DRR(ins[1], ins[2], ins[0]&0b00000111)?,
            0b01011000 ... 0b01011111 => vm.ram_LOAD_RIR(ins[1], ins[2], ins[0]&0b00000111)?,
            0b01100000 ... 0b01100111 => vm.ram_LOAD_DIR(ins[1], ins[2], ins[0]&0b00000111)?,
            0b01101000 => vm.alu_SUB_RV(ins[1], ins[2])?,
            0b01101001 => vm.alu_SUB_VR(ins[1], ins[2])?,
            0b01101010 => vm.alu_SUB_RR(ins[1], ins[2])?,
            0b01101011 => vm.alu_ADD_RV(ins[1], ins[2])?,
            0b01101100 => vm.alu_ADD_RR(ins[1], ins[2])?,
            0b01101101 => vm.alu_OR_RV(ins[1], ins[2])?,
            0b01101110 => vm.alu_OR_RR(ins[1], ins[2])?,
            0b01101111 => vm.alu_XOR_RV(ins[1], ins[2])?,
            0b01110000 => vm.alu_XOR_RR(ins[1], ins[2])?,
            0b01110001 => vm.alu_NOR_RV(ins[1], ins[2])?,
            0b01110010 => vm.alu_NOR_RR(ins[1], ins[2])?,
            0b01110011 => vm.alu_AND_RV(ins[1], ins[2])?,
            0b01110100 => vm.alu_AND_RR(ins[1], ins[2])?,
            0b01110101 => vm.cmp_GRT_RV(ins[1], ins[2])?,
            0b01110110 => vm.cmp_GRT_RR(ins[1], ins[2])?,
            0b01110111 => vm.cmp_LST_RV(ins[1], ins[2])?,
            0b01111000 => vm.cmp_LST_RR(ins[1], ins[2])?,
            0b01111001 => vm.cmp_GREQT_RV(ins[1], ins[2])?,
            0b01111010 => vm.cmp_GREQT_RR(ins[1], ins[2])?,
            0b01111011 => vm.cmp_LSEQT_RV(ins[1], ins[2])?,
            0b01111100 => vm.cmp_LSEQT_RR(ins[1], ins[2])?,
            0b01111101 => vm.cmp_EQL_RV(ins[1], ins[2])?,
            0b01111110 => vm.cmp_EQL_RR(ins[1], ins[2])?,
            0b01111111 => vm.cmp_NEQL_RV(ins[1], ins[2])?,
            0b10000000 => vm.cmp_NEQL_RR(ins[1], ins[2])?,
            0b10000001 => vm.goto_UNCON_DI(ins[1], ins[2])?,
            0b10000010 => vm.goto_UNCON_DR(ins[1], ins[2])?,
            0b10000011 => vm.goto_UNCON_RI(ins[1], ins[2])?,
            0b10000100 => vm.goto_UNCON_RR(ins[1], ins[2])?,
            0b10001000 ... 0b10001111 => vm.goto_ZRO_DRI(ins[1], ins[2], ins[0]&0b00000111)?,
            0b10010000 ... 0b10010111 => vm.goto_ZRO_DRR(ins[1], ins[2], ins[0]&0b00000111)?,
            0b10011000 ... 0b10011111 => vm.goto_ZRO_RRI(ins[1], ins[2], ins[0]&0b00000111)?,
            0b10100000 ... 0b10100111 => vm.goto_ZRO_RRR(ins[1], ins[2], ins[0]&0b00000111)?,
            0b10101000 ... 0b10001111 => vm.goto_NZRO_DRI(ins[1], ins[2], ins[0]&0b00000111)?,
            0b10110000 ... 0b10010111 => vm.goto_NZRO_DRR(ins[1], ins[2], ins[0]&0b00000111)?,
            0b10111000 ... 0b10011111 => vm.goto_NZRO_RRI(ins[1], ins[2], ins[0]&0b00000111)?,
            0b11000000 ... 0b10100111 => vm.goto_NZRO_RRR(ins[1], ins[2], ins[0]&0b00000111)?,
            _ => vm.end_END()?
        }
    }
    Ok(())
}

impl Machine {
    //instructions:
    //instruction naming scheme:
    //[type]_[INS]_[VAL USE]
    //type can be ram, alu
    //INS can be whatever
    //VAL USE is two letters in sucession showing how the parameters are used;
    //e.g. VR uses a program-specified value and a register;
    //letters can be V, R, I (index)
    //Current total of 12 embedded disc functions and 25 other functions
    fn ram_SAV_DIV(&mut self, i: u8, v: u8, d: u8) -> Result<(), &'static str> {
        self.sav(d, i, v)
    }

    fn ram_SAV_DIR(&mut self, i: u8, r: u8, d: u8) -> Result<(), &'static str> {
        self.sav(d, i, self.reg(r)?)
    }

    fn ram_SAV_DRV(&mut self, r1: u8, v: u8, d: u8) -> Result<(), &'static str> {
        self.sav(d, self.reg(r1)?, v)
    }

    fn ram_SAV_DRR(&mut self, r1: u8, r2: u8, d: u8) -> Result<(), &'static str> {
        self.sav(d, self.reg(r1)?, self.reg(r2)?)
    }

    fn ram_SAV_RIV(&mut self, i: u8, v: u8, r1: u8) -> Result<(), &'static str> {
        self.sav(self.reg(r1)?, i, v)
    }

    fn ram_SAV_RIR(&mut self, i: u8, r1: u8, r2: u8) -> Result<(), &'static str> {
        self.sav(self.reg(r2)?, i, self.reg(r1)?)
    }

    fn ram_SAV_RRV(&mut self, r1: u8, v: u8, r2: u8) -> Result<(), &'static str> {
        self.sav(self.reg(r2)?, self.reg(r1)?, v)
    }

    fn ram_SAV_RRR(&mut self, r1: u8, r2: u8, r3: u8) -> Result<(), &'static str> {
        self.sav(self.reg(r3)?, self.reg(r1)?, self.reg(r2)?)
    }

    fn ram_LOAD_DIR(&mut self, i: u8, r: u8, d: u8) -> Result<(), &'static str> {
        self.set(r, self.mem(d, i)?)
    }

    fn ram_LOAD_DRR(&mut self, r1: u8, r2: u8, d: u8) -> Result<(), &'static str> {
        self.set(r2, self.mem(d, r2)?)
    }

    fn ram_LOAD_RIR(&mut self, i: u8, r1: u8, r2: u8) -> Result<(), &'static str> {
        self.set(r1, self.mem(self.reg(r2)?, i)?)
    }

    fn ram_LOAD_RRR(&mut self, r1: u8, r2: u8, r3: u8) -> Result<(), &'static str> {
        self.set(r2, self.mem(self.reg(r3)?, r2)?)
    }

    fn alu_SUB_RV(&mut self, r: u8, v: u8) -> Result<(), &'static str> {
        self.set(0, self.reg(r)?-v)
    }

    fn alu_SUB_VR(&mut self, v: u8, r: u8) -> Result<(), &'static str> {
        self.set(0, v-self.reg(r)?)
    }

    fn alu_SUB_RR(&mut self, r1: u8, r2: u8) -> Result<(), &'static str> {
        self.set(0, self.reg(r1)?-self.reg(r2)?)
    }

    fn alu_ADD_RV(&mut self, r1: u8, v: u8) -> Result<(), &'static str> {
        self.set(0, self.reg(r1)?-v)
    }

    fn alu_ADD_RR(&mut self, r1: u8, r2: u8) -> Result<(), &'static str> {
        self.set(0, self.reg(r1)?+self.reg(r2)?)
    }

    fn alu_OR_RV(&mut self, r1: u8, v: u8) -> Result<(), &'static str> {
        self.set(0, self.reg(r1)?|v)
    }

    fn alu_OR_RR(&mut self, r1: u8, r2: u8) -> Result<(), &'static str> {
        self.set(0, self.reg(r1)?|self.reg(r2)?)
    }

    fn alu_XOR_RV(&mut self, r1: u8, v: u8) -> Result<(), &'static str> {
        self.set(0, self.reg(r1)?^v)
    }

    fn alu_XOR_RR(&mut self, r1: u8, r2: u8) -> Result<(), &'static str> {
        self.set(0, self.reg(r1)?^self.reg(r2)?)
    }

    fn alu_NOR_RV(&mut self, r1: u8, v: u8) -> Result<(), &'static str> {
        self.set(0, !(self.reg(r1)?|v))
    }

    fn alu_NOR_RR(&mut self, r1: u8, r2: u8) -> Result<(), &'static str> {
        self.set(0, !(self.reg(r1)?|self.reg(r2)?))
    }

    fn alu_AND_RV(&mut self, r1: u8, v: u8) -> Result<(), &'static str> {
        self.set(0, self.reg(r1)?&v)
    }

    fn alu_AND_RR(&mut self, r1: u8, r2: u8) -> Result<(), &'static str> {
        self.set(0, self.reg(r1)?&self.reg(r2)?)
    }

    fn cmp_GRT_RV(&mut self, r1: u8, v: u8) -> Result<(), &'static str> {
        self.set(0, u8ify(self.reg(r1)?>v))
    }

    fn cmp_GRT_RR(&mut self, r1: u8, r2: u8) -> Result<(), &'static str> {
        self.set(0, u8ify(self.reg(r1)?>self.reg(r2)?))
    }

    fn cmp_LST_RV(&mut self, r1: u8, v: u8) -> Result<(), &'static str> {
        self.set(0, u8ify(self.reg(r1)?<v))
    }

    fn cmp_LST_RR(&mut self, r1: u8, r2: u8) -> Result<(), &'static str> {
        self.set(0, u8ify(self.reg(r1)?<self.reg(r2)?))
    }

    fn cmp_GREQT_RV(&mut self, r1: u8, v: u8) -> Result<(), &'static str> {
        self.set(0, u8ify(self.reg(r1)?>=v))
    }

    fn cmp_GREQT_RR(&mut self, r1: u8, r2: u8) -> Result<(), &'static str> {
        self.set(0, u8ify(self.reg(r1)?>=self.reg(r2)?))
    }

    fn cmp_LSEQT_RV(&mut self, r1: u8, v: u8) -> Result<(), &'static str> {
        self.set(0, u8ify(self.reg(r1)?<=v))
    }

    fn cmp_LSEQT_RR(&mut self, r1: u8, r2: u8) -> Result<(), &'static str> {
        self.set(0, u8ify(self.reg(r1)?<=self.reg(r2)?))
    }

    fn cmp_EQL_RV(&mut self, r1: u8, v: u8) -> Result<(), &'static str> {
        self.set(0, u8ify(self.reg(r1)?==v))
    }

    fn cmp_EQL_RR(&mut self, r1: u8, r2: u8) -> Result<(), &'static str> {
        self.set(0, u8ify(self.reg(r1)?==self.reg(r2)?))
    }

    fn cmp_NEQL_RV(&mut self, r1: u8, v: u8) -> Result<(), &'static str> {
        self.set(0, u8ify(self.reg(r1)?!=v))
    }

    fn cmp_NEQL_RR(&mut self, r1: u8, r2: u8) -> Result<(), &'static str> {
        self.set(0, u8ify(self.reg(r1)?!=self.reg(r2)?))
    }

    fn goto_ZRO_DRI(&mut self, r1: u8, i: u8, d: u8) -> Result<(), &'static str> { //goto i if @r1 is 0
        if (self.reg(r1)?==0) {self.goto(i, d)?};
        Ok(())
    }

    fn goto_ZRO_DRR(&mut self, r1: u8, r2: u8, d: u8) -> Result<(), &'static str> { //goto @r2 if @r1 is 0
        if (self.reg(r1)?==0) {self.goto(self.reg(r2)?, d)?};
        Ok(())
    }

    fn goto_ZRO_RRI(&mut self, r1: u8, i: u8, r2: u8) -> Result<(), &'static str> { //goto i if @r1 is 0
        if (self.reg(r1)?==0) {self.goto(i, self.reg(r2)?)?};
        Ok(())
    }

    fn goto_ZRO_RRR(&mut self, r1: u8, r2: u8, r3: u8) -> Result<(), &'static str> { //goto @r2 if @r1 is 0
        if (self.reg(r1)?==0) {self.goto(self.reg(r2)?, self.reg(r3)?)?};
        Ok(())
    }

    fn goto_NZRO_DRI(&mut self, r1: u8, i: u8, d: u8) -> Result<(), &'static str> { //goto i if @r1 is 0
        if (self.reg(r1)?!=0) {self.goto(i, d)?};
        Ok(())
    }

    fn goto_NZRO_DRR(&mut self, r1: u8, r2: u8, d: u8) -> Result<(), &'static str> { //goto @r2 if @r1 is 0
        if (self.reg(r1)?!=0) {self.goto(self.reg(r2)?, d)?};
        Ok(())
    }

    fn goto_NZRO_RRI(&mut self, r1: u8, i: u8, r2: u8) -> Result<(), &'static str> { //goto i if @r1 is 0
        if (self.reg(r1)?!=0) {self.goto(i, self.reg(r2)?)?};
        Ok(())
    }

    fn goto_NZRO_RRR(&mut self, r1: u8, r2: u8, r3: u8) -> Result<(), &'static str> { //goto @r2 if @r1 is 0
        if (self.reg(r1)?!=0) {self.goto(self.reg(r3)?, self.reg(r2)?)?};
        Ok(())
    }

    fn goto_UNCON_DI(&mut self, i: u8, d: u8) -> Result<(), &'static str> { //goto i
        self.goto(i, d)
    }

    fn goto_UNCON_DR(&mut self, r: u8, d: u8) -> Result<(), &'static str> { //goto @r
        self.goto(self.reg(r)?, d)
    }

    fn goto_UNCON_RI(&mut self, i: u8, r: u8) -> Result<(), &'static str> { //goto i
        self.goto(i, self.reg(r)?)
    }

    fn goto_UNCON_RR(&mut self, r1: u8, r2: u8) -> Result<(), &'static str> { //goto @r
        self.goto(self.reg(r1)?, self.reg(r2)?)
    }

    fn end_END(&mut self) -> Result<(), &'static str> {
        self.isactive = false;
        Ok(())
    }

    //non-instructions
    fn mem(&self, d: u8, i: u8) -> Result<u8, &'static str> { //return val at index
        let d = self.getdsc(d)? as usize;
        if {d < DISCS} {
            Ok(self.mem[d][i as usize])
        } else {
            Err("Failed to set memory: Disc does not exist.")
        }
    }

    fn sav(&mut self, d: u8, i: u8, v: u8) -> Result<(), &'static str> { //save at index
        let d = self.getdsc(d)? as usize;
        if {d < DISCS} {
            self.mem[d][i as usize] = v;
            Ok(())
        } else {
            Err("Failed to set memory: Disc does not exist.")
        }
    }

    fn reg(&self, r: u8) -> Result<u8, &'static str> { //return val at register
        if {(r as usize) < REGISTERS} {
            Ok(self.registers[r as usize])
        } else {
            Err("Failed to get register: Register does not exist.")
        }
    }

    fn set(&mut self, r: u8, v: u8) -> Result<(), &'static str> { //set register
        if {(r as usize) < REGISTERS} {
            self.registers[r as usize] = v;
            Ok(())
        } else {
            Err("Failed to set register: Register does not exist.")
        }
    }

    fn pos(&mut self) -> Result<u8, &'static str> { //get current program pos (byte; not word)
        Ok(self.prgcount)
    }

    fn goto(&mut self, i: u8, d: u8) -> Result<(), &'static str> { //set program pos (byte; not word) & disc
        self.prgcount = i;
        self.dsccount = self.getdsc(d)?;
        Ok(())
    }

    fn getdsc(&self, d: u8) -> Result<u8, &'static str> { //get disc
        if {d==0} {Ok(self.dsccount)} else {Ok(d-1)}
    }

    fn loaddsc(&mut self, d: u8, f: [u8; 256]) -> Result<(), &'static str> {
        self.mem[self.getdsc(d)? as usize] = f;
        Ok(())
    }

}
