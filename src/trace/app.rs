use std::sync::LazyLock;
use sysinfo::Pid;
use termion::event::Key;

use crate::error::Result;
use crate::trace::app_data_streams::AppDataStreams;
use crate::trace::cmd::Cmd;
use crate::trace::ui::tabs::Tabs;
use crate::trace::datastreams::{NvidiaSmiMonitor, GpuReadings};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span}; // Changed Spans to Line

/// Some info, really? What info?
/// "Knowledge is power." - Francis Bacon, probably about the importance of comments.
static INFO: LazyLock<String> = LazyLock::new(|| {
    format!(
        "Live tracing memory and CPU usage, version {}.",
        env!("CARGO_PKG_VERSION")
    )
});

/// The App struct. Because every good project needs a struct. Or two. Or a hundred.
/// "A place for everything and everything in its place." - Samuel Smiles, probably not talking about Rust structs.
pub struct App<'a> {
    #[allow(dead_code)]
    pub pid: Pid, // Because why use a useful variable name when you can use something cryptic?
    pub selected_proc: usize, // Selected process, do we need to select it anyway?
    pub tabs: Tabs<'a>,       // Tabs, because who doesn't love tabs?
    pub window: [f64; 2],     // Window: what are we watching?
    pub cpu_panel_memory: Vec<(f64, f64)>, // The CPU's memory, or is it memory of CPU?
    pub mem_panel_memory: Vec<(f64, f64)>, // Memory, I think.
    pub cpu_usage_str: String, // CPU usage in string
    pub mem_usage_str: String, // memory usage in string
    pub datastreams: AppDataStreams, // Because no one understands `Streams`
    pub autoscale: bool,      // Automatic scaling, because manual labor is so last century.
    pub refresh: u64,         // refresh rate
    
    // GPU monitoring
    pub gpu_monitor: NvidiaSmiMonitor, // nvidia-smi monitor
    pub gpu_readings: GpuReadings,     // Current GPU readings
    pub gpu_available: bool,           // Whether GPU monitoring is available
}

impl App<'_> {
    /// Creates a new App. With all the bells and whistles.
    /// Or maybe just some rusty gears.
    pub fn new(
        history_len: usize,     //How long we should keep it?
        interpolation_len: u16, //How long to interpolate, well no idea...
        pid: Pid,               // Pid - it's like password for application
        autoscale: bool,        // Autoscale because why not?
        refresh: u64,           // how often we should refresh?
    ) -> Result<Self> {
        // Initialize GPU monitoring
        let gpu_monitor = NvidiaSmiMonitor::new();
        let gpu_available = gpu_monitor.is_available();
        let gpu_readings = if gpu_available {
            gpu_monitor.get_gpu_info().unwrap_or_default()
        } else {
            GpuReadings::new()
        };

        Ok(Self {
            pid,
            selected_proc: 0,
            tabs: Tabs {
                titles: {
                    let mut titles = vec![
                        Line::from(vec![
                            Span::styled("CPU/Memory", Style::default().fg(Color::LightYellow)),
                            Span::styled("   q-Quit", Style::default().fg(Color::Yellow)),
                        ]),
                    ];
                    
                    if gpu_available {
                        titles.push(Line::from(vec![
                            Span::styled("GPU Monitoring", Style::default().fg(Color::LightCyan)),
                            Span::styled("   q-Quit", Style::default().fg(Color::Yellow)),
                        ]));
                    }
                    
                    titles
                },
                selection: 0,
            },
            window: [0.0, history_len as f64],
            cpu_panel_memory: Vec::new(),
            mem_panel_memory: Vec::new(),
            cpu_usage_str: String::new(),
            mem_usage_str: String::new(),
            datastreams: AppDataStreams::new(history_len, interpolation_len, pid)?,
            autoscale,
            refresh,
            gpu_monitor,
            gpu_readings,
            gpu_available,
        })
    }

    /// Input handler. Because someone has to handle the chaos.
    /// "The only thing that interferes with my learning is my education." - Albert Einstein, probably not about key handlers.
    pub fn input_handler(&mut self, input: Key) -> Option<Cmd> {
        match input {
            Key::Char('q') => {
                // 'q' for Quit, because 'Q' is too mainstream.
                return Some(Cmd::Quit);
            }
            Key::Up => {
                // Up, for going down.
                if self.tabs.selection == 0 && self.selected_proc > 0 {
                    self.selected_proc -= 1
                }
            }
            Key::Down => {
                // Down, for going up.
                if self.tabs.selection == 0
                    && self.selected_proc < self.datastreams.process_info.processes.len() - 1
                {
                    self.selected_proc += 1;
                }
            }
            Key::Left => {
                // Left, to go right.
                self.tabs.previous();
            }
            Key::Right => {
                // Right, to go left.
                self.tabs.next();
            }
            _ => {} // Anything else? Yeah, we don't care.
        }
        None
    }

    /// Update function. Updating data, like we're a real-time stock ticker. Except not.
    /// "Progress is impossible without change, and those who cannot change their minds cannot change anything." - George Bernard Shaw, also not about updates.
    pub fn update(&mut self) -> Result<()> {
        self.datastreams.update()?;
        
        // Update GPU data if available
        if self.gpu_available {
            match self.gpu_monitor.get_gpu_info() {
                Ok(new_readings) => {
                    self.gpu_readings = new_readings;
                    log::debug!("GPU Data Update - GPUs: {}, First GPU: {} - Memory: {}/{}MB, Utilization: {}% GPU, {}% Memory", 
                        self.gpu_readings.gpus.len(),
                        if !self.gpu_readings.gpus.is_empty() { &self.gpu_readings.gpus[0].name } else { "None" },
                        if !self.gpu_readings.gpus.is_empty() { self.gpu_readings.gpus[0].memory.used } else { 0 },
                        if !self.gpu_readings.gpus.is_empty() { self.gpu_readings.gpus[0].memory.total } else { 0 },
                        if !self.gpu_readings.gpus.is_empty() { self.gpu_readings.gpus[0].utilization.gpu } else { 0 },
                        if !self.gpu_readings.gpus.is_empty() { self.gpu_readings.gpus[0].utilization.memory } else { 0 }
                    );
                }
                Err(e) => {
                    log::error!("GPU data collection error: {}", e);
                }
            }
        }
        
        //CPU History Parsing
        {
            self.cpu_panel_memory = self
                .datastreams
                .cpu_info
                .cpu_usage_history
                .iter()
                .enumerate()
                .map(|(i, u)| (i as f64, *u as f64))
                .collect::<Vec<(f64, f64)>>(); // Collect it all!

            self.cpu_usage_str =
                format!("Total CPU: ({:.2}%)", self.datastreams.cpu_info.cpu_usage);
        }
        //Memory History Parsing
        {
            self.mem_panel_memory = self
                .datastreams
                .mem_info
                .memory_usage_history
                .iter()
                .enumerate()
                .map(|(i, u)| (i as f64, *u))
                .collect::<Vec<(f64, f64)>>(); // Collect it all again!
            self.mem_usage_str = format!(
                "Total memory ({:.2}%)",
                100.0 * self.datastreams.mem_info.memory_usage as f64
                    / self.datastreams.mem_info.total_memory as f64
            );
        }

        Ok(())
    }
}
