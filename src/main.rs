use serde::Deserialize;

#[derive(Deserialize, Default)]
struct Input {
    #[serde(default)]
    model: Option<Model>,
    #[serde(default)]
    effort: Option<Effort>,
    #[serde(default)]
    context_window: Option<Context>,
    #[serde(default)]
    cost: Option<Cost>,
}

#[derive(Deserialize, Default)]
struct Model {
    #[serde(default)]
    display_name: Option<String>,
    #[serde(default)]
    id: Option<String>,
}

#[derive(Deserialize, Default)]
struct Effort {
    #[serde(default)]
    level: Option<String>,
}

#[derive(Deserialize, Default)]
struct Context {
    #[serde(default)]
    used_percentage: Option<f64>,
    #[serde(default)]
    used_tokens: Option<f64>,
    #[serde(default)]
    context_window_size: Option<f64>,
}

#[derive(Deserialize, Default)]
struct Cost {
    #[serde(default)]
    total_cost_usd: Option<f64>,
}

fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap_or_default();
    println!("{}", render(&input));
}

fn render(json: &str) -> String {
    let input: Input = serde_json::from_str(json).unwrap_or_default();

    let model = parse_model(&input);
    let effort = parse_effort(&input);
    let context = parse_context(&input);
    let cost = parse_cost(&input);

    format!("🧠 {model} 💪 {effort} 💭 {context}% 💰 ${cost:.2}")
}

fn parse_model(input: &Input) -> &str {
    input
        .model
        .as_ref()
        .and_then(|model| {
            model
                .display_name
                .as_deref()
                .filter(|s| !s.is_empty())
                .or_else(|| model.id.as_deref().filter(|s| !s.is_empty()))
        })
        .unwrap_or("?")
}

fn parse_effort(input: &Input) -> &str {
    input
        .effort
        .as_ref()
        .and_then(|e| e.level.as_deref())
        .filter(|level| !level.is_empty())
        .unwrap_or("?")
}

fn parse_context(input: &Input) -> i32 {
    input
        .context_window
        .as_ref()
        .and_then(|cw| {
            cw.used_percentage.or_else(|| {
                let (used, size) = (cw.used_tokens?, cw.context_window_size?);
                (size > 0.0).then_some(used / size * 100.0)
            })
        })
        .unwrap_or(0.0)
        .round() as i32
}

fn parse_cost(input: &Input) -> f64 {
    input
        .cost
        .as_ref()
        .and_then(|cost| cost.total_cost_usd)
        .unwrap_or(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_input_renders_expected_line() {
        let json = r#"{
            "model": {"display_name": "Opus"},
            "effort": {"level": "high"},
            "context_window": {"used_percentage": 42.4},
            "cost": {"total_cost_usd": 1.234}
        }"#;
        assert_eq!(render(json), "🧠 Opus 💪 high 💭 42% 💰 $1.23");
    }

    #[test]
    fn model_defaults_to_id_when_display_name_empty_or_missing() {
        assert_eq!(
            render(r#"{"model": {"display_name": "", "id": "claude-opus"}}"#),
            "🧠 claude-opus 💪 ? 💭 0% 💰 $0.00"
        );
        assert_eq!(
            render(r#"{"model": {"id": "claude-opus"}}"#),
            "🧠 claude-opus 💪 ? 💭 0% 💰 $0.00"
        );
    }

    #[test]
    fn model_defaults_to_question_mark_when_empty() {
        assert_eq!(
            render(r#"{"model": {"display_name": ""}}"#),
            "🧠 ? 💪 ? 💭 0% 💰 $0.00"
        );
        assert_eq!(
            render(r#"{"model": {"id": ""}}"#),
            "🧠 ? 💪 ? 💭 0% 💰 $0.00"
        );
    }

    #[test]
    fn model_defaults_to_question_mark_when_missing() {
        assert_eq!(render("{}"), "🧠 ? 💪 ? 💭 0% 💰 $0.00");
    }

    #[test]
    fn effort_defaults_to_question_mark_when_empty() {
        assert_eq!(render(r#"{"effort": ""}"#), "🧠 ? 💪 ? 💭 0% 💰 $0.00");
        assert_eq!(
            render(r#"{"effort": {"level": ""}}"#),
            "🧠 ? 💪 ? 💭 0% 💰 $0.00"
        );
    }

    #[test]
    fn effort_defaults_to_question_mark_when_missing() {
        assert_eq!(render("{}"), "🧠 ? 💪 ? 💭 0% 💰 $0.00");
    }

    #[test]
    fn context_used_percentage_takes_priority_over_used_tokens() {
        let json = r#"{"context_window": {"used_percentage": 10.0, "used_tokens": 900, "context_window_size": 1000}}"#;
        assert_eq!(render(json), "🧠 ? 💪 ? 💭 10% 💰 $0.00");
    }

    #[test]
    fn context_computed_from_tokens_and_size() {
        let json = r#"{"context_window": {"used_tokens": 250, "context_window_size": 1000}}"#;
        assert_eq!(render(json), "🧠 ? 💪 ? 💭 25% 💰 $0.00");
    }

    #[test]
    fn context_defaults_to_zero_when_size_is_zero() {
        let json = r#"{"context_window": {"used_tokens": 250, "context_window_size": 0}}"#;
        assert_eq!(render(json), "🧠 ? 💪 ? 💭 0% 💰 $0.00");
    }

    #[test]
    fn context_rounds_to_nearest_integer() {
        let json = r#"{"context_window": {"used_percentage": 42.5}}"#;
        assert_eq!(render(json), "🧠 ? 💪 ? 💭 43% 💰 $0.00");
    }

    #[test]
    fn cost_is_formatted_with_two_decimals() {
        assert_eq!(
            render(r#"{"cost": {"total_cost_usd": 1.5}}"#),
            "🧠 ? 💪 ? 💭 0% 💰 $1.50"
        );
    }

    #[test]
    fn cost_defaults_to_zero_when_empty() {
        assert_eq!(
            render(r#"{"cost": {"total_cost_usd": }}"#),
            "🧠 ? 💪 ? 💭 0% 💰 $0.00"
        );
    }

    #[test]
    fn cost_defaults_to_zero_when_missing() {
        assert_eq!(render("{}"), "🧠 ? 💪 ? 💭 0% 💰 $0.00");
    }

    #[test]
    fn invalid_json_renders_default_line() {
        assert_eq!(render("not json"), "🧠 ? 💪 ? 💭 0% 💰 $0.00");
    }
}
