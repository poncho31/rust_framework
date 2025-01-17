use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use std::fs;
use std::env;
use serde_json::{from_str, to_string_pretty, Value};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use base64::Engine;
use tokio::net::windows::named_pipe::PipeEnd::Client;

pub struct GeckoDriverAPI {
    driver_path: String,
    session_id: Option<String>,
    gecko_driver_host: String,
    error: Vec<HashMap<String, String>>,
    url_portal: String,
}

enum GeckoDriverFunction {
    Visit(fn(&mut GeckoDriverAPI, &str, Option<HashMap<&str, bool>>) -> Result<(), Box<dyn std::error::Error>>),
    FillForm(fn(&mut GeckoDriverAPI, HashMap<&str, &str>) -> Result<(), Box<dyn std::error::Error>>),
    ClickElement(fn(&mut GeckoDriverAPI, &str) -> Result<(), Box<dyn std::error::Error>>),
    ConfirmBox(fn(&mut GeckoDriverAPI, u64) -> Result<(), Box<dyn std::error::Error>>),
}


impl GeckoDriverAPI {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let url_portal = format!("http://{}:{}", env::var("APP_URL")?, env::var("APP_PORT")?);
        let driver_path = format!("{}/tests/GeckoDriver/geckodriver.exe", env::current_dir()?.display());
        let mut api = GeckoDriverAPI {
            driver_path,
            session_id: None,
            gecko_driver_host: "127.0.0.1:4444".to_string(),
            error: vec![],
            url_portal,
        };
        api.start_web_driver()?;
        if api.session_id.is_none() {
            panic!("NO SESSION ID RETRIEVED");
        }
        Ok(api)
    }

    pub fn start_web_driver(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Command::new("powershell")
            .arg("Stop-Process -Name geckodriver -Force")
            .output()?;

        Command::new("powershell")
            .arg("start")
            .arg(&self.driver_path)
            .output()?;

        let webdriver_url = format!("{}/session", self.gecko_driver_host);
        let data = serde_json::json!({
            "capabilities": {
                "browserName": "firefox",
                "moz:firefoxOptions": {
                    "prefs": {
                        "browser.startup.homepage": self.url_portal
                    }
                }
            }
        });

        let data_request = self.send_request(&webdriver_url, &data, "POST")?;
        self.session_id = data_request["value"]["sessionId"].as_str().map(String::from);
        Ok(())
    }

    pub fn visit(&mut self, url: &str, options: Option<HashMap<&str, bool>>) -> Result<(), Box<dyn std::error::Error>> {
        let webdriver_url = format!("{}/session/{}/url", self.gecko_driver_host, self.session_id.as_ref().unwrap());
        let response = self.send_request(&webdriver_url, &serde_json::json!({ "url": url }), "POST")?;

        if let Some(error) = response["value"]["error"].as_str() {
            self.error("visit_url", error.to_string());
        } else if options.clone().unwrap_or_default().get("get_html").copied().unwrap_or(false) {
            self.get_html_content()?;
        } else if options.unwrap_or_default().get("record_actions").copied().unwrap_or(false) {
            self.action_js("record_actions", None, None)?;
        }

        Ok(())
    }

    pub fn fill_form(&mut self, form_data: HashMap<&str, &str>) -> Result<(), Box<dyn std::error::Error>> {
        for (selector, value) in form_data {
            let element_id = self.find_element_id(selector)?;
            let element_tag = self.get_element_tag(&element_id)?;
            let element_type = self.get_element_type(&element_id)?;

            match element_tag.as_str() {
                "input" => match element_type.as_str() {
                    "text" | "number" | "time" | "date" | "email" | "password" => {
                        self.add_text_to_element(&element_id, value)?;
                    }
                    "file" => {
                        self.upload_file_to_element(selector, value)?;
                    }
                    _ => {
                        self.error("fill_form", format!("Unsupported input type '{}' for selector '{}'", element_type, selector));
                    }
                },
                "select" => {
                    self.slim_select(selector, value)?;
                }
                "textarea" => {
                    self.add_text_to_element(&element_id, value)?;
                }
                _ => {
                    self.error("fill_form", format!("Unsupported element type '{}' for selector '{}'", element_tag, selector));
                }
            }
        }

        Ok(())
    }

    fn upload_file_to_element(&mut self, selector: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        if !fs::metadata(file_path).is_ok() {
            self.error("upload_file", format!("File '{}' does not exist", file_path));
        }

        let webdriver_url = format!("{}/session/{}/execute/sync", self.gecko_driver_host, self.session_id.as_ref().unwrap());
        let script = r#"
            var input = document.querySelector(arguments[0]);
            if (!input) {
                throw new Error('Element not found');
            }
            var dataTransfer = new DataTransfer();
            var file = new File([new Uint8Array(atob(arguments[1]).split('').map(char => char.charCodeAt(0)))], arguments[2]);
            dataTransfer.items.add(file);
            input.files = dataTransfer.files;
            input.dispatchEvent(new Event('change'));
        "#;
        let file_content = Engine::encode(&fs::read(file_path)?);
        let data = serde_json::json!({
            "script": script,
            "args": [selector, file_content, file_path]
        });

        let response = self.send_request(&webdriver_url, &data, "POST")?;
        if let Some(error) = response["value"]["error"].as_str() {
            self.error("upload_file", error.to_string());
        }
        Ok(())
    }

    fn get_element_tag(&self, element_id: &str) -> Result<String, Box<dyn std::error::Error>> {
        let webdriver_url = format!("{}/session/{}/element/{}/name", self.gecko_driver_host, self.session_id.as_ref().unwrap(), element_id);
        let response = self.send_request(&webdriver_url, &serde_json::json!({}), "GET")?;
        Ok(response["value"].as_str().unwrap_or_default().to_string())
    }

    fn get_element_type(&self, element_id: &str) -> Result<String, Box<dyn std::error::Error>> {
        let webdriver_url = format!("{}/session/{}/element/{}/attribute/type", self.gecko_driver_host, self.session_id.as_ref().unwrap(), element_id);
        let response = self.send_request(&webdriver_url, &serde_json::json!({}), "GET")?;
        Ok(response["value"].as_str().unwrap_or_default().to_string())
    }

    fn get_element_attribute(&self, element_id: &str, type_attribute: &str) -> Result<String, Box<dyn std::error::Error>> {
        let webdriver_url = format!("{}/session/{}/element/{}/attribute/{}", self.gecko_driver_host, self.session_id.as_ref().unwrap(), element_id, type_attribute);
        let response = self.send_request(&webdriver_url, &serde_json::json!({}), "GET")?;
        Ok(response["value"].as_str().unwrap_or_default().to_string())
    }

    pub fn slim_select(&mut self, selector: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.click_element(&format!(".slimSelect{}", selector))?;

        let webdriver_url = format!("{}/session/{}/execute/sync", self.gecko_driver_host, self.session_id.as_ref().unwrap());
        let script = r#"
            let el = document.getElementById(arguments[0]);
            let options = arguments[1].split('||');
            options.forEach(option => {
                let elOption = el.nextElementSibling.querySelector('.ss-option[title*="' + option.trim() + '"]');
                if(elOption) {
                    elOption.click();
                } else {
                    console.log('Option non trouvée : ' + option);
                }
            });
        "#;

        let data = serde_json::json!({
            "script": script,
            "args": [selector.trim_start_matches('#'), value]
        });

        self.send_request(&webdriver_url, &data, "POST")?;
        Ok(())
    }

    pub fn confirm_box(&self, sleep_duration: u64) -> Result<(), Box<dyn std::error::Error>> {
        self.send_request(&format!("{}/session/{}/alert/accept", self.gecko_driver_host, self.session_id.as_ref().unwrap()), &serde_json::json!({}), "POST")?;
        sleep(Duration::from_secs(sleep_duration));
        Ok(())
    }

    pub fn click_element(&mut self, selector: &str) -> Result<(), Box<dyn std::error::Error>> {
        let element_id = self.find_element_id(selector)?;
        self.action_js("scroll_to_element", Some(selector), None)?;
        self.send_request(&format!("{}/session/{}/element/{}/click", self.gecko_driver_host, self.session_id.as_ref().unwrap(), element_id), &serde_json::json!({}), "POST")?;
        self.switch_to_new_window()?;
        self.sleep(3);
        Ok(())
    }

    pub fn switch_to_new_window(&self) -> Result<(), Box<dyn std::error::Error>> {
        let webdriver_url = format!("{}/session/{}/window/handles", self.gecko_driver_host, self.session_id.as_ref().unwrap());
        let response = self.send_request(&webdriver_url, &serde_json::json!({}), "GET")?;

        let window_handles: Vec<String> = response["value"].as_array().unwrap_or(&vec![]).iter().map(|v| v.as_str().unwrap().to_string()).collect();

        if window_handles.len() > 1 {
            let new_window_handle = window_handles.last().unwrap().to_string();
            self.close_old_windows(&window_handles, &new_window_handle)?;
            self.send_request(&format!("{}/session/{}/window", self.gecko_driver_host, self.session_id.as_ref().unwrap()), &serde_json::json!({ "handle": new_window_handle }), "POST")?;
        }

        Ok(())
    }

    pub fn close_old_windows(&self, window_handles: &[String], new_window_handle: &str) -> Result<(), Box<dyn std::error::Error>> {
        for handle in window_handles {
            if handle != new_window_handle {
                self.send_request(&format!("{}/session/{}/window", self.gecko_driver_host, self.session_id.as_ref().unwrap()), &serde_json::json!({ "handle": handle }), "DELETE")?;
            }
        }
        Ok(())
    }

    pub fn fill_input_mass(&self, elements: HashMap<&str, &str>) -> Result<(), Box<dyn std::error::Error>> {
        for (element, value) in elements {
            self.action_js("fill_in_mass", Some(element), Some(value))?;
        }
        Ok(())
    }

    pub fn action_js(&self, action: &str, selector: Option<&str>, values: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
        let webdriver_url = format!("{}/session/{}/execute/sync", self.gecko_driver_host, self.session_id.as_ref().unwrap());
        let data = match action {
            "click" => serde_json::json!({
                "script": "document.querySelector(arguments[0]).click();",
                "args": [selector.unwrap()]
            }),
            "record_actions" => serde_json::json!({
                "script": "(function() { localStorage.setItem('record_actions', 'true'); location.reload(); })();",
                "args": []
            }),
            "scroll_to_bottom" => serde_json::json!({
                "script": "window.scrollTo(0, document.body.scrollHeight);",
                "args": []
            }),
            "scroll_to_element" => serde_json::json!({
                "script": "document.querySelector(arguments[0]).scrollIntoView({ behavior: 'smooth', block: 'start' });",
                "args": [selector.unwrap()]
            }),
            "fill_in_mass" => serde_json::json!({
                "script": r#"
                    (function(selector, values) {
                        const elements = document.querySelectorAll(selector);
                        elements.forEach(function(element) {
                            element.scrollIntoView({ behavior: 'smooth', block: 'start' });
                            let new_value = null;
                            if (values.includes('BASED_ON::')) {
                                const nearbyBasedOnSelector = values.replace('BASED_ON::', '').trim();
                                const parentRow = element.closest('tr');
                                const targetElement = parentRow.querySelector(nearbyBasedOnSelector);
                                if (targetElement) {
                                    let rawValue = targetElement.textContent || targetElement.value;
                                    let cleanedValue = rawValue.replace(/[^\d.,]/g, '');
                                    new_value = parseFloat(cleanedValue.replace(',', '.')) / 2;
                                    new_value = parseFloat(new_value.toFixed(0));
                                }
                            }
                            element.value = new_value ?? values;
                            element.focus();
                            element.checked = true;
                            element.dispatchEvent(new Event('change', { bubbles: true, cancelable: true }));
                        });
                    })(arguments[0], arguments[1]);
                "#,
                "args": [selector.unwrap(), values.unwrap()]
            }),
            _ => serde_json::json!({}),
        };

        if !data.is_null() {
            self.send_request(&webdriver_url, &data, "POST")?;
            sleep(Duration::from_secs(1));
        }
        Ok(())
    }

    fn find_element_id(&mut self, selector: &str) -> Result<String, Box<dyn std::error::Error>> {
        let webdriver_url = format!("{}/session/{}/element", self.gecko_driver_host, self.session_id.as_ref().unwrap());
        let data = serde_json::json!({
            "using": "css selector",
            "value": selector
        });

        let response = self.send_request(&webdriver_url, &data, "POST")?;
        let value = response["value"]["element-6066-11e4-a52e-4f735466cecf"].as_str().unwrap_or_default().to_string();

        if value.is_empty() {
            self.error("find_element_id", format!("Unable to find element with selector '{}'", selector));
        }

        Ok(value)
    }

    fn error(&mut self, name: &str, message: String) {
        let mut error_map = HashMap::new();
        error_map.insert(name.to_string(), message);
        self.error.push(error_map);
    }

    fn add_text_to_element(&self, element_id: &str, text: &str) -> Result<(), Box<dyn std::error::Error>> {
        let webdriver_url = format!("{}/session/{}/element/{}/value", self.gecko_driver_host, self.session_id.as_ref().unwrap(), element_id);
        let data = serde_json::json!({
            "text": text,
            "value": text.chars().collect::<Vec<_>>()
        });

        self.send_request(&webdriver_url, &data, "POST")?;
        Ok(())
    }

    pub fn get_html_content(&self) -> Result<String, Box<dyn std::error::Error>> {
        let source_url = format!("{}/session/{}/source", self.gecko_driver_host, self.session_id.as_ref().unwrap());
        let data = self.send_request(&source_url, &serde_json::json!({}), "GET")?;
        Ok(data["value"].as_str().unwrap_or_default().to_string())
    }

    fn send_request(&self, url: &str, data: &Value, method: &str) -> Result<Value, Box<dyn std::error::Error>> {
        let client = Client::new();
        let response = match method {
            "POST" => client.post(url).json(data).send()?,
            "GET" => client.get(url).send()?,
            "DELETE" => client.delete(url).json(data).send()?,
            _ => client.get(url).send()?,
        };

        let response_text = response.text()?;
        Ok(serde_json::from_str(&response_text)?)
    }

    pub fn sleep(&self, number_seconds: u64) {
        sleep(Duration::from_secs(number_seconds));
    }

    pub fn replay_actions(&mut self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        if fs::metadata(file_path).is_ok() {
            let actions: Vec<HashMap<String, Value>> = serde_json::from_str(&fs::read_to_string(file_path)?)?;
            for action in actions {
                let function = action["function"].as_str().unwrap();
                let args = action["args"].as_array().unwrap();

                if let Some(method) = self.get_method(function) {
                    method(&mut self, args)?;
                }
            }
        }
        Ok(())
    }

    fn get_method(&self, function: &str)  -> Result<(), Box<dyn std::error::Error>> {
        match function {
            "visit" => Some(Self::visit),
            "fill_form" => Some(Self::fill_form),
            "click_element" => Some(Self::click_element),
            "confirm_box" => Some(Self::confirm_box),
            "fill_input_mass" => Some(Self::fill_input_mass),
            "action_js" => Some(Self::action_js),
            _ => None,
        }
    }


    pub fn save_record_actions() -> bool {
        if env::var("REQUEST_METHOD").unwrap_or_default() == "POST" {
            let filename = "tests/GeckoDriver/actions_log.json";
            let mut actions = String::new();
            std::io::stdin().read_to_string(&mut actions).unwrap();
            let new_actions: Value = match from_str(&actions) {
                Ok(val) => val,
                Err(_) => {
                    eprintln!("JSON decode error");
                    return false;
                }
            };

            let mut merged_actions = if let Ok(mut file) = File::open(filename) {
                let mut existing_actions = String::new();
                file.read_to_string(&mut existing_actions).unwrap();
                match from_str(&existing_actions) {
                    Ok(Value::Array(mut arr)) => {
                        if let Some(last_action) = arr.last() {
                            if last_action["function"] == "slim_select" && last_action["args"][0] == new_actions[0]["args"][0] {
                                arr.pop();
                            }
                        }
                        arr.extend(new_actions.as_array().unwrap().clone());
                        Value::Array(arr)
                    }
                    _ => new_actions,
                }
            } else {
                new_actions
            };

            let mut file = OpenOptions::new().write(true).create(true).open(filename).unwrap();
            file.write_all(to_string_pretty(&merged_actions).unwrap().as_bytes()).unwrap();
            return true;
        }
        false
    }

    pub fn keep_awake(interval: u64) {
        println!("Keep awake {} seconds", interval);
        loop {
            match std::env::consts::OS {
                "windows" => {
                    Command::new("powershell")
                        .arg("-command")
                        .arg("$wshell = New-Object -ComObject wscript.shell; $wshell.SendKeys('{SCROLLLOCK}')")
                        .output()
                        .expect("Failed to execute command");
                },
                "linux" => {
                    Command::new("xdotool")
                        .arg("key")
                        .arg("shift")
                        .output()
                        .expect("Failed to execute command");
                },
                _ => {
                    println!("Unsupported OS");
                }
            }
            sleep(Duration::from_secs(interval));
        }
    }

}