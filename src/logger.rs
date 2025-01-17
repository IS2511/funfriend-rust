use tracing::{self, *};
use tracing_subscriber::{prelude::*, Layer};
use chrono::Local;
use ansi_term::{Color, Style}; // eat your heart ouy british people

pub struct Logger;
struct CustomLogLayer;
#[derive(Default)]
struct CustomLogVisitor{
	message: Option<String>
}

impl Logger{
	pub fn init(){
		subscriber::set_global_default(
			tracing_subscriber::Registry::default().with(CustomLogLayer)
		).expect("Failed to set default tracing subscriber");
		info!("Logger initialized");
	}
}

impl<S: Subscriber> Layer<S> for CustomLogLayer{
	fn on_event(&self, event: &tracing::Event<'_>, _: tracing_subscriber::layer::Context<'_, S>) {
		let mut visitor = CustomLogVisitor::default();
		event.record(&mut visitor);
		
		let timestamp = Local::now().to_rfc3339();
		let target = event.metadata().target();
		let level = event.metadata().level();
		
		let colored_level = match level{
			&Level::TRACE => Color::Purple.paint(level.to_string()),
			&Level::DEBUG => Color::Blue.paint(level.to_string()),
			&Level::INFO => Color::Green.paint(level.to_string()),
			&Level::WARN => Color::Yellow.paint(level.to_string()),
			&Level::ERROR => Color::Red.paint(level.to_string()),
		};
		
		let style = Style::new().dimmed();
		
		let message = format!("{}[{}]{} [{}/{}]: {}", style.prefix(), timestamp, style.suffix(), target, colored_level, visitor.message.as_deref().unwrap_or("[no message]"));
		
		println!("{message}");
	}
}

impl field::Visit for CustomLogVisitor {
	fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
		if field.name() == "message"{
			self.message = Some(format!("{:?}", value));
		}
	}
}