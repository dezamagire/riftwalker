import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";

function App() {
  const [activeWorkspace, setActiveWorkspace] = useState(1);

  useEffect(() => {
    const unlisten = listen<number>("workspace-hotkey", (event) => {
      setActiveWorkspace(event.payload);
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  return (
    <main className="app">
      <div className="bar">
        <div className="bar-left">
          <div className="brand">
            <span className="rift-icon">✦</span>
            <span>RIFTWALKER</span>
          </div>

          <nav className="workspaces">
            {[1, 2, 3, 4].map((workspace) => (
              <button
                key={workspace}
                className={activeWorkspace === workspace ? "active" : ""}
              >
                {workspace}
              </button>
            ))}
          </nav>
        </div>

        <div className="bar-center">
          <span>alacritty</span>
          <span>Firefox</span>
          <span>Discord</span>
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