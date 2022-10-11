use std::env;
use std::io;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Please select 1 file");
    }

    let interfaces = local_ip_address::list_afinet_netifas().unwrap();

    println!();
    println!("Select interface for game sharing:");
    println!();

    for (i, (name, ip)) in interfaces.iter().enumerate() {
        println!("{}) {}:\t{:?}", i + 1, name, ip);
    }

    let mut interface_answer = String::new();

    io::stdin()
        .read_line(&mut interface_answer)
        .expect("Can't read interface");

    let selected_ip = interfaces[interface_answer.trim().parse::<usize>().unwrap() - 1].1;
    let host = format!("{}:23183", selected_ip);
    let game_url = format!("http://{}/{}", host, &args[1]);

    println!();
    println!("Game is available on: {}", game_url);
    println!();

     let qrcode = fast_qr::QRBuilder::new(game_url)
         .ecl(fast_qr::ECL::H)
         .version(fast_qr::Version::V06)
         .build();

    qrcode.unwrap().print();

    let mut app = tide::new();
    app.at(format!("/{}", &args[1]).as_str()).serve_file(&args[1])?;
    app.listen(host).await?;
    Ok(())
}
