import { useState, useEffect } from 'react'
import axios from 'axios'
import {
  Users, Calendar, CheckSquare, Package, Home,
  BarChart3, Info, RefreshCw, ChevronRight,
  Smile, AlertTriangle, Tag
} from 'lucide-react'
import './App.css'

const api = axios.create({
  baseURL: `${import.meta.env.VITE_API_URL || ''}/api/v1`,
})

type Tab = 'dashboard' | 'people' | 'events' | 'tasks' | 'things' | 'spaces'

interface DashboardData {
  family_name: string
  stats: { people_count: number; upcoming_events: number; pending_tasks: number; things_count: number; spaces_count: number }
  happiness_score: number
  recent_activities: { type: string; content: string; time: string }[]
}

function App() {
  const [tab, setTab] = useState<Tab>('dashboard')
  const [data, setData] = useState<Record<string, unknown[]>>({})
  const [dashboard, setDashboard] = useState<DashboardData | null>(null)
  const [loading, setLoading] = useState(false)
  const [notice, setNotice] = useState(true)

  useEffect(() => {
    loadDashboard()
  }, [])

  useEffect(() => {
    if (tab !== 'dashboard') loadTab(tab)
  }, [tab])

  const loadDashboard = async () => {
    setLoading(true)
    try {
      const res = await api.get('/dashboard')
      setDashboard(res.data)
    } finally {
      setLoading(false)
    }
  }

  const loadTab = async (t: Tab) => {
    if (data[t]) return
    setLoading(true)
    try {
      const res = await api.get(`/${t}`)
      setData(prev => ({ ...prev, [t]: Array.isArray(res.data) ? res.data : [] }))
    } finally {
      setLoading(false)
    }
  }

  const tabs: { id: Tab; label: string; icon: React.ReactNode }[] = [
    { id: 'dashboard', label: '概览', icon: <BarChart3 size={18} /> },
    { id: 'people', label: '人员', icon: <Users size={18} /> },
    { id: 'events', label: '日程', icon: <Calendar size={18} /> },
    { id: 'tasks', label: '事务', icon: <CheckSquare size={18} /> },
    { id: 'things', label: '物品', icon: <Package size={18} /> },
    { id: 'spaces', label: '空间', icon: <Home size={18} /> },
  ]

  return (
    <div className="app">
      {notice && (
        <div className="demo-notice">
          <Info size={16} />
          <span>演示环境 · 数据每日凌晨 3:00 自动重置 · 仅供体验，不保存真实数据</span>
          <button onClick={() => setNotice(false)}>✕</button>
        </div>
      )}

      <header className="header">
        <div className="header-brand">
          <span className="brand-logo">⚡</span>
          <span className="brand-name">HamR 家庭管家</span>
          <span className="demo-badge">演示</span>
        </div>
        <div className="header-family">
          {dashboard?.family_name || '示例家庭'}
        </div>
      </header>

      <nav className="tab-nav">
        {tabs.map(t => (
          <button
            key={t.id}
            className={`tab-btn ${tab === t.id ? 'active' : ''}`}
            onClick={() => setTab(t.id)}
          >
            {t.icon}
            <span>{t.label}</span>
          </button>
        ))}
      </nav>

      <main className="main-content">
        {loading && <div className="loading"><RefreshCw size={20} className="spin" /> 加载中...</div>}

        {!loading && tab === 'dashboard' && dashboard && (
          <DashboardView data={dashboard} />
        )}
        {!loading && tab === 'people' && (
          <ListSection title="家庭成员" items={data['people'] || []} renderItem={PersonCard} />
        )}
        {!loading && tab === 'events' && (
          <ListSection title="日程安排" items={data['events'] || []} renderItem={EventCard} />
        )}
        {!loading && tab === 'tasks' && (
          <ListSection title="事务清单" items={data['tasks'] || []} renderItem={TaskCard} />
        )}
        {!loading && tab === 'things' && (
          <ListSection title="家庭物品" items={data['things'] || []} renderItem={ThingCard} />
        )}
        {!loading && tab === 'spaces' && (
          <ListSection title="生活空间" items={data['spaces'] || []} renderItem={SpaceCard} />
        )}
      </main>

      <footer className="footer">
        <a href="https://hamr.store" target="_blank" rel="noreferrer">体验完整版 →</a>
        <span>|</span>
        <a href="https://deploy.hamr.top" target="_blank" rel="noreferrer">私有部署指南</a>
      </footer>
    </div>
  )
}

function DashboardView({ data }: { data: DashboardData }) {
  const score = data.happiness_score
  const scoreColor = score >= 80 ? '#22c55e' : score >= 60 ? '#eab308' : '#ef4444'
  return (
    <div className="dashboard">
      <div className="stats-grid">
        <div className="stat-card"><Users size={24} /><span className="stat-num">{data.stats.people_count}</span><span className="stat-label">家庭成员</span></div>
        <div className="stat-card"><Calendar size={24} /><span className="stat-num">{data.stats.upcoming_events}</span><span className="stat-label">近期日程</span></div>
        <div className="stat-card"><CheckSquare size={24} /><span className="stat-num">{data.stats.pending_tasks}</span><span className="stat-label">待办事项</span></div>
        <div className="stat-card"><Package size={24} /><span className="stat-num">{data.stats.things_count}</span><span className="stat-label">家庭物品</span></div>
      </div>
      <div className="happiness-card">
        <Smile size={28} />
        <div className="happiness-content">
          <span className="happiness-label">家庭幸福指数</span>
          <span className="happiness-score" style={{ color: scoreColor }}>{score}</span>
          <span className="happiness-desc">五维综合评分（满分 100）</span>
        </div>
      </div>
      <div className="recent-section">
        <h3>最近动态</h3>
        {data.recent_activities.map((act, i) => (
          <div key={i} className="activity-item">
            <ChevronRight size={14} />
            <span className="activity-content">{act.content}</span>
            <span className="activity-time">{act.time}</span>
          </div>
        ))}
      </div>
    </div>
  )
}

function ListSection({ title, items, renderItem }: {
  title: string
  items: Record<string, unknown>[]
  renderItem: (item: Record<string, unknown>) => React.ReactNode
}) {
  return (
    <div className="list-section">
      <h2>{title} <span className="count-badge">{items.length}</span></h2>
      <div className="item-grid">
        {items.map((item, i) => (
          <div key={i} className="item-card">{renderItem(item)}</div>
        ))}
      </div>
    </div>
  )
}

const PersonCard = (p: Record<string, unknown>) => (
  <>
    <div className="card-header"><Users size={18} /><strong>{String(p.name)}</strong><Tag size={14} className="tag">{String(p.role)}</Tag></div>
    <div className="card-body">
      <div>生日：{String(p.birthday)}</div>
      <div>爱好：{(p.hobbies as string[]).join('、')}</div>
      {p.notes && <div className="card-note">{String(p.notes)}</div>}
    </div>
  </>
)

const EventCard = (e: Record<string, unknown>) => (
  <>
    <div className="card-header"><Calendar size={18} /><strong>{String(e.title)}</strong></div>
    <div className="card-body">
      <div>时间：{String(e.start_time).replace('T', ' ')}</div>
      <div>地点：{String(e.location)}</div>
      <div>参与：{(e.participants as string[]).join('、')}</div>
      {e.recurring && <div className="card-note">🔄 {String(e.recurring)}</div>}
    </div>
  </>
)

const TaskCard = (t: Record<string, unknown>) => {
  const statusColor: Record<string, string> = { '已完成': '#22c55e', '进行中': '#3b82f6', '待办': '#f59e0b' }
  return (
    <>
      <div className="card-header">
        <CheckSquare size={18} />
        <strong>{String(t.title)}</strong>
        <span className="status-badge" style={{ background: statusColor[String(t.status)] || '#6b7280' }}>{String(t.status)}</span>
      </div>
      <div className="card-body">
        <div>优先级：{String(t.priority)} | 负责人：{String(t.assignee)}</div>
        <div>截止：{String(t.due_date)}</div>
      </div>
    </>
  )
}

const ThingCard = (t: Record<string, unknown>) => (
  <>
    <div className="card-header"><Package size={18} /><strong>{String(t.name)}</strong></div>
    <div className="card-body">
      <div>类别：{String(t.category)} | 位置：{String(t.location)}</div>
      <div>购入：{String(t.purchase_date)}</div>
      {t.warranty_until && <div>保修至：{String(t.warranty_until)}</div>}
      {t.notes && <div className="card-note">{String(t.notes)}</div>}
    </div>
  </>
)

const SpaceCard = (s: Record<string, unknown>) => (
  <>
    <div className="card-header"><Home size={18} /><strong>{String(s.name)}</strong><Tag size={14}>{String(s.type)}</Tag></div>
    <div className="card-body">
      <div>面积：{String(s.area)}</div>
      <div className="card-note">{String(s.description)}</div>
    </div>
  </>
)

export default App
