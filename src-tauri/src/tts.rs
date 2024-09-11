use once_cell::sync::Lazy;
use sbv2_core::tts::{TTSIdent, TTSModelHolder};
use std::{env, fs, sync::Arc};
use tokio::fs as tfs;
use tokio::sync::{Mutex, MutexGuard};

fn load_model_holder() -> anyhow::Result<TTSModelHolder> {
    Ok(TTSModelHolder::new(
        &fs::read(env::var("BERT_MODEL_PATH").unwrap_or("models/deberta.onnx".to_string()))?,
        &fs::read(env::var("TOKENIZER_PATH").unwrap_or("models/tokenizer.json".to_string()))?,
    )?)
}

static MODEL_HOLDER: Lazy<Option<Arc<Mutex<TTSModelHolder>>>> =
    Lazy::new(|| load_model_holder().ok().map(|m| Arc::new(Mutex::new(m))));

fn get_model_holder() -> Result<Arc<Mutex<TTSModelHolder>>, String> {
    MODEL_HOLDER.ok_or("Seems that loading bert and tokenizer has failed.".to_string())
}

#[tauri::command(async)]
pub async fn reload_models() -> Result<(), String> {
    let lock = get_model_holder()?.lock().await;
    for m in lock.models() {
        lock.unload(m);
    }
    let models = env::var("MODELS_PATH").unwrap_or("models".to_string());
    let mut f = tfs::read_dir(&models).await?;
    let mut entries = vec![];
    while let Ok(Some(e)) = f.next_entry().await {
        let name = e.file_name().to_string_lossy().to_string();
        if name.ends_with(".onnx") && name.starts_with("model_") {
            let name_len = name.len();
            let name = name.chars();
            entries.push(
                name.collect::<Vec<_>>()[6..name_len - 5]
                    .iter()
                    .collect::<String>(),
            );
        }
    }
    for entry in entries {
        let style_vectors_bytes =
            match tfs::read(format!("{models}/style_vectors_{entry}.json")).await {
                Ok(b) => b,
                Err(e) => {
                    continue;
                }
            };
        let vits2_bytes = match tfs::read(format!("{models}/model_{entry}.onnx")).await {
            Ok(b) => b,
            Err(e) => {
                continue;
            }
        };
        lock.load(&entry, style_vectors_bytes, vits2_bytes).ok();
    }
    Ok(())
}

fn synthesize_inner(
    lock: MutexGuard<TTSModelHolder>,
    ident: String,
    text: String,
    sdp_ratio: f32,
    length_scale: f32,
) -> anyhow::Result<Vec<u8>> {
    let (bert_ori, phones, tones, lang_ids) = lock.parse_text(&text)?;
    let style_vector = lock.get_style_vector(&ident, 0, 1.0)?;
    Ok(lock.synthesize(
        ident,
        bert_ori.to_owned(),
        phones,
        tones,
        lang_ids,
        style_vector,
        sdp_ratio,
        length_scale,
    )?)
}

#[tauri::command(async)]
async fn synthesize(
    ident: String,
    text: String,
    sdp_ratio: f32,
    length_scale: f32,
) -> Result<Vec<u8>, String> {
    let lock = get_model_holder()?.lock().await;
    let buffer = match synthesize_inner(lock, ident, text, sdp_ratio, length_scale) {
        Ok(b) => b,
        Err(e) => Err(e.to_string()),
    };
    Ok(buffer)
}
#[tauri::command(async)]
async fn models() -> Result<Vec<String>, String> {
    let lock = get_model_holder()?.lock().await;
    Ok(lock.models())
}
