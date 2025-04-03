use btel::{get_btel_vars, set_btel_vars};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let mut vars = get_btel_vars(args);
    if let Some(cmd_args) = vars.args.to_owned() {
        match cmd_args[0].as_str() {
            "init" |"i"=> init(&mut vars,cmd_args),
            "update" |"u"=> update(&mut vars,cmd_args),
            "finalize" | "f"=> finalize(&mut vars),
            cmd => vars.line_name = String::from(format!("unknown command: \"{}\"",cmd))
        }
    } else {
        vars.line_name = String::from("missing command")
    }
    set_btel_vars(vars);
}
fn init(vars: &mut btel::BtelVars,args: Vec<String>) {
    if args.len() == 1 {
        vars.line_name = String::from("cannot create empty table");
        return;
    }
    vars.input[0] = format!("\"{}\"",get_true_args(&args).join("\"    \""));
}
fn update(vars: &mut btel::BtelVars,args: Vec<String>) {
    let mut table = get_table(vars);
    let args = get_true_args(&args);
    if table[0].len() != args.len() {
        vars.line_name = "New Fields must have same quantaty as initial field".to_string();
        return;
    }
    table.push(args);
    vars.input = Vec::new();
    for row in table{
        vars.input.push(format!("\"{}\"",row.join("\"    \"")))
    }
}
fn get_true_args(args: &Vec<String>) -> Vec<String>{
    let args = &args[1..].join(" ");
    let mut i: usize = 0;
    let args = args.split("\"").collect::<Vec<&str>>();
    let args = args.iter().filter(|_| {i += 1;i.rem_euclid(2) == 0}).collect::<Vec<&&str>>();
    args.iter().map(|f| f.to_string()).collect()
}
fn get_table(vars: &mut btel::BtelVars) -> Vec<Vec<String>> {
    let mut res = Vec::new();
    for line in &vars.input {
        let mut sub_res = vec![String::new()];
        let mut ptr = 0;
        for c in line.chars() {
            match c {
                '"' => {sub_res.push(String::new());ptr += 1;},
                x => sub_res[ptr] += &x.to_string(),
            }
        }
        ptr = 0;
        sub_res = sub_res.iter().filter(|_|{ptr += 1;ptr.rem_euclid(2) == 0}).map(|f|f.to_string()).collect();
        res.push(sub_res);
    }
    res
}
fn finalize(vars: &mut btel::BtelVars) {
    let table = get_table(vars);
    vars.input = vec![String::new();table.len()];
    let mut longest:Vec<usize> = Vec::new();
    for i in 0..table[0].len() {
        longest.push(0);
        for j in 0..table.len() {
            longest[i] = std::cmp::max(longest[i], table[j][i].len())
        }
    }
    for i in 0..table[0].len() {
        for j in 0..table.len() {
            vars.input[j] += &table[j][i];
            let mut length = table[j][i].len();
            while length < longest[i] + 4{
                length += 1;
                vars.input[j] += " ";
            }
        }
    }
}