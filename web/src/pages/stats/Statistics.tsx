import { useState, useEffect } from 'react'

interface Stats {
  total_tokens: number
  avg_latency: number
  request_count: number
}

export default function Statistics() {
  const [stats, setStats] = useState<Stats | null>(null)

  useEffect(() => {
    fetch('/api/admin/stats')
      .then(res => res.json())
      .then(data => {
        if (data.success) setStats(data.data)
      })
  }, [])

  return (
    <div className="max-w-6xl mx-auto space-y-8">
      <div>
        <h2 className="text-2xl font-bold text-zinc-900">System Statistics</h2>
        <p className="text-sm text-zinc-500 mt-1">Real-time gateway performance metrics.</p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        <div className="bg-white p-6 border border-zinc-200 rounded-lg shadow-sm">
          <div className="text-sm text-zinc-500 font-medium">Total Tokens</div>
          <div className="text-3xl font-mono mt-2 font-bold">{stats?.total_tokens.toLocaleString() ?? '-'}</div>
        </div>
        <div className="bg-white p-6 border border-zinc-200 rounded-lg shadow-sm">
          <div className="text-sm text-zinc-500 font-medium">Avg Latency (ms)</div>
          <div className="text-3xl font-mono mt-2 font-bold">{stats?.avg_latency.toFixed(0) ?? '-'}</div>
        </div>
        <div className="bg-white p-6 border border-zinc-200 rounded-lg shadow-sm">
          <div className="text-sm text-zinc-500 font-medium">Total Requests</div>
          <div className="text-3xl font-mono mt-2 font-bold">{stats?.request_count.toLocaleString() ?? '-'}</div>
        </div>
      </div>
    </div>
  )
}
