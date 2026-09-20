import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import ThemePicker from "./components/ThemePicker.tsx";
import {
	getCurrentWebviewWindow,
	WebviewWindow,
} from "@tauri-apps/api/webviewWindow";

function App() {
	const [activeWorkspace, setActiveWorkspace] = useState(1);
	const [foregroundApp, setForegroundApp] = useState("");

	const currentWindow = getCurrentWebviewWindow();
	const isThemePicker = currentWindow.label === "theme-picker";

	useEffect(() => {
		const unlistenAppearance = listen<{
			borderWidth: number;
			borderRadius: number;
			opacity: number;
		}>("appearance-change", (event) => {
			document.documentElement.style.setProperty(
				"--border-width",
				`${event.payload.borderWidth}px`,
			);

			document.documentElement.style.setProperty(
				"--border-radius",
				`${event.payload.borderRadius}px`,
			);

			document.documentElement.style.setProperty(
				"--bg-opacity",
				`${event.payload.opacity}`,
			);
		});

		if (isThemePicker) return;

		const unlistenForegroundApp = listen<string>(
			"foreground-app",
			(event) => {
				setForegroundApp(event.payload);
			},
		);

		const unlistenWorkspace = listen<number>(
			"workspace-hotkey",
			(event) => {
				setActiveWorkspace(event.payload);
			},
		);

		const unlistenTheme = listen<string>("theme-change", (event) => {
			console.log("THEME RECEIVED: {}", event.payload);
			document.documentElement.dataset.theme =
				event.payload === "void" ? "" : event.payload;
		});

		return () => {
			unlistenWorkspace.then((fn) => fn());
			unlistenTheme.then((fn) => fn());
			unlistenAppearance.then((fn) => fn());
			unlistenForegroundApp.then((fn) => fn());
		};
	}, [isThemePicker]);

	if (isThemePicker) {
		return <ThemePicker />;
	}

	return (
		<main className="app">
			<div className="bar">
				<div className="bar-left">
					<div className="brand">
						<span
							className="rift-icon"
							onClick={async () => {
								console.log("CHROMA CLICKED");

								const picker =
									await WebviewWindow.getByLabel(
										"theme-picker",
									);

								console.log("THEME PICKER:", picker);

								if (!picker) {
									alert("theme-picker was not found");
									return;
								}

								console.log("SHOWING THEME PICKER");

								await picker.show();
								console.log("THEME PICKER SHOWN");

								await picker.setFocus();
								console.log("THEME PICKER FOCUSED");
							}}
						>
							✦
						</span>
						<span>RIFTWALKER</span>
					</div>

					<nav className="workspaces">
						{[1, 2, 3, 4].map((workspace) => (
							<button
								key={workspace}
								className={
									activeWorkspace === workspace
										? "active"
										: ""
								}
							>
								{workspace}
							</button>
						))}
					</nav>
				</div>

				<div className="bar-center">
					<div className="bar-center">
						<span className="foreground-app">
							{foregroundApp || "Desktop"}
						</span>
					</div>
				</div>

				<div className="bar-right">
					<span>12% CPU</span>
					<span>7.4 GB RAM</span>
					<span>21:37</span>
				</div>
			</div>
		</main>
	);
}

export default App;
