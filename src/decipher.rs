use nom::types::CompleteStr;
use nom::IResult;
use std::error::Error;

named!(parse_sig<CompleteStr, CompleteStr>, 
    do_parse!(
        take_until_and_consume!(r#""signature","#) >>
        f: take_until!("(") >>
        delimited!( tag!("("), take_until!(")"), tag!(")"))>>
        (f)
    )
);

fn definition<'a>(
    input: CompleteStr<'a>,
    name: &'a str,
) -> IResult<CompleteStr<'a>, CompleteStr<'a>> {
    let mut exp = String::new();
    exp.push_str("(?m)");
    exp.push_str(r"var\s?");
    exp.push_str(name);
    exp.push_str(r"\s?=\s?\{(\w+:function(.*)\{[^}]+},?\s*)+};");
    re_find!(input, exp.as_str())
}

fn fdef<'a>(input: CompleteStr<'a>, name: &'a str) -> IResult<CompleteStr<'a>, CompleteStr<'a>> {
    let mut exp = String::new();
    exp.push_str(name);
    exp.push_str(r"=function\(\w+\)\{.+\};");
    re_find!(input, exp.as_str())
}

named!(
    var<&str, Vec<&str>>,
    re_capture_static!(r"(\w+)\.\w+\(\w+,\d+\)")
);

pub fn decipher(script: &str) -> Result<(String, String), Box<Error>> {
    let mut temp_js = String::new();
    let (_, f) = parse_sig(CompleteStr(script)).unwrap();
    let (_, def) = fdef(CompleteStr(script), &f).unwrap();
    let (_, v) = var(def.0).unwrap();
    temp_js.push_str(def.0);
    match definition(CompleteStr(script), v[1]) {
        Ok((_, body)) => {
            temp_js.push_str(body.0);
            // temp_js.push_str(&format!(r#"console.log({}("{}"))"#, f.0, sig));
            // fs::write("temp.js", &temp_js).unwrap();
        }
        Err(_) => {
            println!("bad");
        }
    }
    debug!("=====signature function start=====");
    debug!("{}", temp_js);
    debug!("=====signature function end=====");
    Ok((temp_js, f.0.to_string()))
}

// #[test]
// fn def_test() {
//     let script = fs::read_to_string("./script.js").unwrap();
//     match fdef(CompleteStr(&script), "pL") {
//         Ok((_, list)) => {
//             println!("Ok {:?}", list);
//         }
//         Err(e) => {
//             println!("Err {:?}", e);
//             println!("Err");
//         }
//     }
// }

// #[test]
// fn definition_test() {
//     let script = fs::read_to_string("./script.js").unwrap();
//     match definition(CompleteStr(&script), "oL") {
//         Ok((_, list)) => {
//             println!("Ok {:?}", list);
//         }
//         Err(e) => {
//             println!("Err {:?}", e);
//             println!("Err");
//         }
//     }
// }

// #[test]
// fn var_test() {
//     let script = r#"pL=function(a){a=a.split("");oL.yq(a,27);oL.Z0(a,50);oL.ZB(a,2);oL.yq(a,80);oL.ZB(a,3);return a.join("")};"#;
//     match var(&script) {
//         Ok((_, list)) => {
//             println!("Ok {:?}", list[1]);
//         }
//         Err(e) => {
//             println!("Err {:?}", e);
//             println!("Err");
//         }
//     }
// }

#[test]
fn decipher_test() {
    let script = ::std::fs::read_to_string("./script.js").unwrap();
    match decipher(script.as_str()) {
        Ok(_) => {}
        Err(e) => {
            println!("Err {:?}", e);
            println!("Err");
        }
    }
}
