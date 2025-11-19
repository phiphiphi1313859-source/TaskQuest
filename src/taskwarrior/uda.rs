use anyhow::{Context, Result};
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

pub struct UDAManager;

impl UDAManager {
    /// Get path to .taskrc file
    fn get_taskrc_path() -> Result<PathBuf> {
        let home = std::env::var("HOME").context("HOME environment variable not set")?;
        Ok(PathBuf::from(home).join(".taskrc"))
    }

    /// Check if UDAs are already configured
    pub fn is_configured() -> Result<bool> {
        let taskrc = Self::get_taskrc_path()?;
        if !taskrc.exists() {
            return Ok(false);
        }

        let file = std::fs::File::open(&taskrc)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            if line.contains("uda.challenge.type") {
                return Ok(true);
            }
        }

        Ok(false)
    }

    /// Add TaskQuest UDAs to .taskrc
    pub fn configure() -> Result<()> {
        if Self::is_configured()? {
            return Ok(());
        }

        let taskrc = Self::get_taskrc_path()?;
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&taskrc)?;

        writeln!(file, "\n# TaskQuest UDAs")?;
        writeln!(file, "uda.challenge.type=numeric")?;
        writeln!(file, "uda.challenge.label=Challenge")?;
        writeln!(file, "uda.challenge.values=1,2,3,4,5,6,7,8,9,10")?;
        writeln!(file)?;
        writeln!(file, "# Stats trained by this task (66% to stat1, 33% to stat2)")?;
        writeln!(file, "uda.stat1.type=string")?;
        writeln!(file, "uda.stat1.label=Primary Stat")?;
        writeln!(file, "uda.stat1.values=STR,DEX,CON,INT,WIS,CHA")?;
        writeln!(file)?;
        writeln!(file, "uda.stat2.type=string")?;
        writeln!(file, "uda.stat2.label=Secondary Stat")?;
        writeln!(file, "uda.stat2.values=STR,DEX,CON,INT,WIS,CHA")?;
        writeln!(file)?;
        writeln!(file, "# Optional: Color coding for challenge levels")?;
        writeln!(file, "color.uda.challenge.1=color246  # Trivial - gray")?;
        writeln!(file, "color.uda.challenge.2=color246")?;
        writeln!(file, "color.uda.challenge.3=color250  # Easy - lighter gray")?;
        writeln!(file, "color.uda.challenge.4=color250")?;
        writeln!(file, "color.uda.challenge.5=color255  # Medium - white")?;
        writeln!(file, "color.uda.challenge.6=color255")?;
        writeln!(file, "color.uda.challenge.7=color226  # Hard - yellow")?;
        writeln!(file, "color.uda.challenge.8=color208  # Deadly - orange")?;
        writeln!(file, "color.uda.challenge.9=color196  # Legendary - red")?;
        writeln!(file, "color.uda.challenge.10=color201 # Epic - magenta")?;

        println!("TaskQuest UDAs added to ~/.taskrc");

        Ok(())
    }
}
