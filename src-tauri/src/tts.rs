use hf_hub::api::sync::Api;
use once_cell::sync::Lazy;
use sbv2_core::tts::TTSModelHolder;
use std::{env, fs, sync::Arc};
use tokio::fs as tfs;
use tokio::sync::{Mutex, MutexGuard};

static MODELS_DIR: Lazy<String> =
    Lazy::new(|| env::var("ROOT_DIR").unwrap_or("models".to_string()));
fn load_model_holder() -> anyhow::Result<TTSModelHolder> {
    let api = Api::new()?;
    fs::create_dir(MODELS_DIR.clone()).ok();
    Ok(TTSModelHolder::new(
        &fs::read(
            api.model("googlefan/sbv2_onnx_models".to_string())
                .get("deberta.onnx")?,
        )?,
        &fs::read(
            api.model("googlefan/sbv2_onnx_models".to_string())
                .get("tokenizer.json")?,
        )?,
    )?)
}

static MODEL_HOLDER: Lazy<Option<Arc<Mutex<TTSModelHolder>>>> =
    Lazy::new(|| load_model_holder().ok().map(|m| Arc::new(Mutex::new(m))));

fn get_model_holder() -> Result<Arc<Mutex<TTSModelHolder>>, String> {
    MODEL_HOLDER
        .clone()
        .ok_or("Seems that loading bert and tokenizer has failed.".to_string())
}

#[tauri::command(async)]
pub async fn reload_models() -> Result<(), String> {
    let lock = get_model_holder()?;
    let mut lock = lock.lock().await;
    for m in lock.models() {
        lock.unload(m);
    }
    let models = MODELS_DIR.clone();
    let mut f = tfs::read_dir(&models)
        .await
        .map_err(|_| "models dir cannot be read".to_string())?;
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
        } else if name.ends_with(".sbv2") {
            let entry = &name[..name.len() - 5];
            let sbv2_bytes = match fs::read(format!("{models}/{entry}.sbv2")).await {
                Ok(b) => b,
                Err(e) => {
                    println!("Error loading sbv2_bytes from file {entry}: {e}");
                    continue;
                }
            };
            if let Err(e) = tts_model.load_sbv2file(&entry, sbv2_bytes) {
                println!("Error loading {entry}: {e}");
            };
        }
    }
    for entry in entries {
        let style_vectors_bytes =
            match tfs::read(format!("{models}/style_vectors_{entry}.json")).await {
                Ok(b) => b,
                Err(e) => {
                    println!("{entry} :{e}");
                    continue;
                }
            };
        let vits2_bytes = match tfs::read(format!("{models}/model_{entry}.onnx")).await {
            Ok(b) => b,
            Err(e) => {
                println!("{entry} :{e}");
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
pub async fn synthesize(
    ident: String,
    text: String,
    sdp_ratio: f32,
    length_scale: f32,
) -> Result<Vec<u8>, String> {
    let lock = get_model_holder()?;
    let lock = lock.lock().await;
    let buffer = match synthesize_inner(lock, ident, text, sdp_ratio, length_scale) {
        Ok(b) => b,
        Err(e) => return Err(e.to_string()),
    };
    Ok(buffer)
}
#[tauri::command(async)]
pub async fn models() -> Result<Vec<String>, String> {
    let lock = get_model_holder()?;
    let lock = lock.lock().await;
    Ok(lock.models())
}
