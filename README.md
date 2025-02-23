# SBV2 GUI
## 使い方
[リリース](https://github.com/tuna2134/sbv2-gui/releases)からダウンロードし、ダブルクリックした後しばらく待機すると起動します。  
現段階ではCPUのみ & x86_64のWindowsのみ対応です。

## 既存のモデルの変換方法
[こちら](https://github.com/tuna2134/sbv2-api/tree/main/convert)を参照してください。

## HTTP APIを使いたい
`./models`に.sbv2ファイルおよび[`debert.onnx`](https://huggingface.co/neody/sbv2-api-assets/resolve/main/deberta/deberta.onnx)、[`tokenizer.json`](https://huggingface.co/neody/sbv2-api-assets/resolve/main/deberta/tokenizer.json)を格納した後に、  
`.env`に
```env
BERT_MODEL_PATH=models/deberta.onnx
TOKENIZER_PATH=models/tokenizer.json
HOLDER_MAX_LOADED_MODElS=20
```
を記入し、  
CPUの場合は
```sh
docker run -it --rm -p 3000:3000 --name sbv2 \
-v ./models:/work/models --env-file .env \
ghcr.io/tuna2134/sbv2-api:cpu
```
CUDAの場合は
```
docker run -it --rm -p 3000:3000 --name sbv2 \
-v ./models:/work/models --env-file .env \
--gpus all \
ghcr.io/tuna2134/sbv2-api:cuda
```
とすることで http://localhost:3000/ にて立ち上がります。  
APIの利用方法については以下のcurlコマンドを参考にしてください。
```sh
curl -XPOST -H "Content-type: application/json" -d '{"text": "こんにちは","ident": "tsukuyomi"}' 'http://localhost:3000/synthesize' --output "output.wav"
curl http://localhost:3000/models
```

## アイコン
Flux devで  
Prompt: `Style bert vits2 simple logo`  
Seed: `1`  
で生成したものです。
![アイコン](https://raw.githubusercontent.com/tuna2134/sbv2-gui/main/public/icon.png)
