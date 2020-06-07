//! Stdout exporter.
use super::Exporter;
use crate::config::FormatSelection;
use crate::error::Result;
use crate::message::Message;

pub struct Stdout {
    format: FormatSelection,
}

impl Stdout {
    pub fn new(format: FormatSelection) -> Stdout {
        Stdout { format }
    }
}

impl Exporter for Stdout {
    fn export(&self, msg: &Message) -> Result<()> {
        let export_content = match self.format {
            FormatSelection::JSON => msg.to_json(),
            FormatSelection::CSV => msg.to_csv(),
        };

        println!("{}", export_content);

        Ok(())
    }
}
