import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { listen } from "@tauri-apps/api/event";

type DownloadProgress = {
	url: string;
	progress: number;
};

function App() {
	const [progress, setProgress] = useState<number>(0);

	useEffect(() => {
		let unListen = listen<DownloadProgress>("download-progress", (event) => {
			console.log(
				`downloading ${event.payload.progress} bytes from ${event.payload.url}`,
			);
			setProgress(event.payload.progress);
		});

		return () => {
			unListen.then((unlisten) => unlisten());
		};
	}, [progress]);

	async function startDownload() {
		try {
			void invoke<string>("start_download");
		} catch (error) {
			console.error("Error starting the download", error);
		}
	}

	return (
		<main className="container">
			<h1 className="text-blue-500 text-xl">Download Progress</h1>
			<p>{progress}</p>
			<button onClick={startDownload}>Start Download</button>
		</main>
	);
}

export default App;
