use minify_js::{minify, Session, TopLevelMode};

#[derive(Debug)]
pub struct Minifier {}

impl Minifier {
    pub fn new() -> Self {
        return Self {};
    }

    pub fn javascript(&self, code: &[u8]) -> Vec<u8> {
        let session = Session::new();
        let mut out = Vec::new();

        let m = minify(&session, TopLevelMode::Global, code, &mut out);
        match m {
            Err(error) => {
                println!("{:#?}", error);
                return code.to_vec(); // Return without minifying
            }
            Ok(_) => {
                return out;
            }
        }
    }

    pub fn css(&self, code: &[u8]) -> Vec<u8> {
        match String::from_utf8(code.to_vec()) {
            Ok(s) => {
                let output = css_minify::optimizations::Minifier::default()
                    .minify(&s, css_minify::optimizations::Level::Two);

                match output {
                    Err(err) => {
                        println!("{:#?}", err);
                        return code.to_vec();
                    }
                    Ok(data) => return data.as_bytes().to_vec(),
                }
            }
            Err(err) => {
                println!("{:#?}", err);
                return code.to_vec();
            }
        }
    }
}
