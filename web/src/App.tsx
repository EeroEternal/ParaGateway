import { useState } from 'react'
import { BrowserRouter as Router, Routes, Route, Link, useLocation } from 'react-router-dom'
import { LayoutDashboard, Database, ShieldCheck, Activity, Layers } from 'lucide-react'
import Select from './components/Select'
import Providers from './pages/Providers'
import Pools from './pages/Pools'
import PoolDetails from './pages/PoolDetails'
import AccessControl from './pages/access/AccessControl'

const options = [
  { id: 1, name: 'Least Connections' },
  { id: 2, name: 'Latency Based' },
  { id: 3, name: 'Priority' },
]

function Dashboard() {
  const [selected, setSelected] = useState(options[0])
  return (
    <div className="max-w-4xl mx-auto space-y-8">
      <div className="bg-white border border-zinc-200 rounded-lg p-6 shadow-sm">
        <h3 className="text-lg font-bold mb-4">Initial Setup Preview</h3>
        <div className="w-64">
          <Select 
            label="Routing Strategy"
            options={options}
            selected={selected}
            onChange={setSelected}
          />
        </div>
      </div>
    </div>
  )
}

function Sidebar() {
  const location = useLocation();
  const isActive = (path: string) => location.pathname === path;

  const navItemClass = (path: string) => `flex items-center gap-3 px-3 py-2 text-sm font-medium rounded-md transition-colors ${
    isActive(path) ? 'bg-zinc-100 text-black' : 'text-zinc-600 hover:bg-zinc-50 hover:text-black'
  }`;

  return (
    <div className="w-64 border-r border-zinc-200 bg-white flex flex-col">
      <div className="p-6 border-b border-zinc-200">
        <h1 className="text-xl font-bold tracking-tight text-black flex items-center gap-2">
          <div className="w-8 h-8 bg-black rounded flex items-center justify-center">
            <div className="w-4 h-4 border-2 border-white rounded-sm" />
          </div>
          ParaGateway
        </h1>
      </div>
      <nav className="flex-1 p-4 space-y-1">
        <Link to="/" className={navItemClass('/')}>
          <LayoutDashboard className="w-4 h-4" />
          Dashboard
        </Link>
        <Link to="/providers" className={navItemClass('/providers')}>
          <Database className="w-4 h-4" />
          Providers
        </Link>
        <Link to="/pools" className={navItemClass('/pools')}>
          <Layers className="w-4 h-4" />
          Model Pools
        </Link>
        <Link to="/access" className={navItemClass('/access')}>
          <ShieldCheck className="w-4 h-4" />
          Access Control
        </Link>
        <Link to="/stats" className={navItemClass('/stats')}>
          <Activity className="w-4 h-4" />
          Statistics
        </Link>
      </nav>
    </div>
  );
}

function App() {
  return (
    <Router>
      <div className="flex h-screen bg-zinc-50 overflow-hidden font-sans">
        <Sidebar />

        {/* Main Content */}
        <div className="flex-1 flex flex-col overflow-auto">
          <header className="h-16 border-b border-zinc-200 bg-white flex items-center justify-between px-8">
            <h2 className="text-sm font-medium text-zinc-500">Admin Console</h2>
            <div className="flex items-center gap-4">
              <span className="w-2 h-2 rounded-full bg-emerald-500" />
              <span className="text-xs font-mono text-zinc-500">SYSTEM HEALTHY</span>
            </div>
          </header>

          <main className="p-8">
            <Routes>
              <Route path="/" element={<Dashboard />} />
              <Route path="/providers" element={<Providers />} />
              import Statistics from './pages/stats/Statistics'

              // ...
                            <Route path="/pools" element={<Pools />} />
                            <Route path="/pools/:id" element={<PoolDetails />} />
                            <Route path="/access" element={<AccessControl />} />
                            <Route path="/stats" element={<Statistics />} />
                            <Route path="*" element={<div className="text-zinc-500">Under Construction</div>} />
                          </Routes>
                        </main>

      </div>
    </Router>
  )
}

export default App
