import { useEffect, useState } from "react";
import { Link } from "react-router-dom";
import { Activity, Bell, CheckCircle2, PauseCircle, Plus, XCircle } from "lucide-react";
import { monitors as monitorsApi } from "@/lib/api";
import { notifications as notificationsApi } from "@/lib/api";
import type { Monitor, NotificationChannel } from "@/types";
import { Button } from "@/components/Button";

function StatCard({
  label,
  value,
  icon: Icon,
  color,
}: {
  label: string;
  value: number;
  icon: React.ElementType;
  color: string;
}) {
  return (
    <div className="rounded-xl border border-slate-800 bg-slate-900 p-5">
      <div className="flex items-center justify-between">
        <p className="text-sm text-slate-400">{label}</p>
        <div className={`flex h-9 w-9 items-center justify-center rounded-lg ${color}`}>
          <Icon className="h-4 w-4" />
        </div>
      </div>
      <p className="mt-3 text-3xl font-bold text-slate-100">{value}</p>
    </div>
  );
}

export default function OverviewPage() {
  const [monitorList, setMonitorList] = useState<Monitor[]>([]);
  const [channelList, setChannelList] = useState<NotificationChannel[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    Promise.all([monitorsApi.list(), notificationsApi.list()])
      .then(([m, c]) => {
        setMonitorList(m);
        setChannelList(c);
      })
      .finally(() => setLoading(false));
  }, []);

  const active = monitorList.filter((m) => !m.is_paused).length;
  const paused = monitorList.filter((m) => m.is_paused).length;
  const verified = channelList.filter((c) => c.verified).length;

  return (
    <div className="space-y-8">
      <div>
        <h1 className="text-2xl font-bold text-slate-100">Overview</h1>
        <p className="mt-1 text-sm text-slate-400">
          A summary of your monitoring activity.
        </p>
      </div>

      {loading ? (
        <div className="flex items-center justify-center py-20">
          <div className="h-8 w-8 animate-spin rounded-full border-2 border-indigo-500 border-t-transparent" />
        </div>
      ) : (
        <>
          <div className="grid grid-cols-2 gap-4 sm:grid-cols-4">
            <StatCard
              label="Total monitors"
              value={monitorList.length}
              icon={Activity}
              color="bg-indigo-500/20 text-indigo-400"
            />
            <StatCard
              label="Active"
              value={active}
              icon={CheckCircle2}
              color="bg-emerald-500/20 text-emerald-400"
            />
            <StatCard
              label="Paused"
              value={paused}
              icon={PauseCircle}
              color="bg-amber-500/20 text-amber-400"
            />
            <StatCard
              label="Channels"
              value={channelList.length}
              icon={Bell}
              color="bg-violet-500/20 text-violet-400"
            />
          </div>

          {/* Recent monitors */}
          <div className="rounded-xl border border-slate-800 bg-slate-900">
            <div className="flex items-center justify-between border-b border-slate-800 px-5 py-4">
              <h2 className="font-semibold text-slate-100">Recent monitors</h2>
              <Link to="/dashboard/monitors">
                <Button variant="ghost" size="sm">
                  View all
                </Button>
              </Link>
            </div>
            {monitorList.length === 0 ? (
              <div className="flex flex-col items-center gap-3 py-12 text-center">
                <XCircle className="h-10 w-10 text-slate-700" />
                <p className="text-sm text-slate-400">No monitors yet.</p>
                <Link to="/dashboard/monitors">
                  <Button size="sm">
                    <Plus className="h-4 w-4" />
                    Add your first monitor
                  </Button>
                </Link>
              </div>
            ) : (
              <ul className="divide-y divide-slate-800">
                {monitorList.slice(0, 5).map((m) => (
                  <li key={m.id} className="flex items-center gap-4 px-5 py-3.5">
                    <span
                      className={`h-2.5 w-2.5 shrink-0 rounded-full ${
                        m.is_paused ? "bg-amber-400" : "bg-emerald-400"
                      }`}
                    />
                    <div className="min-w-0 flex-1">
                      <p className="truncate text-sm font-medium text-slate-200">
                        {m.name ?? m.url}
                      </p>
                      {m.name && (
                        <p className="truncate text-xs text-slate-500">{m.url}</p>
                      )}
                    </div>
                    <span
                      className={`shrink-0 rounded-full px-2 py-0.5 text-xs font-medium ${
                        m.is_paused
                          ? "bg-amber-500/10 text-amber-400"
                          : "bg-emerald-500/10 text-emerald-400"
                      }`}
                    >
                      {m.is_paused ? "Paused" : "Active"}
                    </span>
                  </li>
                ))}
              </ul>
            )}
          </div>

          {/* Notification channels summary */}
          <div className="rounded-xl border border-slate-800 bg-slate-900">
            <div className="flex items-center justify-between border-b border-slate-800 px-5 py-4">
              <h2 className="font-semibold text-slate-100">Notification channels</h2>
              <Link to="/dashboard/notifications">
                <Button variant="ghost" size="sm">
                  Manage
                </Button>
              </Link>
            </div>
            {channelList.length === 0 ? (
              <div className="flex flex-col items-center gap-3 py-12 text-center">
                <Bell className="h-10 w-10 text-slate-700" />
                <p className="text-sm text-slate-400">No notification channels configured.</p>
                <Link to="/dashboard/notifications">
                  <Button size="sm">
                    <Plus className="h-4 w-4" />
                    Add channel
                  </Button>
                </Link>
              </div>
            ) : (
              <ul className="divide-y divide-slate-800">
                {channelList.map((c) => (
                  <li key={c.id} className="flex items-center gap-4 px-5 py-3.5">
                    <div className="min-w-0 flex-1">
                      <p className="truncate text-sm font-medium text-slate-200">{c.value}</p>
                      <p className="text-xs text-slate-500">{c.channel_type}</p>
                    </div>
                    <div className="flex items-center gap-2">
                      {verified > 0 && (
                        <span className="shrink-0 rounded-full bg-emerald-500/10 px-2 py-0.5 text-xs font-medium text-emerald-400">
                          {c.verified ? "Verified" : "Unverified"}
                        </span>
                      )}
                      {!c.verified && (
                        <span className="shrink-0 rounded-full bg-amber-500/10 px-2 py-0.5 text-xs font-medium text-amber-400">
                          Unverified
                        </span>
                      )}
                    </div>
                  </li>
                ))}
              </ul>
            )}
          </div>
        </>
      )}
    </div>
  );
}
