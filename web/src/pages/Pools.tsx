import { useState, useEffect } from 'react'
import { Plus, X } from 'lucide-react'

interface ModelPool {
  id: string
  name: string
  strategy: string
  enabled: boolean
  created_at: string
}

export default function Pools() {
  const [pools, setPools] = useState<ModelPool[]>([])
  const [loading, setLoading] = useState(true)
  const [isModalOpen, setIsModalOpen] = useState(false)
  const [formData, setFormData] = useState({
    name: '',
    strategy: 'round_robin',
  })
  const [submitting, setSubmitting] = useState(false)

  useEffect(() => {
    fetchPools()
  }, [])

  const fetchPools = async () => {
    try {
      const res = await fetch('/api/admin/pools')
      const data = await res.json()
      if (data.success) {
        setPools(data.data)
      }
    } catch (error) {
      console.error('Failed to fetch pools:', error)
    } finally {
      setLoading(false)
    }
  }

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setSubmitting(true)
    try {
      const res = await fetch('/api/admin/pools', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(formData)
      })
      const data = await res.json()
      if (data.success) {
        setIsModalOpen(false)
        setFormData({ name: '', strategy: 'round_robin' })
        fetchPools()
      } else {
        alert(data.message || 'Failed to create pool')
      }
    } catch (error) {
      console.error('Submit error:', error)
      alert('Network error')
    } finally {
      setSubmitting(false)
    }
  }

  return (
    <div className="max-w-6xl mx-auto space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-2xl font-bold text-zinc-900">Model Pools</h2>
          <p className="text-sm text-zinc-500 mt-1">Group endpoints and define routing strategies.</p>
        </div>
        <button 
          onClick={() => setIsModalOpen(true)}
          className="flex items-center gap-2 px-4 py-2 bg-black text-white text-sm font-medium rounded-md hover:bg-zinc-800 transition-colors focus:outline-none focus:ring-2 focus:ring-black focus:ring-offset-2"
        >
          <Plus className="w-4 h-4" />
          Create Pool
        </button>
      </div>

      <div className="bg-white border border-zinc-200 rounded-lg shadow-sm overflow-hidden">
        <div className="overflow-x-auto">
          <table className="w-full text-left text-sm whitespace-nowrap">
            <thead className="bg-zinc-50 text-zinc-600 border-b border-zinc-200">
              <tr>
                <th className="px-6 py-3 font-medium">Name</th>
                <th className="px-6 py-3 font-medium">Strategy</th>
                <th className="px-6 py-3 font-medium">Status</th>
                <th className="px-6 py-3 font-medium text-right">Actions</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-zinc-200">
              {loading ? (
                <tr>
                  <td colSpan={4} className="px-6 py-8 text-center text-zinc-500">
                    Loading model pools...
                  </td>
                </tr>
              ) : pools.length === 0 ? (
                <tr>
                  <td colSpan={4} className="px-6 py-8 text-center text-zinc-500">
                    No model pools configured. Click "Create Pool" to start.
                  </td>
                </tr>
              ) : (
                pools.map((pool) => (
                  <tr key={pool.id} className="hover:bg-zinc-50/50 transition-colors">
                    <td className="px-6 py-4 font-medium text-zinc-900">{pool.name}</td>
                    <td className="px-6 py-4">
                      <span className="px-2.5 py-1 rounded-md bg-zinc-100 border border-zinc-200 text-xs font-mono">
                        {pool.strategy}
                      </span>
                    </td>
                    <td className="px-6 py-4">
                      <div className="flex items-center gap-2">
                        <span className={`w-2 h-2 rounded-full ${pool.enabled ? 'bg-emerald-500' : 'bg-zinc-300'}`} />
                        <span className="capitalize">{pool.enabled ? 'Active' : 'Disabled'}</span>
                      </div>
                    </td>
                    <td className="px-6 py-4 text-right">
                      <Link to={`/pools/${pool.id}`} className="text-zinc-500 hover:text-black font-medium">Manage Endpoints</Link>
                    </td>
                  </tr>
                ))
              )}
            </tbody>
          </table>
        </div>
      </div>

      {/* Create Pool Modal */}
      {isModalOpen && (
        <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/50">
          <div className="bg-white rounded-lg shadow-lg w-full max-w-md overflow-hidden">
            <div className="flex items-center justify-between px-6 py-4 border-b border-zinc-200">
              <h3 className="text-lg font-bold">Create Model Pool</h3>
              <button onClick={() => setIsModalOpen(false)} className="text-zinc-400 hover:text-black">
                <X className="w-5 h-5" />
              </button>
            </div>
            <form onSubmit={handleSubmit} className="p-6 space-y-4">
              <div>
                <label className="block text-sm font-medium text-zinc-700 mb-1">Pool Name</label>
                <input 
                  type="text" 
                  required
                  className="w-full bg-white border border-zinc-300 rounded-md px-3 py-2 text-sm focus:outline-none focus:border-black focus:ring-1 focus:ring-black"
                  placeholder="e.g. GPT-4 Enterprise"
                  value={formData.name}
                  onChange={e => setFormData({...formData, name: e.target.value})}
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-zinc-700 mb-1">Routing Strategy</label>
                <select 
                  className="w-full bg-white border border-zinc-300 rounded-md px-3 py-2 text-sm focus:outline-none focus:border-black focus:ring-1 focus:ring-black"
                  value={formData.strategy}
                  onChange={e => setFormData({...formData, strategy: e.target.value})}
                >
                  <option value="round_robin">Round Robin (Default)</option>
                  <option value="score_ordered">Score Ordered (Strict Priority)</option>
                  <option value="least_connections">Least Connections</option>
                  <option value="latency_based">Latency Based (EMA)</option>
                </select>
                <p className="mt-1.5 text-xs text-zinc-500">
                  Select how traffic will be distributed among the endpoints in this pool.
                </p>
              </div>
              
              <div className="pt-4 flex justify-end gap-3">
                <button 
                  type="button" 
                  onClick={() => setIsModalOpen(false)}
                  className="px-4 py-2 text-sm font-medium text-zinc-900 bg-transparent border border-zinc-300 rounded-md hover:bg-zinc-50"
                >
                  Cancel
                </button>
                <button 
                  type="submit" 
                  disabled={submitting}
                  className="px-4 py-2 text-sm font-medium text-white bg-black rounded-md hover:bg-zinc-800 disabled:opacity-50"
                >
                  {submitting ? 'Creating...' : 'Create Pool'}
                </button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  )
}
