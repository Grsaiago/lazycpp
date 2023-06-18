use std::io::Write;

fn createCPP(path: &String, filename: &String, mut patharg: String)
{
    let mut fd: std::fs::File;

    patharg.push_str(".cpp");
    fd = match std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&patharg) {
            Ok(fd) => {fd},
            Err(_) => {eprintln!("\x1b[0;33m{filename}.cpp already exists\x1b[0m");return ;}
        };
    write!(&mut fd, "#include \"{filename}.hpp\"\n\n").expect("error writting .cpp");
    write!(&mut fd, "{filename}::{filename}(void)\n{{\n}}\n\n").expect("error writting .cpp");
    write!(&mut fd, "{filename}::~{filename}(void)\n{{\n}}\n\n").expect("error writting .cpp");
    write!(&mut fd, "{filename}\t&{filename}::operator=({filename} const &rhs)\n{{\n}}\n\n").expect("error writting .cpp");
    println!("\x1b[0;32mCreated {filename}.cpp\x1b[0m");
}

fn createHPP(path: &String, filename: &String, mut patharg: String)
{
    let mut fd: std::fs::File;

    patharg.push_str(".hpp");
    fd = match std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&patharg) {
            Ok(fd) => {fd},
            Err(_) => {eprintln!("\x1b[0;33m{filename}.hpp already exists\x1b[0m");return ;}
        };
    write!(&mut fd, "#ifndef {}_HPP\n# define {}_HPP\n\n", filename.to_uppercase(), filename.to_uppercase())
        .expect("error writting .cpp");
    write!(&mut fd, "class {filename}\n{{\n").
        expect("\x1b[0;31mError writting to {filename}.hpp\x1b[0m");
    write!(&mut fd, "\tpublic:\n")
        .expect("\x1b[0;31mError writting to {filename}.hpp\x1b[0m");
    write!(&mut fd, "\t\t{filename}(void);\n")
        .expect("\x1b[0;31mError writting to {filename}.hpp\x1b[0m");
    write!(&mut fd, "\t\t~{filename}(void);\n")
        .expect("\x1b[0;31mError writting to {filename}.hpp\x1b[0m");
    write!(&mut fd, "\t\t{filename}&\toperator=({filename} const &rhs);\n")
        .expect("\x1b[0;31mError writting to {filename}.hpp\x1b[0m");
    write!(&mut fd, "}};\n")
        .expect("\x1b[0;31mError writting to {filename}.hpp\x1b[0m");
    write!(&mut fd, "\n#endif\n").expect("Error on write to hpp");
    println!("\x1b[0;32mCreated {filename}.hpp\x1b[0m");
}

fn main() {
    let vec: Vec<String> = std::env::args().collect();
    let mut arg: Option<&String>;
    let path: String;
    let mut patharg: String;

    if (vec.len() <= 1) {
        println!("This tool was developed by gsaiago in an effort to help 42 students on the tedious parts of the CPP piscine");
        println!("ussage: lazycpp <class_names> ...");
        println!("P.S: lazycpp will not write to a file that already exists.");
        return ;
    }
    let mut i = vec.iter().skip(1);
    path = format!("{}{}", std::env::current_dir()
                        .expect("\x1b[0;31Error on getting path\x1b[0m")
                        .to_str()
                        .expect("\x1b[0;31Error on to_str\x1b[0m"), "/");
    arg = i.next();
    if (arg.is_some() && (arg.unwrap().eq("-h") || arg.unwrap().eq("--help"))) {
        println!("This tool was developed by gsaiago in an effort to help 42 students on the tedious parts of the CPP piscine");
        println!("ussage: lazycpp <class_names> ...");
        println!("P.S: lazycpp will not write to a file that already exists.");
        return ;
    }
    while (arg.is_some())
    {
        patharg = path.clone();
        patharg.push_str(arg.unwrap());
        createHPP(&path, &arg.unwrap(), patharg.clone());
        if (!arg.unwrap().starts_with('I')) {
            createCPP(&path, &arg.unwrap(), patharg.clone());
        }
        arg = i.next();
    }
    return ;
}
