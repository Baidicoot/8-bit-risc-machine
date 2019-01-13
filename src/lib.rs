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
}

fn run_disc(program: Vec<String>) {
    let mut vm = Machine { mem: [[0; 256]; DISCS], registers: [0; REGISTERS], prgcount: 0, dsccount: 0, };
    fn parse_byte(x: &str) -> u8 {
        u8::from_str_radix(x, 2).unwrap()
    }
    fn make_len(mut v: Vec<u8>, l: usize) -> Vec<u8> {
        if {v.len() == l} {v}
        else if {v.len() > l} {v[0..l].to_vec()}
        else {
            for i in 0..l-v.len() {
                v.push(0);
            }
            v
        }
    }
    fn format(bytes: Vec<u8>) -> [u8; 256] {
        let mut array = [0; 256];
        array.copy_from_slice(&bytes[..256]);
        array
    }
    fn load_disc(program: String, mut vm: Machine, dsc: u8) -> Result<(), &'static str> {
        if {dsc as usize >= DISCS} {
            return Err("Failed to set memory: Disc does not exist.");
        }
         vm.mem[dsc as usize] = format(make_len(program.split(" ").map(parse_byte).collect(), 256));
         Ok(())
    }
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

}
