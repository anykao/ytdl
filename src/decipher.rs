use nom::types::CompleteStr;
use nom::IResult;
use std::collections::HashSet;
use std::error::Error;
use std::fs;


fn fdef<'a>(input: CompleteStr<'a>, f: &'a str) -> IResult<CompleteStr<'a>, CompleteStr<'a>> {
    do_parse!(
        input,
        take_until_and_consume!(format!(r#"{}=function"#, f).as_str()) >> 
        delimited!(tag!("("), take_until!(")"), tag!(")")) >> 
        body: delimited!(tag!("{"), take_until!("}"), tag!("}")) >> 
        (body)
    )
}

// a=a.split("");oL.yq(a,27);oL.Z0(a,50);oL.ZB(a,2);oL.yq(a,80);oL.ZB(a,3);return a.join("")
named!(f2s<CompleteStr, Vec<(CompleteStr,CompleteStr)>>, 
    many0!(
        do_parse!(
            take_until_and_consume!(".") >>
            f: take_until!("(") >>
            body: delimited!(tag!("("), take_until!(")"), tag!(")")) >>
            ((f, body))
        )
    )
);

named!(parse_sig<CompleteStr, CompleteStr>, 
    do_parse!(
        take_until_and_consume!(r#""signature","#) >>
        f: take_until!("(") >>
        delimited!( tag!("("), take_until!(")"), tag!(")"))>>
        (f)
    )
);

// named!(
//   string<&str, &str>,
//   delimited!(
//     tag!("\""),
//     map_res!(
//       escaped!(take_while1!(is_alphanumeric), '\\', one_of!("\"n\\")),
//       str::from_utf8
//     ),
//     tag!("\"")
//   )
// );

named!(
  key_value<&str, (&str, &str)>,
  ws!(separated_pair!(take_until!(":"), tag!(":"), take_until!("}")))
);

named!(
  object<&str, Vec<(&str, &str)>>,
  dbg!(
      do_parse!(
      take_until!("{")>>

  body: ws!(
    delimited!(
      tag!("{"),
      separated_list!(tag!(","), key_value),
      tag!("};")
    )

      ) >>
      (body)
  )
  )
);


fn f2def<'a>(input: CompleteStr<'a>, f: &'a str) -> IResult<CompleteStr<'a>, CompleteStr<'a>> {
    do_parse!(
        input,
        take_until!(format!(r#"{}:function"#, f).as_str()) >> 
        body: take_until_and_consume!("}") >> 
        (body)
    )
}

pub fn decipher(_js: &str, s: &str) -> Result<String, Box<Error>> {
    let mut temp_js = String::new();
    // debug!("{}", s);
    let script = fs::read_to_string("./script.js")?;
    let (_, f) = parse_sig(CompleteStr(script.as_str())).unwrap();
    let (_, def) = fdef(CompleteStr(script.as_str()), &f).unwrap();
    let (_, f2l) = f2s(CompleteStr(&def)).unwrap();
    debug!("{}", f);
    debug!("{}", def);
    debug!("{:?}", f2l);
    let mut seen: HashSet<&str> = HashSet::new();
    for (k, v) in &f2l {
        if v.0 != r#""""# {
            if !seen.contains(k.0) {
                seen.insert(k.0);
                let (_, f2def) = f2def(CompleteStr(script.as_str()), &k).unwrap();
                temp_js.push_str(f2def.0);
                temp_js.push('}');
                temp_js.push('\n');
            }
        }
    }
    debug!("{}", temp_js);
    Ok(String::new())
}

#[test]
fn decipher_test() {
  let test = "var oL={ZB:function(a,b){a.splice(0,b)},
Z0:function(a,b){var c=a[0];a[0]=a[b%a.length];a[b%a.length]=c},
yq:function(a){a.reverse()}};";
    if let Ok((_, list)) =  object(test) {
        println!("Ok {:?}", list);
    } else {
        println!("Err");
    }
}
