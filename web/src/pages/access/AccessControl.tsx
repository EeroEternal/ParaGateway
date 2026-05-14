import { useState, useEffect } from 'react'
import { Plus, X, Key, LayoutGrid } from 'lucide-react'

interface Project {
  id: string
  name: string
  org_id: string
}

interface ApiKey {
  id: string
  name: string
  key_prefix: string
  created_at: string
}

export default function AccessControl() {
  const [projects, setProjects] = useState<Project[]>([])
  const [keys, setKeys] = useState<ApiKey[]>([])
  const [isKeyModalOpen, setIsKeyModalOpen] = useState(false)
  const [newKey, setNewKey] = useState<{name: string, project_id: string, key?: string} | null>(null)

  useEffect(() => {
    fetchData()
  }, [])

  const fetchData = async () => {
    const [pRes, kRes] = await Promise.all([
      fetch('/api/admin/projects'),
      fetch('/api/admin/api-keys')
    ])
    const pData = await pRes.json()
    const kData = await kRes.json()
    if (pData.success) setProjects(pData.data)
    if (kData.success) setKeys(kData.data)
  }

  const handleCreateKey = async (e: React.FormEvent) => {
    e.preventDefault()
    const formData = new FormData(e.target as HTMLFormElement)
    const res = await fetch('/api/admin/api-keys', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        project_id: formData.get('project_id'),
        name: formData.get('name'),
      })
    })
    const data = await res.json()
    if (data.success) {
      setNewKey(data.data)
      fetchData()
    }
  }

  return (
    <div className="max-w-6xl mx-auto space-y-8">
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-2xl font-bold text-zinc-900">Access Control</h2>
          <p className="text-sm text-zinc-500 mt-1">Manage Projects and API Keys.</p>
        </div>
        <button 
          onClick={() => setIsKeyModalOpen(true)}
          className="flex items-center gap-2 px-4 py-2 bg-black text-white text-sm font-medium rounded-md hover:bg-zinc-800"
        >
          <Key className="w-4 h-4" /> Issue API Key
        </button>
      </div>

      {/* API Keys Table */}
      <div className="bg-white border border-zinc-200 rounded-lg shadow-sm">
        <div className="px-6 py-4 border-b border-zinc-200 font-bold">API Keys</div>
        <table className="w-full text-left text-sm">
          <thead className="bg-zinc-50 text-zinc-600 border-b border-zinc-200">
            <tr>
              <th className="px-6 py-3">Name</th>
              <th className="px-6 py-3">Prefix</th>
              <th className="px-6 py-3">Created</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-zinc-200">
            {keys.map(key => (
              <tr key={key.id}>
                <td className="px-6 py-3">{key.name}</td>
                <td className="px-6 py-3 font-mono">{key.key_prefix}****</td>
                <td className="px-6 py-3">{new Date(key.created_at).toLocaleDateString()}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      {/* Key Modal */}
      {isKeyModalOpen && (
        <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/50">
          <div className="bg-white rounded-lg w-full max-w-sm">
            <div className="px-6 py-4 border-b flex justify-between">
              <h3 className="font-bold">Issue New API Key</h3>
              <button onClick={() => { setIsKeyModalOpen(false); setNewKey(null); }}><X className="w-4 h-4" /></button>
            </div>
            
            {newKey ? (
              <div className="p-6 space-y-4">
                <div className="p-4 bg-zinc-100 rounded font-mono text-xs break-all">
                  {newKey.key}
                </div>
                <p className="text-sm text-zinc-500">Copy this key now. It will not be shown again.</p>
                <button className="w-full bg-black text-white py-2 rounded" onClick={() => setIsKeyModalOpen(false)}>Done</button>
              </div>
            ) : (
              <form onSubmit={handleCreateKey} className="p-6 space-y-4">
                <div>
                  <label className="block text-sm mb-1">Project</label>
                  <select name="project_id" className="w-full border rounded p-2" required>
                    {projects.map(p => <option key={p.id} value={p.id}>{p.name}</option>)}
                  </select>
                </div>
                <div>
                  <label className="block text-sm mb-1">Key Name</label>
                  <input name="name" className="w-full border rounded p-2" required />
                </div>
                <button className="w-full bg-black text-white py-2 rounded">Generate</button>
              </form>
            )}
          </div>
        </div>
      )}
    </div>
  )
}
