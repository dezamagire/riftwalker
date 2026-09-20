import { useEffect, useState } from "react";
import { emitTo } from "@tauri-apps/api/event";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";

const themes = [
	{
		id: "void",
		name: "Void",
		description: "Riftwalker's native theme",
	},
	{
		id: "nord",
		name: "Nord",
		description: "Cold polar blues",
	},
	{
		id: "catppuccin",
		name: "Catppuccin",
		description: "Soft pastel dark",
	},
	{
		id: "rose-pine",
		name: "Rosé Pine",
		description: "Muted and dreamy",
	},
	{
		id: "gruvbox",
		name: "Gruvbox",
		description: "Warm and earthy",
	},
	{
		id: "jade",
		name: "Jade",
		description: "Deep emerald",
	},
	{
		id: "tokyo-night",
		name: "Tokyo Night",
		description: "Electric midnight",
	},
	{
		id: "everforest",
		name: "Everforest",
		description: "Muted forest",
	},
	{
		id: "kanagawa",
		name: "Kanagawa",
		description: "Ink and moonlight",
	},
	{
		id: "dracula",
		name: "Dracula",
		description: "Purple nocturne",
	},
];

function ThemePicker() {
	const [selected, setSelected] = useState("void");

	const [borderWidth, setBorderWidth] = useState(1);
	const [borderRadius, setBorderRadius] = useState(8);
	const [bgOpacity, setBgOpacity] = useState(100);

	const currentWindow = getCurrentWebviewWindow();

	useEffect(() => {
		document.documentElement.style.setProperty("--border-width", "1px");

		document.documentElement.style.setProperty("--border-radius", "8px");

		document.documentElement.style.setProperty("--bg-opacity", "100%");
	}, []);

	const updateAppearance = async (
		width: number,
		radius: number,
		opacity: number,
	) => {
		document.documentElement.style.setProperty(
			"--border-width",
			`${width}px`,
		);

		document.documentElement.style.setProperty(
			"--border-radius",
			`${radius}px`,
		);

		document.documentElement.style.setProperty(
			"--bg-opacity",
			`${opacity}%`,
		);

		await emitTo("main", "appearance-change", {
			borderWidth: width,
			borderRadius: radius,
			bgOpacity: opacity,
		});
	};

	const handleNumberChange = (
		name: "borderWidth" | "borderRadius" | "bgOpacity",
		value: string,
	) => {
		const parsed = Number(value);

		if (!Number.isFinite(parsed)) return;

		const limits = {
			borderWidth: { min: 0, max: 10 },
			borderRadius: { min: 0, max: 32 },
			bgOpacity: { min: 0, max: 100 },
		};

		const { min, max } = limits[name];

		const clamped = Math.min(max, Math.max(min, parsed));

		const next = {
			borderWidth,
			borderRadius,
			bgOpacity,
			[name]: clamped,
		};

		setBorderWidth(next.borderWidth);
		setBorderRadius(next.borderRadius);
		setBgOpacity(next.bgOpacity);

		updateAppearance(next.borderWidth, next.borderRadius, next.bgOpacity);
	};

	const selectTheme = async (theme: string) => {
		setSelected(theme);

		document.documentElement.dataset.theme = theme === "void" ? "" : theme;

		await emitTo("main", "theme-change", theme);
	};

	const closePicker = async () => {
		await currentWindow.hide();
	};

	return (
		<main className="theme-picker">
			<header className="theme-header">
				<div>
					<span className="theme-kicker">RIFTWALKER</span>
					<h1>CHROMA</h1>
				</div>

				<div className="theme-header-right">
					<button
						className="theme-close"
						onClick={closePicker}
						aria-label="Close theme picker"
					>
						×
					</button>

					<span className="theme-count">{themes.length} THEMES</span>
				</div>
			</header>

			<section className="theme-grid">
				{themes.map((theme) => (
					<button
						key={theme.id}
						className={`theme-card ${
							selected === theme.id ? "active" : ""
						}`}
						data-theme={theme.id}
						onClick={() => selectTheme(theme.id)}
					>
						<div className="theme-preview">
							<span className="preview-icon">✦</span>

							<span className="preview-workspaces">
								<i />
								<i />
								<i />
								<i />
							</span>

							<span className="preview-line" />
						</div>

						<div className="theme-info">
							<strong>{theme.name}</strong>
						</div>
					</button>
				))}
			</section>

			<section className="appearance-controls">
				<div className="appearance-control">
					<label htmlFor="border-width">Border width</label>

					<div className="number-input">
						<input
							id="border-width"
							name="borderWidth"
							type="number"
							min="0"
							max="10"
							step="1"
							value={borderWidth}
							onChange={(event) =>
								handleNumberChange(
									"borderWidth",
									event.target.value,
								)
							}
						/>

						<span>px</span>
					</div>
				</div>

				<div className="appearance-control">
					<label htmlFor="border-radius">Border radius</label>

					<div className="number-input">
						<input
							id="border-radius"
							name="borderRadius"
							type="number"
							min="0"
							max="32"
							step="1"
							value={borderRadius}
							onChange={(event) =>
								handleNumberChange(
									"borderRadius",
									event.target.value,
								)
							}
						/>

						<span>px</span>
					</div>
				</div>

				<div className="appearance-control">
					<label htmlFor="bg-opacity">Background opacity</label>

					<div className="number-input">
						<input
							id="bg-opacity"
							name="bgOpacity"
							type="number"
							min="0"
							max="100"
							step="1"
							value={bgOpacity}
							onChange={(event) =>
								handleNumberChange(
									"bgOpacity",
									event.target.value,
								)
							}
						/>

						<span>%</span>
					</div>
				</div>
			</section>
		</main>
	);
}

export default ThemePicker;
