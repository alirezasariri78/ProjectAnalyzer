use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;

use super::result::AnalyzeResult;
pub struct Counter {
    input_channel: Receiver<PathBuf>,
    output_channel: Sender<AnalyzeResult>,
}

impl Counter {
    pub fn new(input_channel: Receiver<PathBuf>, output_channel: Sender<AnalyzeResult>) -> Self {
        Self {
            input_channel,
            output_channel,
        }
    }

    pub fn start(mut self) {
        tokio::spawn(async move {
            let mut result = AnalyzeResult::new();

            while let Some(x) = self.input_channel.recv().await {
                let postfix = x
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap_or("")
                    .split('.')
                    .last()
                    .unwrap_or("nothing_file");

                match File::open(x.clone()) {
                    Ok(file) => {
                        let content = BufReader::new(file);
                        result.add(postfix, content);
                    }
                    Err(_) => println!("can't read file:{}", x.to_str().unwrap()),
                }
            }

            if let Err(e) = self.output_channel.send(result).await {
                println!("error while sending signal in from counter channel: {}", e);
            }
        });
    }
}
