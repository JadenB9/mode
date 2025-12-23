/// ASCII art logo for MODE application
/// Professional block letter design
pub struct Logo;

impl Logo {
    /// Returns the MODE logo as a vector of strings
    /// Each string represents one line of the logo
    pub fn get() -> Vec<&'static str> {
        vec![
            r" ███╗   ███╗ ██████╗ ██████╗ ███████╗",
            r" ████╗ ████║██╔═══██╗██╔══██╗██╔════╝",
            r" ██╔████╔██║██║   ██║██║  ██║█████╗  ",
            r" ██║╚██╔╝██║██║   ██║██║  ██║██╔══╝  ",
            r" ██║ ╚═╝ ██║╚██████╔╝██████╔╝███████╗",
            r" ╚═╝     ╚═╝ ╚═════╝ ╚═════╝ ╚══════╝",
        ]
    }

    /// Returns the height of the logo in lines
    pub fn height() -> u16 {
        6
    }

    /// Returns the width of the logo in characters
    pub fn width() -> u16 {
        39
    }

    /// Returns a subtitle/tagline for the application
    pub fn tagline() -> &'static str {
        "Terminal Utility Manager"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logo_dimensions() {
        let logo = Logo::get();
        assert_eq!(logo.len(), Logo::height() as usize);

        // Verify all lines have the same width
        for line in logo {
            assert_eq!(line.chars().count(), Logo::width() as usize);
        }
    }
}
