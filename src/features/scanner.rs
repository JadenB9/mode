use crate::utils::Result;
use std::net::{IpAddr, TcpStream, ToSocketAddrs};
use std::time::Duration;

/// Scanner state machine
#[derive(Debug, Clone)]
pub enum ScannerState {
    /// Selecting scan type
    SelectingScanType { selected: usize },
    /// Entering target IP or hostname
    EnteringTarget { scan_type: ScanType, input: String },
    /// Selecting scan options
    SelectingOptions {
        scan_type: ScanType,
        target: String,
        selected: usize,
        service_detection: bool,
        save_to_file: bool,
    },
    /// Confirming scan parameters
    Confirming {
        scan_type: ScanType,
        target: String,
        service_detection: bool,
        save_to_file: bool,
    },
    /// Scanning in progress
    Scanning {
        scan_type: ScanType,
        target: String,
        progress: usize,
        total: usize,
        service_detection: bool,
        save_to_file: bool,
    },
    /// Viewing results
    ViewingResults {
        target: String,
        open_ports: Vec<PortInfo>,
        scroll: usize,
    },
    /// Success state with message
    Success { message: String },
    /// Error state with message
    Error { message: String },
}

/// Port information
#[derive(Debug, Clone)]
pub struct PortInfo {
    pub port: u16,
    pub service: Option<String>,
    pub state: PortState,
}

/// Port state
#[derive(Debug, Clone, PartialEq)]
pub enum PortState {
    Open,
    Closed,
    Filtered,
}

/// Scan type options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScanType {
    /// Quick scan of common ports
    QuickScan,
    /// Standard scan of top 100 ports
    StandardScan,
    /// Full scan of all 65535 ports
    FullScan,
    /// Custom port range
    CustomRange,
}

impl ScanType {
    /// Returns all available scan types
    pub fn all() -> Vec<ScanType> {
        vec![
            ScanType::QuickScan,
            ScanType::StandardScan,
            ScanType::FullScan,
            ScanType::CustomRange,
        ]
    }

    /// Returns the display name
    pub fn name(&self) -> &'static str {
        match self {
            ScanType::QuickScan => "Quick Scan",
            ScanType::StandardScan => "Standard Scan",
            ScanType::FullScan => "Full Scan",
            ScanType::CustomRange => "Custom Range",
        }
    }

    /// Returns the description
    pub fn description(&self) -> &'static str {
        match self {
            ScanType::QuickScan => "Scan common ports (21, 22, 23, 25, 53, 80, 110, 143, 443, 3306, 3389, 5432, 8080, 8443)",
            ScanType::StandardScan => "Scan top 100 most common ports",
            ScanType::FullScan => "Scan all 65535 ports (may take several minutes)",
            ScanType::CustomRange => "Scan a custom port range (e.g., 1-1000)",
        }
    }

    /// Returns the ports to scan
    pub fn get_ports(&self) -> Vec<u16> {
        match self {
            ScanType::QuickScan => vec![
                21, 22, 23, 25, 53, 80, 110, 143, 443, 3306, 3389, 5432, 8080, 8443,
            ],
            ScanType::StandardScan => {
                // Top 100 ports
                vec![
                    21, 22, 23, 25, 53, 80, 110, 111, 135, 139, 143, 443, 445, 993, 995, 1723,
                    3306, 3389, 5900, 8080, 8443, 20, 69, 123, 137, 138, 161, 162, 389, 636,
                    989, 990, 1025, 1026, 1027, 1433, 1434, 1521, 2049, 2082, 2083, 2086, 2087,
                    2095, 2096, 3128, 5432, 5800, 5901, 6000, 6001, 8000, 8008, 8009, 8081,
                    8082, 8083, 8084, 8085, 8086, 8087, 8088, 8089, 8090, 8180, 8181, 8888,
                    9090, 9091, 9100, 9999, 10000, 32768, 32769, 32770, 32771, 32772, 32773,
                    32774, 32775, 32776, 32777, 49152, 49153, 49154, 49155, 49156, 49157, 50000,
                    50001, 50002, 50003,
                ]
            }
            ScanType::FullScan => (1..=65535).collect(),
            ScanType::CustomRange => vec![], // Will be filled by user input
        }
    }
}

/// Scan options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScanOption {
    ServiceDetection,
    SaveToFile,
}

impl ScanOption {
    /// Returns all available options
    pub fn all() -> Vec<ScanOption> {
        vec![ScanOption::ServiceDetection, ScanOption::SaveToFile]
    }

    /// Returns the display name
    pub fn name(&self) -> &'static str {
        match self {
            ScanOption::ServiceDetection => "Service Detection",
            ScanOption::SaveToFile => "Save Results to File",
        }
    }

    /// Returns the description
    pub fn description(&self, enabled: bool) -> String {
        let status = if enabled { "ON" } else { "OFF" };
        match self {
            ScanOption::ServiceDetection => {
                format!("[{}] Attempt to identify services running on open ports", status)
            }
            ScanOption::SaveToFile => {
                format!("[{}] Save scan results to a file", status)
            }
        }
    }
}

/// Port Scanner feature
#[derive(Debug)]
pub struct Scanner {
    pub state: ScannerState,
}

impl Scanner {
    /// Creates a new Scanner
    pub fn new() -> Self {
        Self {
            state: ScannerState::SelectingScanType { selected: 0 },
        }
    }

    /// Moves selection up
    pub fn previous(&mut self) {
        match &mut self.state {
            ScannerState::SelectingScanType { selected } => {
                let total = ScanType::all().len();
                *selected = if *selected == 0 {
                    total - 1
                } else {
                    *selected - 1
                };
            }
            ScannerState::SelectingOptions { selected, .. } => {
                let total = ScanOption::all().len();
                *selected = if *selected == 0 {
                    total - 1
                } else {
                    *selected - 1
                };
            }
            ScannerState::ViewingResults { scroll, .. } => {
                if *scroll > 0 {
                    *scroll -= 1;
                }
            }
            _ => {}
        }
    }

    /// Moves selection down
    pub fn next(&mut self) {
        match &mut self.state {
            ScannerState::SelectingScanType { selected } => {
                let total = ScanType::all().len();
                *selected = (*selected + 1) % total;
            }
            ScannerState::SelectingOptions { selected, .. } => {
                let total = ScanOption::all().len();
                *selected = (*selected + 1) % total;
            }
            ScannerState::ViewingResults { scroll, open_ports, .. } => {
                if *scroll < open_ports.len().saturating_sub(1) {
                    *scroll += 1;
                }
            }
            _ => {}
        }
    }

    /// Confirms the selected scan type
    pub fn confirm_scan_type(&mut self) {
        if let ScannerState::SelectingScanType { selected } = self.state {
            let scan_type = ScanType::all()[selected];
            self.state = ScannerState::EnteringTarget {
                scan_type,
                input: String::new(),
            };
        }
    }

    /// Handles character input for target
    pub fn handle_char(&mut self, c: char) {
        if let ScannerState::EnteringTarget { input, .. } = &mut self.state {
            input.push(c);
        }
    }

    /// Handles backspace in target input
    pub fn handle_backspace(&mut self) {
        if let ScannerState::EnteringTarget { input, .. } = &mut self.state {
            input.pop();
        }
    }

    /// Advances from target input to options
    pub fn advance_to_options(&mut self) {
        if let ScannerState::EnteringTarget { scan_type, input } = self.state.clone() {
            if input.trim().is_empty() {
                self.state = ScannerState::Error {
                    message: "Target cannot be empty".to_string(),
                };
                return;
            }

            self.state = ScannerState::SelectingOptions {
                scan_type,
                target: input.trim().to_string(),
                selected: 0,
                service_detection: false,
                save_to_file: false,
            };
        }
    }

    /// Toggles an option
    pub fn toggle_option(&mut self) {
        if let ScannerState::SelectingOptions {
            selected,
            service_detection,
            save_to_file,
            ..
        } = &mut self.state
        {
            let option = ScanOption::all()[*selected];
            match option {
                ScanOption::ServiceDetection => *service_detection = !*service_detection,
                ScanOption::SaveToFile => *save_to_file = !*save_to_file,
            }
        }
    }

    /// Advances from options to confirmation
    pub fn advance_to_confirmation(&mut self) {
        if let ScannerState::SelectingOptions {
            scan_type,
            target,
            service_detection,
            save_to_file,
            ..
        } = self.state.clone()
        {
            self.state = ScannerState::Confirming {
                scan_type,
                target,
                service_detection,
                save_to_file,
            };
        }
    }

    /// Executes the scan
    pub fn execute_scan(&mut self) {
        if let ScannerState::Confirming {
            scan_type,
            target,
            service_detection,
            save_to_file,
        } = self.state.clone()
        {
            let ports = scan_type.get_ports();
            let total = ports.len();

            self.state = ScannerState::Scanning {
                scan_type,
                target: target.clone(),
                progress: 0,
                total,
                service_detection,
                save_to_file,
            };

            // Perform the actual scan
            match self.perform_scan(&target, &ports, service_detection) {
                Ok(open_ports) => {
                    if save_to_file {
                        if let Err(e) = self.save_results(&target, &open_ports) {
                            self.state = ScannerState::Error {
                                message: format!("Scan completed but failed to save results: {}", e),
                            };
                            return;
                        }
                    }

                    if open_ports.is_empty() {
                        self.state = ScannerState::Success {
                            message: format!("Scan completed. No open ports found on {}", target),
                        };
                    } else {
                        self.state = ScannerState::ViewingResults {
                            target,
                            open_ports,
                            scroll: 0,
                        };
                    }
                }
                Err(e) => {
                    self.state = ScannerState::Error {
                        message: format!("Scan failed: {}", e),
                    };
                }
            }
        }
    }

    /// Performs the actual port scan
    fn perform_scan(
        &mut self,
        target: &str,
        ports: &[u16],
        service_detection: bool,
    ) -> Result<Vec<PortInfo>> {
        let mut open_ports = Vec::new();

        // Resolve hostname to IP
        let ip = self.resolve_target(target)?;

        let timeout = Duration::from_millis(500);

        for (idx, &port) in ports.iter().enumerate() {
            // Update progress
            if let ScannerState::Scanning { progress, .. } = &mut self.state {
                *progress = idx + 1;
            }

            let addr = format!("{}:{}", ip, port);
            if let Ok(socket_addr) = addr.parse::<std::net::SocketAddr>() {
                match TcpStream::connect_timeout(&socket_addr, timeout) {
                    Ok(_) => {
                        let service = if service_detection {
                            Self::detect_service(port)
                        } else {
                            None
                        };

                        open_ports.push(PortInfo {
                            port,
                            service,
                            state: PortState::Open,
                        });
                    }
                    Err(_) => {
                        // Port is closed or filtered
                    }
                }
            }
        }

        Ok(open_ports)
    }

    /// Resolves target hostname to IP address
    fn resolve_target(&self, target: &str) -> Result<IpAddr> {
        // Try to parse as IP address first
        if let Ok(ip) = target.parse::<IpAddr>() {
            return Ok(ip);
        }

        // Try to resolve as hostname
        let addr = format!("{}:80", target);
        match addr.to_socket_addrs() {
            Ok(mut addrs) => {
                if let Some(socket_addr) = addrs.next() {
                    Ok(socket_addr.ip())
                } else {
                    Err(crate::utils::ModeError::Generic(format!(
                        "Failed to resolve hostname: {}",
                        target
                    )))
                }
            }
            Err(e) => Err(crate::utils::ModeError::Generic(format!(
                "Failed to resolve target {}: {}",
                target, e
            ))),
        }
    }

    /// Detects service running on a port
    fn detect_service(port: u16) -> Option<String> {
        let service = match port {
            20 => "FTP Data",
            21 => "FTP",
            22 => "SSH",
            23 => "Telnet",
            25 => "SMTP",
            53 => "DNS",
            80 => "HTTP",
            110 => "POP3",
            143 => "IMAP",
            443 => "HTTPS",
            445 => "SMB",
            993 => "IMAPS",
            995 => "POP3S",
            1433 => "MSSQL",
            1521 => "Oracle",
            3306 => "MySQL",
            3389 => "RDP",
            5432 => "PostgreSQL",
            5900 => "VNC",
            6379 => "Redis",
            8080 => "HTTP Proxy",
            8443 => "HTTPS Alt",
            27017 => "MongoDB",
            _ => return None,
        };

        Some(service.to_string())
    }

    /// Saves scan results to a file
    fn save_results(&self, target: &str, results: &[PortInfo]) -> Result<()> {
        use std::fs::File;
        use std::io::Write;

        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
        let filename = format!("scan_{}_{}.txt", target.replace(".", "_"), timestamp);

        let mut file = File::create(&filename)?;

        writeln!(file, "Port Scan Results")?;
        writeln!(file, "==================")?;
        writeln!(file, "Target: {}", target)?;
        writeln!(file, "Scan Time: {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"))?;
        writeln!(file, "Open Ports: {}\n", results.len())?;

        if results.is_empty() {
            writeln!(file, "No open ports found.")?;
        } else {
            writeln!(file, "PORT     STATE    SERVICE")?;
            writeln!(file, "----     -----    -------")?;
            for port_info in results {
                let service = port_info.service.as_deref().unwrap_or("unknown");
                writeln!(file, "{:<8} {:<8} {}", port_info.port, "open", service)?;
            }
        }

        Ok(())
    }

    /// Goes back to previous state
    pub fn go_back(&mut self) {
        match &self.state {
            ScannerState::EnteringTarget { scan_type, .. } => {
                let idx = ScanType::all().iter().position(|st| st == scan_type).unwrap_or(0);
                self.state = ScannerState::SelectingScanType { selected: idx };
            }
            ScannerState::SelectingOptions { scan_type, .. } => {
                self.state = ScannerState::EnteringTarget {
                    scan_type: *scan_type,
                    input: String::new(),
                };
            }
            ScannerState::Confirming { scan_type, target, service_detection, save_to_file } => {
                self.state = ScannerState::SelectingOptions {
                    scan_type: *scan_type,
                    target: target.clone(),
                    selected: 0,
                    service_detection: *service_detection,
                    save_to_file: *save_to_file,
                };
            }
            _ => {}
        }
    }

    /// Returns whether the scanner is done
    pub fn is_done(&self) -> bool {
        matches!(
            self.state,
            ScannerState::Success { .. } | ScannerState::Error { .. }
        )
    }

    /// Gets the current selected index
    pub fn get_selected(&self) -> Option<usize> {
        match self.state {
            ScannerState::SelectingScanType { selected } => Some(selected),
            ScannerState::SelectingOptions { selected, .. } => Some(selected),
            _ => None,
        }
    }

    /// Gets the current input
    pub fn get_input(&self) -> String {
        match &self.state {
            ScannerState::EnteringTarget { input, .. } => input.clone(),
            _ => String::new(),
        }
    }

    /// Gets the prompt text for the current state
    pub fn get_prompt(&self) -> String {
        match &self.state {
            ScannerState::SelectingScanType { .. } => {
                "Select scan type (↑/↓ to navigate, Enter to select, ESC to cancel):".to_string()
            }
            ScannerState::EnteringTarget { scan_type, .. } => {
                format!("{}\nEnter target IP address or hostname:", scan_type.name())
            }
            ScannerState::SelectingOptions { .. } => {
                "Configure scan options (↑/↓ to navigate, Space to toggle, Enter to continue):".to_string()
            }
            ScannerState::Confirming { .. } => "Review scan parameters:".to_string(),
            ScannerState::Scanning { progress, total, .. } => {
                format!("Scanning... {}/{} ports", progress, total)
            }
            ScannerState::ViewingResults { target, open_ports, .. } => {
                format!("Scan Results for {} ({} open ports)", target, open_ports.len())
            }
            ScannerState::Success { message } => message.clone(),
            ScannerState::Error { message } => format!("Error: {}", message),
        }
    }

    /// Gets confirmation data
    pub fn get_confirmation_data(&self) -> Option<Vec<(String, String)>> {
        match &self.state {
            ScannerState::Confirming {
                scan_type,
                target,
                service_detection,
                save_to_file,
            } => {
                let data = vec![
                    ("Scan Type".to_string(), scan_type.name().to_string()),
                    ("Target".to_string(), target.clone()),
                    ("Ports".to_string(), format!("{} ports", scan_type.get_ports().len())),
                    (
                        "Service Detection".to_string(),
                        if *service_detection { "Enabled" } else { "Disabled" }.to_string(),
                    ),
                    (
                        "Save to File".to_string(),
                        if *save_to_file { "Yes" } else { "No" }.to_string(),
                    ),
                ];
                Some(data)
            }
            _ => None,
        }
    }

    /// Gets the current scan options state
    pub fn get_options_state(&self) -> Option<(bool, bool)> {
        match &self.state {
            ScannerState::SelectingOptions {
                service_detection,
                save_to_file,
                ..
            } => Some((*service_detection, *save_to_file)),
            _ => None,
        }
    }
}

impl Default for Scanner {
    fn default() -> Self {
        Self::new()
    }
}
