fn u8ify(b: bool) -> u8 {
    if b {1} else {0}
}

pub struct Machine {
    mem: [[u8; 256]; 4], //four port memory: port 1 is RAM and input/output, port 2 is the removable disc, the rest is the hard drive
    registers: [u8; 16],
    prgcount: u8, //index on disc
    dsccount: u8, //current disc
}

fn run_disc(program: String) {
    let mut vm = Machine { mem: [[0; 256]; 4], registers: [0; 16], prgcount: 0, dsccount: 0, };
    fn parseByte(x: &str) -> u8 {
        u8::from_str_radix(x, 2).unwrap()
    }
    let prg: Vec<u8> = program.split(" ").map(parseByte).collect();
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
    fn ram_SAV_IV(&mut self, i: u8, v: u8, d: u8) -> Result<(), &'static str> { //saves v to index i
        self.sav(d, i, v)
    }

    fn ram_SAV_IR(&mut self, i: u8, r: u8, d: u8) -> Result<(), &'static str> { //saves val at register r to index i
        self.sav(d, i, self.reg(r)?)
    }

    fn ram_SAV_RV(&mut self, r1: u8, v: u8, d: u8) -> Result<(), &'static str> { //saves v to index at register r1
        self.sav(d, self.reg(r1)?, v)
    }

    fn ram_SAV_RR(&mut self, r1: u8, r2: u8, d: u8) -> Result<(), &'static str> { //saves val at register r2 to index at register r1
        self.sav(d, self.reg(r1)?, self.reg(r2)?)
    }

    fn ram_LOAD_IR(&mut self, i: u8, r: u8, d: u8) -> Result<(), &'static str> { //loads a val at an index to a register
        self.set(r, self.mem(d, i)?)
    }

    fn ram_LOAD_RR(&mut self, r1: u8, r2: u8, d: u8) -> Result<(), &'static str> { //loads a val at an index to a register
        self.set(r2, self.mem(d, r2)?)
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

    fn goto_ZRO_RI(&mut self, r1: u8, i: u8, d: u8) -> Result<(), &'static str> { //goto i if @r1 is 0
        if (self.reg(r1)?==0) {self.goto(i)?};
        Ok(())
    }

    fn goto_ZRO_RR(&mut self, r1: u8, r2: u8, d: u8) -> Result<(), &'static str> { //goto @r2 if @r1 is 0
        if (self.reg(r1)?==0) {self.goto(self.reg(r2)?)?};
        Ok(())
    }

    fn goto_NZRO_RI(&mut self, r1: u8, i: u8, d: u8) -> Result<(), &'static str> { //goto i if @r1 is 0
        if (self.reg(r1)?!=0) {self.goto(i)?};
        Ok(())
    }

    fn goto_NZRO_RR(&mut self, r1: u8, r2: u8, d: u8) -> Result<(), &'static str> { //goto @r2 if @r1 is 0
        if (self.reg(r1)?!=0) {self.goto(self.reg(r2)?)?};
        Ok(())
    }

    fn goto_UNCON_I(&mut self, i: u8, d: u8) -> Result<(), &'static str> { //goto i
        self.goto(i)
    }

    fn goto_UNCON_R(&mut self, r: u8, d: u8) -> Result<(), &'static str> { //goto @r
        self.goto(self.reg(r)?)
    }

    //non-instructions
    fn mem(&self, d: u8, i: u8) -> Result<u8, &'static str> { //return val at index
        let d = self.getdsc(d)?;
        if {d < 4} {
            Ok(self.mem[d as usize][i as usize])
        } else {
            Err("Failed to set memory: Disc does not exist.")
        }
    }

    fn sav(&mut self, d: u8, i: u8, v: u8) -> Result<(), &'static str> { //save at index
        let d = self.getdsc(d)?;
        if {d < 4} {
            self.mem[d as usize][i as usize] = v;
            Ok(())
        } else {
            Err("Failed to set memory: Disc does not exist.")
        }
    }

    fn reg(&self, r: u8) -> Result<u8, &'static str> { //return val at register
        if {r < 16} {
            Ok(self.registers[r as usize])
        } else {
            Err("Failed to get register: Register does not exist.")
        }
    }

    fn set(&mut self, r: u8, v: u8) -> Result<(), &'static str> { //set register
        if {r < 16} {
            self.registers[r as usize] = v;
            Ok(())
        } else {
            Err("Failed to set register: Register does not exist.")
        }
    }

    fn pos(&mut self) -> Result<u8, &'static str> { //get current program pos (byte; not word)
        Ok(self.prgcount)
    }

    fn goto(&mut self, i: u8) -> Result<(), &'static str> { //set program pos (byte; not word)
        self.prgcount = i;
        Ok(())
    }

    fn godsc(&mut self, v: u8) -> Result<(), &'static str> { //set program disc
        self.dsccount = self.getdsc(v)?;
        Ok(())
    }

    fn getdsc(&self, d: u8) -> Result<u8, &'static str> { //get disc
        if {d==0} {Ok(self.dsccount)} else {Ok(d-1)}
    }

}
