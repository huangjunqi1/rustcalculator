#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Complex{
    pub r: f64,
    pub i: f64
}
fn abs(x:f64) -> f64
{
    if x<0.0 {return -x;}
    x
}
impl Complex{
    pub fn add(a:Self,b:Self) -> Self{
        Self{r:(a.r+b.r),i:(a.i+b.i)}
    }
    pub fn sub(a:Self,b:Self) -> Self{
        Self{r:(a.r-b.r),i:(a.i-b.i)}
    }
    pub fn opsite(a:Self) ->Self{
        Self{r:-a.r,i:-a.i}
    }
    pub fn mul(a:Self,b:Self) -> Self{
        Self{r:(a.r*b.r-a.i*b.i),i:(a.r*b.i+a.i*b.r)}
    }
    pub fn divv(a:Self,b:f64) -> Result<Self,String>{
        if abs(b) < 1e-9 {return Err("div 0".to_string());}
        return Ok(Complex{r:a.r/b,i:a.i/b});
    }
    pub fn div(a:Self,b:Self) -> Result<Self,String>{
        let bb: Self = Self::mul(Self{r:a.r,i:a.i},Self{r:b.r,i:-b.i});
        let x: f64 = sqr(b.r)+sqr(b.i);
        return Self::divv(bb,x);
    }
    pub fn ln(a:Self) -> Result<Self,String>{
        if a.i != 0.0 {return Err("只有纯实数才能取对数".to_string());}
        if a.r <= 0.0 {return Err("只有正数才能取对数".to_string());}
        return Ok(Complex { r: (a.r.ln()), i: (0.0) });
    }
    pub fn exp(a:Self) -> Self{  //e^a
        Self::mul(Complex{r:a.r.exp(),i:0.0},Complex{r:a.i.cos(),i:a.i.sin()})
    }
    pub fn pow(a:Self,b:Self) -> Result<Self,String>{
        if (a.i != 0.0) || (a.r <= 0.0) {return Err("底数需要是正数".to_string());}
        let x:f64 = a.r.ln();
        let bb:Self = Self::mul(b,Self{r:x,i:0.0});
        return Ok(Self::exp(bb));
    }
    pub fn sin(a:Self) -> Self{
        let l:Self = Self::exp(Complex{r:-a.i,i:a.r});
        let r:Self = Self::exp(Complex{r:a.i,i:-a.r});
        return Self::divv(Self::sub(l,r),2.0).unwrap();
    }
    pub fn cos(a:Self) -> Self{
        let l:Self = Self::exp(Complex{r:-a.i,i:a.r});
        let r:Self = Self::exp(Complex{r:a.i,i:-a.r});
        return Self::divv(Self::add(l,r),2.0).unwrap();
    }
    pub fn bopcalc(c:char,a:Self,b:Self)->Self{
        if c=='+' {return Self::add(a,b);}
        if c=='-' {return Self::sub(a,b);}
        if c=='*' {return Self::mul(a,b);}
        if c=='^' {return Self::pow(a,b).unwrap();}
        Self::div(a,b).unwrap()
    }
    pub fn sopcalc(c:char,a:Self)->Self{
        if c=='-' {return Self::opsite(a);}
        if c=='S' {return Self::sin(a);}
        if c=='C' {return Self::cos(a);}
        if c=='T' {return Self::div(Self::sin(a),Self::cos(a)).unwrap();}
        Self::ln(a).unwrap()
    }
    pub fn write(&self){
        println!("r:{},i:{}",self.r,self.i);
    }
}
fn sqr(x: f64) -> f64{
    x*x
}

pub enum Node {
    Ident(Complex),
    Dop(char, Box<Node>, Box<Node>),
    Sop(char, Box<Node>)
}

impl Node{
    pub fn calc(self) -> Complex{
        match self{
            Self::Ident(v) => v,
            Self::Dop(c, x,y) => Complex::bopcalc(c,x.calc(),y.calc()),
            Self::Sop(c,x) => Complex::sopcalc(c,x.calc())
        }
    }
}