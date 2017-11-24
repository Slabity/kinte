/// TODO: Macro for all of this from YAML file.
/// Handlers for all kinte requests.
use ::serde_json::Value;
use ::result::*;

/// Performs the request command.
pub fn request(request: &Value) -> Result<()> {
    let request = request.as_object().chain_err(|| "command request expects object as value")?;
    for (ref key, ref val) in request.iter() {
        match key.as_ref() {
            // TODO: Print to specific buffer
            "print_text" => {
                let text = val.as_str().chain_err(|| "print_text expects string as value")?;
                println!("{}", text);
            },
            e => return Err(format!("Unknown key: {}", e).into())
        }
    }

    Ok(())
}
