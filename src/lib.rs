use extism_pdk::*;
use serde_json::Value;

#[host_fn]
extern "ExtismHost" {
    fn write_topic (file_name: String, text : String) -> u32;
    fn send_message_to_user(message : String) -> u32;
}

#[plugin_fn]
pub fn started(input: String) -> FnResult<u32> {
    
    let is_json = serde_json::from_str::<Value>(&input);
    match is_json {
        Ok(json) => {
            if let(Some(actor_name_value), Some(virtual_parameters_value)) = (json.get("actor_name"), json.get("virtual_parameters")){
                let file_name = format!("{}.json", actor_name_value.to_string().trim_matches('"'));
                let virtual_parameters = virtual_parameters_value.to_string();
                unsafe{
                    _ = write_topic(file_name, virtual_parameters).unwrap();
                }
            }
        }
        Err(err) => {
            unsafe {
                send_message_to_user(err.to_string()).unwrap();
            }
        }
    }

    return Ok(1)
}