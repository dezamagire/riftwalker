function App() {
  return (
    <main className="app">
      <div className="bar">
        <div className="bar-left">
          <div className="brand">
            <span className="rift-icon">✦</span>
            <span>RIFTWALKER</span>
          </div>

          <nav className="workspaces">
            <button>1</button>
            <button className="active">2</button>
            <button>3</button>
            <button>4</button>
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
  )
}

export default App