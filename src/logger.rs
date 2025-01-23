// eat your heart ouy british people
use ansi_term::{Color, Style};
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _};

struct CustomLogLayer;
#[derive(Default)]
struct CustomLogVisitor {
	message: Option<String>,
}

pub fn init() {
	tracing_subscriber::registry()
		.with(CustomLogLayer)
		.with(
			tracing_subscriber::EnvFilter::builder()
				.from_env()
				.expect("invalid format in RUST_LOG env variable"),
		)
		.try_init()
		.expect("failed to init logger");

	tracing::info!("Logger initialized.");
}

impl<S: tracing::Subscriber> tracing_subscriber::Layer<S> for CustomLogLayer {
	fn on_event(&self, event: &tracing::Event<'_>, _: tracing_subscriber::layer::Context<'_, S>) {
		let mut visitor = CustomLogVisitor::default();
		event.record(&mut visitor);

		let timestamp = chrono::Local::now().to_rfc3339();
		let target = event.metadata().target();
		let level = event.metadata().level();

		let colored_level = match level {
			&Level::TRACE => Color::Purple.paint(level.to_string()),
			&Level::DEBUG => Color::Blue.paint(level.to_string()),
			&Level::INFO => Color::Green.paint(level.to_string()),
			&Level::WARN => Color::Yellow.paint(level.to_string()),
			&Level::ERROR => Color::Red.paint(level.to_string()),
		};

		let style = Style::new().dimmed();

		let message = format!(
			"{}[{}]{} [{}/{}]: {}",
			style.prefix(),
			timestamp,
			style.suffix(),
			target,
			colored_level,
			visitor.message.as_deref().unwrap_or("[no message]")
		);

		println!("{message}");
	}
}

impl tracing::field::Visit for CustomLogVisitor {
	fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
		if field.name() == "message" {
			self.message = Some(format!("{:?}", value));
		}
	}
}
