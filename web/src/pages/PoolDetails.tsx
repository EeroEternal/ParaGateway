import { useState, useEffect } from 'react'
import { useParams, Link } from 'react-router-dom'
import { Plus, X, ArrowLeft } from 'lucide-react'

interface Endpoint {
  id: string
  name: string
  upstream_model_id: string
}

interface Pool {
  id: string
  name: string
  strategy: string
}

export default function PoolDetails() {
  const { id } = useParams<{ id: string }>()
  const [pool, setPool] = useState<Pool | null>(null)
  const [endpoints, setEndpoints] = useState<Endpoint[]>([])
  const [isBindModalOpen, setIsBindModalOpen] = useState(false)
  
  // Form state
  const [selectedEndpoint, setSelectedEndpoint] = useState('')
  const [priority, setPriority] = useState('1')
  const [weight, setWeight] = useState('1')

  useEffect(() => {
    fetchPool()
    fetchEndpoints()
  }, [id])

  const fetchPool = async () => {
    const res = await fetch('/api/admin/pools')
    const data = await res.json()
    if (data.success) {
      setPool(data.data.find((p: Pool) => p.id === id))
    }
  }

  const fetchEndpoints = async () => {
    const res = await fetch('/api/admin/endpoints')
    const data = await res.json()
    if (data.success) {
      setEndpoints(data.data)
    }
  }

  const handleBind = async (e: React.FormEvent) => {
    e.preventDefault()
    const res = await fetch('/api/admin/pools/bind', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        pool_id: id,
        endpoint_id: selectedEndpoint,
        priority: parseInt(priority),
        weight: parseInt(weight)
      })
    })
    const data = await res.json()
    if (data.success) {
      setIsBindModalOpen(false)
      alert('Endpoint bound successfully!')
    }
  }

  if (!pool) return <div className="p-8">Loading...</div>

  return (
    <div className="max-w-4xl mx-auto space-y-6">
      <Link to="/pools" className="flex items-center gap-2 text-sm text-zinc-500 hover:text-black">
        <ArrowLeft className="w-4 h-4" /> Back to Pools
      </Link>
      
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-2xl font-bold text-zinc-900">{pool.name}</h2>
          <p className="text-sm text-zinc-500 mt-1">Strategy: {pool.strategy}</p>
        </div>
        <button 
          onClick={() => setIsBindModalOpen(true)}
          className="flex items-center gap-2 px-4 py-2 bg-black text-white text-sm font-medium rounded-md hover:bg-zinc-800"
        >
          <Plus className="w-4 h-4" /> Bind Endpoint
        </button>
      </div>

      {isBindModalOpen && (
        <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/50">
          <div className="bg-white rounded-lg shadow-lg w-full max-w-sm">
            <div className="px-6 py-4 border-b border-zinc-200 flex justify-between">
              <h3 className="font-bold">Bind Endpoint</h3>
              <button onClick={() => setIsBindModalOpen(false)}><X className="w-4 h-4" /></button>
            </div>
            <form onSubmit={handleBind} className="p-6 space-y-4">
              <div>
                <label className="block text-sm font-medium mb-1">Select Endpoint</label>
                <select className="w-full border rounded p-2" onChange={e => setSelectedEndpoint(e.target.value)}>
                  <option value="">Select an endpoint...</option>
                  {endpoints.map(e => <option key={e.id} value={e.id}>{e.name} ({e.upstream_model_id})</option>)}
                </select>
              </div>
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="block text-sm font-medium mb-1">Priority</label>
                  <input type="number" className="w-full border rounded p-2" value={priority} onChange={e => setPriority(e.target.value)} />
                </div>
                <div>
                  <label className="block text-sm font-medium mb-1">Weight</label>
                  <input type="number" className="w-full border rounded p-2" value={weight} onChange={e => setWeight(e.target.value)} />
                </div>
              </div>
              <button className="w-full bg-black text-white py-2 rounded font-medium">Bind</button>
            </form>
          </div>
        </div>
      )}
    </div>
  )
}
