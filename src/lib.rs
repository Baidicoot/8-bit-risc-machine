fn u8ify(b: bool) -> u8 {
    if b {255} else {0}
}

pub struct Machine {
    mem: [[u8; 256]; 4], //four port memory: port 1 is RAM and input/output, port 2 is the removable disc, the rest is the hard drive
    reg: [u8; 16],
    prgcount: u8, //index on disc
    prgdisc: u8, //port ('disc') currently running from
}

fn run_disc(program: String) {
    let mut vm = Machine { mem: [[0; 256]; 4], reg: [0; 16], prgcount: 0, prgdisc: 0, };
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
    fn esc_INTRPT_CV(&mut self, c: u8, v: u8) -> Result<(), &'static str> {
        unimplemented!()
    }

    fn esc_INTRPT_CR(&mut self, c: u8, r: u8) -> Result<(), &'static str> {
        self.esc_INTRPT_CR(c, self.reg(r)?)
    }

    fn ram_SAV_IV(&mut self, i: u8, v: u8) -> Result<(), &'static str> {
        self.sav(i, v)
    }

    fn ram_SAV_IR(&mut self, i: u8, r: u8) -> Result<(), &'static str> {
        self.sav(i, self.reg(r)?)
    }

    fn ram_LOAD_IR(&mut self, i: u8, r: u8) -> Result<(), &'static str> {
        self.set(r, self.mem(i)?)
    }

    fn ram_SET_RV(&mut self, rin: u8, v: u8) -> Result<(), &'static str> {
        self.set(rin, v)
    }

    fn ram_SET_RR(&mut self, rin: u8, rout: u8) -> Result<(), &'static str> {
        self.set(rin, self.reg(rout)?)
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

    fn goto_ZRO_RI(&mut self, r1: u8, i: u8) -> Result<(), &'static str> {
        if (self.reg(r1)?==0) {self.goto(i)?};
        Ok(())
    }

    fn goto_ZRO_RR(&mut self, r1: u8, r2: u8) -> Result<(), &'static str> {
        if (self.reg(r1)?==0) {self.goto(self.reg(r2)?)?};
        Ok(())
    }

    fn goto_NZRO_RI(&mut self, r1: u8, i: u8) -> Result<(), &'static str> {
        if (self.reg(r1)?!=0) {self.goto(i)?};
        Ok(())
    }

    fn goto_NZRO_RR(&mut self, r1: u8, r2: u8) -> Result<(), &'static str> {
        if (self.reg(r1)?!=0) {self.goto(self.reg(r2)?)?};
        Ok(())
    }

    fn goto_UNCON_IV(&mut self, i: u8, v: u8) -> Result<(), &'static str> {
        self.goto(i)
    }

    fn goto_UNCON_RV(&mut self, r1: u8, v: u8) -> Result<(), &'static str> {
        self.goto(self.reg(r1)?)
    }

    //non-instructions
    fn mem(&self, i: u8) -> Result<u8, &'static str> { //return val at index
        unimplemented!()
    }

    fn sav(&mut self, i: u8, v: u8) -> Result<(), &'static str> { //save at index
        unimplemented!()
    }

    fn reg(&self, r: u8) -> Result<u8, &'static str> { //return val at register
        unimplemented!()
    }

    fn set(&mut self, r: u8, v: u8) -> Result<(), &'static str> { //set register
        unimplemented!()
    }

    fn pos(&mut self) -> Result<u8, &'static str> { //get current program pos (byte; not word)
        unimplemented!()
    }

    fn goto(&mut self, i: u8) -> Result<(), &'static str> { //set program pos (byte; not word)
        unimplemented!()
    }

    fn disc(&mut self) -> Result<u8, &'static str> { //get current disc
        unimplemented!()
    }

    fn gocd(&mut self, i: u8) -> Result<(), &'static str> { //set program disc
        unimplemented!()
    }

}
