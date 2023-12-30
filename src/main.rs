use ftp::FtpStream;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    ftp_host: String,
    ftp_port: u16,
    ftp_user: String,
    ftp_password: String,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    match envy::from_env::<Config>() {
        Ok(config) => run_task(config).await,
        Err(error) => eprintln!("{:#?}", error),
    }
}

async fn run_task(config: Config) {
    let host = format!("{}:{}", config.ftp_host, config.ftp_port);

    println!("{}", &host);

    match FtpStream::connect(host.as_str()) {
        Ok(mut ftp_stream) => {
            ftp_stream
                .login(&config.ftp_user, &config.ftp_password)
                .expect("Couldn't login to the server");

            match ftp_stream.pwd() {
                Ok(pwd) => println!("Tô na pasta: {:#?}", pwd),
                Err(err) => eprintln!("Deu caquinha pra pegar a pasta: {:#?}", err),
            }
        }
        Err(error) => eprintln!("{:#?}", error),
    }
}
