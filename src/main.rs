// Calculator

struct Calc<'a>{
    a: i32,
    b: i32,
    op: & 'a str
}

fn main(){
    let e = "10 + 2";
    let v = calc_parser(e);
    let a: Result<i32, _> = v[0].parse();
    let b: Result<i32, _> = v[2].parse();
    let (a,b) = match (a,b){
        (Ok(a),Ok(b)) => (a,b),
        _ => {println!("Inavlid Input");return;}
    };
    let statement = Calc{a,b,op:&v[1]};

    let ans = match statement.op{
        "+" => statement.a + statement.b,
        "-" => statement.a - statement.b,
        "*" => statement.a * statement.b,
        "/" => statement.a / statement.b,
        _ => {println!("Invalid Operator");0}
    };
    print!("The answer is {}",ans)
}

fn calc_parser(exp: &str) -> Vec<String>{
    let p: Vec<String> = exp.split_whitespace().map(|s| s.to_string()).collect();
    if p.len() != 3{
        panic!("Only use 2 numbers and a single operator")
    }
    p
}
