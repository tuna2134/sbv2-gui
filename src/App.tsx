import { useEffect, useState } from "react";
import { reloadModels, getModels, synthesize, open, getPath } from "./typing";
import { Button } from "~/components/ui/button";
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from "~/components/ui/select";
import { Label } from "~/components/ui/label";
import { Textarea } from "~/components/ui/textarea";
import { Slider } from "~/components/ui/slider";

function App() {
	const [models, setModels] = useState<string[]>([]);
	const [model, setModel] = useState<string | null>(null);
	const [text, setText] = useState<string>("こんにちは。");
	const [speed, setSpeed] = useState(1);
	const [sdpRatio, setSdpRatio] = useState(0.4);
	const [reloading, setReloading] = useState(false);
	const [inSynthesize, setInSynthesize] = useState(false);
	const [audio, setAudio] = useState<string | null>(null);
	const [error, setError] = useState<string | null>(null);
	const [path, setPath] = useState<string | null>(null);
	useEffect(() => {
		(async () => {
			try {
				setPath(await getPath());
			} catch (e) {
				setError(String(e));
			}
		})();
		(async () => {
			setReloading(true);
			try {
				await reloadModels();
				setModels(await getModels());
			} catch (e) {
				setError(String(e));
			}
			setReloading(false);
		})();
	}, []);
	if (error) {
		return (
			<div className="flex min-h-[100vh] justify-center items-center flex-col">
				<p className="text-lg">エラー: {error}</p>
				<Button
					onClick={() => {
						setError(null);
					}}
				>
					理解した
				</Button>
			</div>
		);
	}
	if (reloading) {
		return (
			<div className="flex min-h-[100vh] justify-center items-center">
				<div>
					<p className="text-lg">読み込み中</p>
					<br />
					<p>初回は1GBほどのダウンロードが発生します</p>
				</div>
			</div>
		);
	}
	if (models.length == 0) {
		return (
			<div className="flex min-h-[100vh] justify-center items-center flex-col">
				<p className="text-lg">
					モデルを
					<a className="text-slate-600" onClick={() => open()}>
						{path}
					</a>
					に配置してください。
				</p>
				<div className="flex mt-2 gap-2">
					<Button
						onClick={async () => {
							setReloading(true);
							try {
								await reloadModels();
								setModels(await getModels());
							} catch (e) {
								setError(String(e));
							}
							setModel(null);
							setReloading(false);
						}}
					>
						再読み込み
					</Button>
					<Button onClick={() => open()}>モデルファイルを開く</Button>
				</div>
			</div>
		);
	}
	return (
		<div className="min-h-[100vh] p-20">
			<Label htmlFor="model">使用するモデル</Label>
			<Select name="model" onValueChange={(value) => setModel(value)}>
				<SelectTrigger className="w-1/3 md:w-1/4">
					<SelectValue />
				</SelectTrigger>
				<SelectContent>
					{models.map((m) => {
						return (
							<SelectItem value={m} key={m}>
								{m[0].toUpperCase() + m.slice(1).toLowerCase()}
							</SelectItem>
						);
					})}
				</SelectContent>
			</Select>
			<Label htmlFor="text">テキスト</Label>
			<Textarea
				name="text"
				onChange={(e) => setText(e.currentTarget.value)}
				defaultValue="こんにちは。"
			/>
			<Label htmlFor="speed">
				話速 {"("}
				{speed}
				{")"}
			</Label>
			<Slider
				defaultValue={[1.0]}
				max={10.0}
				min={0.25}
				step={0.05}
				name="speed"
				className="mb-6 mt-4"
				onValueChange={(value) => setSpeed(value[0])}
			/>
			<Label htmlFor="sdpratio">
				SDP {"("}
				{sdpRatio}
				{")"}
			</Label>
			<Slider
				defaultValue={[0.4]}
				max={1.0}
				min={0.0}
				step={0.05}
				name="sdpratio"
				className="mb-6 mt-4"
				onValueChange={(value) => setSdpRatio(value[0])}
			/>
			{audio && <audio controls src={audio} autoPlay></audio>}
			<div className="flex mt-2 gap-2">
				<Button
					onClick={async () => {
						setReloading(true);
						try {
							await reloadModels();
							setModels(await getModels());
						} catch (e) {
							setError(String(e));
						}
						setModel(null);
						setReloading(false);
					}}
				>
					再読み込み
				</Button>
				<Button
					disabled={inSynthesize}
					onClick={async () => {
						if (audio) {
							URL.revokeObjectURL(audio);
						}
						setInSynthesize(true);
						if (!text.length || !model || reloading) {
							setInSynthesize(false);
							return;
						}
						try {
							const res = new Blob(
								[
									new Uint8Array(
										await synthesize(
											model,
											text,
											sdpRatio,
											1 / speed,
										),
									),
								],
								{ type: "audio/wav" },
							);
							setAudio(URL.createObjectURL(res));
						} catch (e) {
							setError(String(e));
						}
						setInSynthesize(false);
					}}
				>
					合成
				</Button>
				<Button onClick={() => open()}>モデルファイルを開く</Button>
			</div>
		</div>
	);
}

export default App;
