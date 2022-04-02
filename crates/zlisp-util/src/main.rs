use clap::Parser;
use zlisp_value::Value;

#[derive(clap::ArgEnum, Debug, Clone)]
enum FromFormat {
    JSON,
    Bin,
    Text,
}

#[derive(clap::ArgEnum, Debug, Clone)]
enum ToFormat {
    JSON,
    Bin,
    Text,
    Ast,
}

#[derive(Parser, Debug, Clone)]
struct Args {
    #[clap(long, arg_enum, help = "The input format")]
    from: FromFormat,
    #[clap(long, arg_enum, help = "The output format")]
    to: ToFormat,
    #[clap(help = "The source path")]
    input: String,
    #[clap(help = "The destination path (will be overwritten)")]
    output: String,
}

fn json_to_zlisp(value: serde_json::Value) -> Value {
    use serde_json::Value::*;
    match value {
        Null => panic!("expected any valid zlisp value, found `null`"),
        Bool(_) => panic!("expected any valid zlisp value, found `bool`"),
        Object(_) => panic!("expected any valid zlisp value, found `object`"),
        Number(n) => {
            if let Some(i) = n.as_i64() {
                return Value::Int(i.try_into().unwrap());
            }
            if let Some(u) = n.as_u64() {
                return Value::Int(u.try_into().unwrap());
            }
            if let Some(f) = n.as_f64() {
                return Value::Float(f as f32);
            }
            panic!("expected any valid zlisp value");
        }
        String(s) => Value::String(s),
        Array(a) => {
            let v: Vec<Value> = a.into_iter().map(json_to_zlisp).collect();
            Value::List(v)
        }
    }
}

fn main() {
    let args: Args = Args::parse();
    println!("Reading {}", args.input);
    let value: Value = match args.from {
        FromFormat::JSON => {
            let input = std::fs::read_to_string(args.input).unwrap();
            // due to serde_json's float handling (f64), an indirection is needed
            let value: serde_json::Value = serde_json::from_str(&input).unwrap();
            json_to_zlisp(value)
        }
        FromFormat::Bin => {
            let input = std::fs::read(args.input).unwrap();
            zlisp_bin::from_slice(&input).unwrap()
        }
        FromFormat::Text => {
            let input = std::fs::read_to_string(args.input).unwrap();
            zlisp_text::from_str(&input).unwrap()
        }
    };
    println!("Writing {}", args.output);
    match args.to {
        ToFormat::JSON => {
            let output = serde_json::to_string_pretty(&value).unwrap();
            std::fs::write(args.output, output).unwrap();
        }
        ToFormat::Bin => {
            let output = zlisp_bin::to_vec(&value).unwrap();
            std::fs::write(args.output, output).unwrap();
        }
        ToFormat::Text => {
            let config = zlisp_text::WhitespaceConfig::default();
            let output = zlisp_text::to_pretty(&value, config).unwrap();
            std::fs::write(args.output, output).unwrap();
        }
        ToFormat::Ast => {
            let output = format!("{:#?}", value);
            std::fs::write(args.output, output).unwrap();
        }
    }
    println!("Done.");
}
