import { invoke } from "@tauri-apps/api/tauri";

export async function reloadModels() {
	return invoke("reload_models");
}

export async function getModels(): Promise<string[]> {
	return invoke("models");
}

export async function synthesize(
	ident: string,
	text: string,
	sdpRatio: number,
	lengthScale: number,
): Promise<number[]> {
	return invoke("synthesize", { ident, text, sdpRatio, lengthScale });
}

export async function open(): Promise<string> {
	return invoke("open");
}
