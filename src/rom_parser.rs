pub mod rom_parser {
    pub fn parse_rom_file(filepath: &str) -> Result<Vec<u16>, String> {
        let file_open = std::fs::File::open(filepath);
    
        match file_open {
            Ok(mut file) => {
                let mut buffer: Vec<u8> = Vec::new();
    
                match file.read_to_end(&mut buffer) {
                    Ok(_) => {
                        let pairs = buffer.iter().step_by(2).zip(buffer.iter().skip(1).step_by(2));
    
                        Ok(pairs.map(|(&x, &y)| join_two_bytes(x, y)).collect())
                    }
                    Err(error) => Err(error.to_string())
                }
            }
            Err(e) => Err(e.to_string())
        }
    }

    pub struct ROMParser {
        
    }


}