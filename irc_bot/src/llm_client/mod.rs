use reqwest::Client;
use serde::{Serialize};
use anyhow::{Result, Context};  // Importar Result y Context desde anyhow

#[derive(Serialize)]
struct Input {
    model: String,
    prompt: String,
}

pub struct LLMClient {
    llm_api_url: String,
}

impl LLMClient {
    pub fn new(llm_api_url: String) -> Self {
        Self {
            llm_api_url,
        }
    }

    pub async fn ask_prompt(&self, prompt: String) -> Result<String> {
        let client = Client::new();
        let input = Input {
            model: "llama3.2".to_string(),
            prompt,
        };

        let mut response = client
            .post(&self.llm_api_url)
            .json(&input)
            .send()
            .await
            .context("Error al enviar la solicitud a la API")?;

        let mut full_response = String::new();

        // Fragmentos de la respuesta
        while let Some(chunk) = response.chunk().await? {
            let chunk_str = String::from_utf8_lossy(&chunk);
            if let Some(start) = chunk_str.find("\"response\":\"") {
                let start_index = start + "\"response\":\"".len();
                if let Some(end) = chunk_str[start_index..].find("\"") {
                    let response_part = &chunk_str[start_index..start_index + end];
                    full_response.push_str(response_part);
                }
            }

            // Verificar si la respuesta está completa
            if full_response.contains("\"done\":true") {
                break;
            }
        }

        // Eliminar saltos de línea y devolver el resultado final
        Ok(full_response.replace("\\n", " "))


    }
}
