use html_escape::encode_text;
use regex::Regex;
use serde_json::{Error, to_value, Value};

pub fn debug_template_engine(json_value: Result<Value, Error>) -> Value {
    // Construire une structure HTML avec les données du contexte
    let mut html_output = String::new();
    html_output.push_str(r#"<div class='debug_html_template app_shadow_2' id="debug_context">"#);

    // Ajouter le contenu JSON formaté
    if let Ok(Value::Object(map)) = json_value {
        for (key, value) in map {
            html_output.push_str(&format!(
                r#"<div class="debug_html_key_value">
                    <strong>{}</strong>: <div class="debug_value">{}</div>
                </div>"#,
                key,
                match value {
                    Value::String(val) => {
                        if let Ok(json_value) = serde_json::from_str::<Value>(&val) {
                            format_json_and_colorize(&json_value)
                        } else {
                            format_html_and_colorize(&val)
                        }
                    }
                    other => encode_text(&other.to_string()).parse().unwrap(),
                }
            ));
        }
    }

    html_output.push_str(r#"</div>"#);

    to_value(html_output).unwrap()
}



fn format_json_and_colorize(json_value: &Value) -> String {
    let mut result = String::new();
    result.push_str(r#"<div class="debug_json">"#);
    match json_value {
        Value::Object(map) => {
            result.push_str(r#"<div class="debug_json_object">{</div>"#);
            for (key, value) in map {
                result.push_str(&format!(
                    r#"<div class="debug_json_key_value">
                        <span class="debug_json_key">"{}"</span>: {}
                    </div>"#,
                    encode_text(key),
                    format_json_and_colorize(value)
                ));
            }
            result.push_str(r#"<div class="debug_json_object">}</div>"#);
        }
        Value::Array(array) => {
            result.push_str(r#"<div class="debug_json_array">[</div>"#);
            for value in array {
                result.push_str(&format!(
                    r#"<div class="debug_json_array_value">{}</div>"#,
                    format_json_and_colorize(value)
                ));
            }
            result.push_str(r#"<div class="debug_json_array">]</div>"#);
        }
        Value::String(string) => {
            result.push_str(&format!(
                r#"<span class="debug_json_string">"{}"</span>"#,
                encode_text(string)
            ));
        }
        Value::Number(number) => {
            result.push_str(&format!(
                r#"<span class="debug_json_number">{}</span>"#,
                number
            ));
        }
        Value::Bool(boolean) => {
            result.push_str(&format!(
                r#"<span class="debug_json_boolean">{}</span>"#,
                boolean
            ));
        }
        Value::Null => {
            result.push_str(r#"<span class="debug_json_null">null</span>"#);
        }
    }
    result.push_str(r#"</div>"#);

    result
}


pub fn format_html_and_colorize(html: &str) -> String {
    let mut result = String::new();

    // Regex pour balises, attributs et valeurs
    let tag_re   = Regex::new(r"<(/?[a-zA-Z0-9_-]+)").unwrap(); // Capture les balises ouvrantes/fermantes
    let attr_re  = Regex::new(r"([a-zA-Z0-9_-]+)=").unwrap();  // Capture les attributs
    let value_re = Regex::new(r#""([^"]*)""#).unwrap();        // Capture les valeurs

    for line in html.lines() {
        let mut in_tag  = false;
        let mut buffer = String::new();

        for char in line.chars() {
            match char {
                '<' => {
                    // Ajouter le texte hors des balises dans le résultat
                    if !buffer.is_empty() {
                        result.push_str(&format!(
                            r#"<span class="debug_text">{}</span>"#,
                            buffer
                        ));
                        buffer.clear();
                    }

                    // Commencer une nouvelle balise
                    in_tag = true;
                    buffer.push(char);
                }
                '>' => {
                    // Terminer une balise et la coloriser
                    buffer.push(char);
                    if in_tag {
                        let colored_tag = colorize_tag(&buffer, &tag_re, &attr_re, &value_re);
                        result.push_str(&colored_tag);
                        buffer.clear();
                        in_tag = false;
                    }
                }
                _ => {
                    // Ajouter des caractères au buffer
                    buffer.push(char);
                }
            }
        }

        // Ajouter le texte restant dans le buffer
        if !buffer.is_empty() {
            result.push_str(&format!(
                r#"<span class="debug_text">{}</span>"#,
                buffer
            ));
        }

        // Ajouter une nouvelle ligne dans le résultat
        result.push('\n');
    }

    // Retourner le contenu final
    format!(r#"<pre class="debug_html_code">{}</pre>"#, result)
}

// Fonction pour coloriser une balise HTML complète
fn colorize_tag(tag: &str, tag_re: &Regex, attr_re: &Regex, value_re: &Regex) -> String {
    let mut result = String::new();

    // Coloriser la balise ouvrante ou fermante
    if let Some(tag_cap) = tag_re.captures(tag) {
        let tag_name = &tag_cap[1];
        let is_closing = tag_name.starts_with('/');

        result.push_str(&format!(
            r#"<span class="debug_html_tag">&lt;{}</span>"#,
            tag_name
        ));

        // Traiter les attributs et leurs valeurs
        let remainder = &tag[tag_cap[0].len()..];
        for attr_cap in attr_re.captures_iter(remainder) {
            let attr_name = &attr_cap[1];
            result.push_str(&format!(
                r#" <span class="debug_html_attr">{}</span>="#,
                attr_name
            ));

            if let Some(value_cap) = value_re.captures(remainder) {
                let value = &value_cap[1];
                result.push_str(&format!(
                    r#"<span class="debug_html_attr_value">"{}"</span>"#,
                    value
                ));
            }
        }

        if !is_closing {
            result.push_str(r#"<span class="debug_html_tag">&gt;</span>"#);
        } else {
            result.push_str(r#"<span class="debug_html_tag">&gt;</span>"#);
        }
    }

    result
}

