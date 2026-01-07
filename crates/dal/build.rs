use utoipa_config::Config;

fn main() {
    Config::new()
        .alias_for("DateTimeUtc", "chrono::DateTime<chrono::Utc>")
        .write_to_file();
}
