extern crate peg;
use calculator::{Complex,Node};
peg::parser!( grammar arithmetic() for str {
    rule number() -> f64
        =n:$(['.'|'0'..='9']+) { n.parse::<f64>().unwrap() }
    rule pi() = _ "pi" _
    rule e() = _ "e" _
    rule i() = _ "i" _
    rule _ = [' '|'\n']*
    pub(crate) rule calculate() -> Node = precedence!{
        pi() {Node::Ident(Complex{r:std::f64::consts::PI,i:0.0})}
        e() {Node::Ident(Complex{r:std::f64::consts::E,i:0.0})}
        i() {Node::Ident(Complex{r:0.0,i:1.0})}
        --
        x:(@) _ "+" _ y:@ { Node::Dop('+',Box::new(x),Box::new(y)) }
        x:(@) _ "-" _ y:@ { Node::Dop('-',Box::new(x),Box::new(y)) }
              _ "-" _ v:@ { Node::Sop('-',Box::new(v))}
              _ "sin" _ v:@ { Node::Sop('S',Box::new(v))}
              _ "ln" _ v:@ { Node::Sop('L',Box::new(v))}
              _ "tan" _ v:@ { Node::Sop('T',Box::new(v))}
              _ "cos" _ v:@ { Node::Sop('C',Box::new(v))}
        --
        x:(@) _ "*" _ y:@ { Node::Dop('*',Box::new(x),Box::new(y)) }
        x:(@) _ "/" _ y:@ { Node::Dop('/',Box::new(x),Box::new(y)) }
        --
        x:@   _ "^" _ y:(@) { Node::Dop('^',Box::new(x),Box::new(y)) }
        --
        "(" _ v:calculate() _ ")" { v }
        n:number() _ {Node::Ident(Complex { r: n, i: 0.0 })}
    }
});


peg::parser!( grammar arithmetic1(xx:f64) for str {
    rule number() -> f64
        = n:$(['.'|'0'..='9'] +) { n.parse::<f64>().unwrap() }
    rule pi() = _ "pi" _
    rule e() = _ "e" _
    rule x() = _ "x" _
    rule _ = [' '|'\n']*
    pub(crate) rule calculate() -> f64 = precedence!{
        pi() {std::f64::consts::PI}
        e() {std::f64::consts::E}
        x() {xx}
        --
        x:(@) _ "+" _ y:@ { x+y }
        x:(@) _ "-" _ y:@ {x-y }
              _ "-" _ v:@ { -v }
              _ "ln" _ v:@ {v.ln()}
              _ "cos" _ v: @ {v.cos()}
              _ "sin" _ v: @ {v.sin()}
              _ "tan" _ v: @ {v.tan()}
        --
        x:(@) _ "*" _ y:@ { x*y }
        x:(@) _ "/" _ y:@ { x/y }
        --
        x:(@) _ "^" _ y:@ {x.powf(y)}
        --
        "(" _ v:calculate() _ ")" { v }
        n:number() _ {n}
    }
});


fn main() {
    println!("please choose a mode.0 for calculator and 1 for integral.");
    let mut ipt = String::new();
    let len = std::io::stdin().read_line(&mut ipt).expect("Failed to read the mode");
    let mode = (&ipt[0..len-2]).parse();
    if (mode != Ok(0) ) && (mode != Ok(1))
    {
        println!("Error. just 0 or 1");
        return;
    }
    if mode == Ok(1) //integral
    {
        let mut ab:String = String::new();
        std::io::stdin().read_line(&mut ab).expect("Failed to read line");
        let mut lr = ab.split_whitespace();
        let l = lr.next().unwrap().parse::<f64>().unwrap();
        let r = lr.next().unwrap().parse::<f64>().unwrap();
        let mut ipt = String::new();
        let len = std::io::stdin().read_line(&mut ipt).expect("Failed to read line");
        let mut ans:f64 = arithmetic1::calculate(&ipt[0..len-2],l).unwrap();
        ans += arithmetic1::calculate(&ipt[0..len-2],r).unwrap();
        let n = 20000;
        for i in 1..=n{
            let x:f64 = (l*(i as f64)+r*((n-i)as f64))/(n as f64);
            let tmp:f64 = arithmetic1::calculate(&ipt[0..len-2],x).unwrap();
            if i % 2 == 0 {ans+= 4.0*tmp}
            else {ans+=2.0*tmp};
        }
        println!("{}",ans * (r-l) / (3.0*(n as f64)));
    }
    else {
        let mut ipt = String::new();
        let len = std::io::stdin().read_line(&mut ipt).expect("Failed to read line");
        let nodes: Node = arithmetic::calculate(&ipt[0..len-2]).unwrap();
        nodes.calc().write();
    }

}